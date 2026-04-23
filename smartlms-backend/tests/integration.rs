//! End-to-end integration test for the Phase 1 API surface.
//!
//! Exercises the full learner journey against a real Postgres:
//!   1. Register the first user (becomes admin) + a second user (learner)
//!   2. Admin creates a course + module + lesson, publishes the course
//!   3. Learner enrols, completes the lesson, progress goes to 100%
//!   4. Admin builds a question, assessment, links them, publishes
//!   5. Learner starts an attempt, submits, assessment auto-grades
//!
//! Gated on the `TEST_DATABASE_URL` env var so `cargo test` in environments
//! without Postgres just skips this test gracefully. Run locally with:
//!
//! ```text
//! docker run -d --rm --name smartlms-pg \
//!     -e POSTGRES_PASSWORD=smart -e POSTGRES_DB=smartlms_test \
//!     -p 5544:5432 postgres:16-alpine
//! TEST_DATABASE_URL=postgres://postgres:smart@localhost:5544/smartlms_test \
//!     cargo test --test integration
//! ```

use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

use smartlms_backend::{
    api,
    middleware::tenant::tenant_middleware,
    tenant::{InstitutionConfig, InstitutionCtx, PlanTier, QuotaLimits, RouterState},
};

async fn setup_schema(pool: &PgPool) -> anyhow::Result<()> {
    // Idempotent reset — drop & recreate public schema, then re-apply the
    // master + all per-institution migrations.
    sqlx::query("DROP SCHEMA IF EXISTS public CASCADE")
        .execute(pool)
        .await?;
    sqlx::query("CREATE SCHEMA public").execute(pool).await?;

    for path in [
        "migrations/001_master_schema.sql",
        "migrations_institution/001_users_and_rbac.sql",
        "migrations_institution/002_courses_and_enrollments.sql",
        "migrations_institution/003_assessments.sql",
    ] {
        let sql = std::fs::read_to_string(path)?;
        sqlx::raw_sql(&sql).execute(pool).await?;
    }
    Ok(())
}

async fn request_json(
    router: &axum::Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header(header::HOST, "localhost")
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(t) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    let body_bytes = body
        .map(|v| serde_json::to_vec(&v).unwrap())
        .unwrap_or_default();
    let req = builder.body(axum::body::Body::from(body_bytes)).unwrap();

    let response = router.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value: Value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, value)
}

#[tokio::test]
async fn end_to_end_learner_journey() {
    let db_url = match std::env::var("TEST_DATABASE_URL") {
        Ok(u) => u,
        Err(_) => {
            eprintln!("TEST_DATABASE_URL not set — skipping integration test");
            return;
        }
    };

    std::env::set_var(
        "JWT_SECRET",
        "integration-test-secret-key-do-not-use-in-prod",
    );

    let pool = PgPool::connect(&db_url).await.expect("connect postgres");
    setup_schema(&pool).await.expect("apply migrations");

    // Seed the master institutions row for slug "demo" pointing at the same
    // DB (single-database self-hosted mode). The router resolves Host:
    // localhost → slug "demo" by default.
    let tenant_id = uuid::Uuid::new_v4();
    sqlx::query(
        "INSERT INTO institutions (id, slug, name, plan_tier, is_active) \
         VALUES ($1, 'demo', 'Demo University', 'starter', true)",
    )
    .bind(tenant_id)
    .execute(&pool)
    .await
    .expect("seed institution");

    // Pre-warm the in-process cache so the router doesn't try to build a
    // second pool from a null database_url.
    let state = RouterState::new(pool.clone());
    let ctx = InstitutionCtx {
        id: tenant_id,
        slug: "demo".to_string(),
        db_pool: pool.clone(),
        config: InstitutionConfig::default(),
        plan: PlanTier::Starter,
        quotas: QuotaLimits::default(),
    };
    state.insert_cached("demo", ctx);

    let router = axum::Router::new()
        .nest("/api", api::create_api_router(state.clone()))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            tenant_middleware,
        ));

    // --- 1. Register admin ---------------------------------------------------
    let (status, admin_resp) = request_json(
        &router,
        "POST",
        "/api/auth/register",
        None,
        Some(json!({
            "email": "ada@demo.localhost",
            "password": "super-secret-pw",
            "first_name": "Ada",
            "last_name": "Lovelace",
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "register admin: {admin_resp}");
    let admin_token = admin_resp["access_token"].as_str().unwrap().to_string();
    let admin_roles = admin_resp["user"]["roles"].as_array().unwrap();
    assert!(
        admin_roles.iter().any(|r| r == "admin"),
        "first user should be admin, got {admin_roles:?}"
    );

    // --- 2. Register learner -------------------------------------------------
    let (status, learner_resp) = request_json(
        &router,
        "POST",
        "/api/auth/register",
        None,
        Some(json!({
            "email": "grace@demo.localhost",
            "password": "hopper-rocks",
            "first_name": "Grace",
            "last_name": "Hopper",
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let learner_token = learner_resp["access_token"].as_str().unwrap().to_string();
    let learner_roles = learner_resp["user"]["roles"].as_array().unwrap();
    assert!(
        learner_roles.iter().any(|r| r == "learner"),
        "second user should be learner by default, got {learner_roles:?}"
    );

    // --- 3. /me round-trip ---------------------------------------------------
    let (status, me_resp) =
        request_json(&router, "GET", "/api/users/me", Some(&admin_token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me_resp["email"], "ada@demo.localhost");

    // --- 4. Admin creates a course and publishes it --------------------------
    let (status, course) = request_json(
        &router,
        "POST",
        "/api/courses",
        Some(&admin_token),
        Some(json!({
            "slug": "intro-cs",
            "title": "Introduction to Computer Science",
            "description": "Foundational CS concepts.",
            "language": "en",
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create course: {course}");
    let course_id = course["id"].as_str().unwrap().to_string();

    let (status, module) = request_json(
        &router,
        "POST",
        &format!("/api/courses/{course_id}/modules"),
        Some(&admin_token),
        Some(json!({"title": "Welcome"})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create module: {module}");
    let module_id = module["id"].as_str().unwrap().to_string();

    let (status, lesson) = request_json(
        &router,
        "POST",
        &format!("/api/courses/{course_id}/modules/{module_id}/lessons"),
        Some(&admin_token),
        Some(json!({
            "title": "Getting Started",
            "kind": "text",
            "content": {"body": "# Welcome!"},
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create lesson: {lesson}");
    let lesson_id = lesson["id"].as_str().unwrap().to_string();

    let (status, _) = request_json(
        &router,
        "PATCH",
        &format!("/api/courses/{course_id}"),
        Some(&admin_token),
        Some(json!({"status": "published"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // --- 5. Learner enrols + completes the lesson ----------------------------
    let (status, enrollment) = request_json(
        &router,
        "POST",
        &format!("/api/courses/{course_id}/enroll"),
        Some(&learner_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "enrol: {enrollment}");

    let (status, progress) = request_json(
        &router,
        "POST",
        &format!("/api/courses/{course_id}/lessons/{lesson_id}/complete"),
        Some(&learner_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "complete lesson: {progress}");
    assert_eq!(progress["progress_pct"], 100);

    let (status, my_enrolments) = request_json(
        &router,
        "GET",
        "/api/enrollments",
        Some(&learner_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let items = my_enrolments.as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["status"], "completed");
    assert_eq!(items[0]["progress_pct"], 100);

    // --- 6. Admin builds an assessment ---------------------------------------
    let (status, question) = request_json(
        &router,
        "POST",
        "/api/questions",
        Some(&admin_token),
        Some(json!({
            "kind": "mcq",
            "stem": "Who is the first programmer?",
            "body": {"options": [
                {"id": "a", "text": "Ada Lovelace"},
                {"id": "b", "text": "Grace Hopper"}
            ]},
            "answer": {"option_id": "a"},
            "default_points": "2.0",
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "create question: {question}");
    let question_id = question["id"].as_str().unwrap().to_string();

    let (status, assessment) = request_json(
        &router,
        "POST",
        &format!("/api/courses/{course_id}/assessments"),
        Some(&admin_token),
        Some(json!({
            "title": "Quiz 1",
            "kind": "quiz",
            "passing_score_pct": "50",
        })),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "create assessment: {assessment}"
    );
    let assessment_id = assessment["id"].as_str().unwrap().to_string();

    let (status, _) = request_json(
        &router,
        "POST",
        &format!("/api/assessments/{assessment_id}/questions"),
        Some(&admin_token),
        Some(json!({"question_id": question_id, "position": 0})),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = request_json(
        &router,
        "PATCH",
        &format!("/api/assessments/{assessment_id}"),
        Some(&admin_token),
        Some(json!({"status": "published"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // --- 7. Learner attempt + grading ----------------------------------------
    let (status, attempt) = request_json(
        &router,
        "POST",
        &format!("/api/assessments/{assessment_id}/attempts"),
        Some(&learner_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "start attempt: {attempt}");
    let attempt_id = attempt["id"].as_str().unwrap().to_string();

    let (status, graded) = request_json(
        &router,
        "POST",
        &format!("/api/assessments/attempts/{attempt_id}/submit"),
        Some(&learner_token),
        Some(json!({
            "answers": [{
                "question_id": question_id,
                "response": {"option_id": "a"},
            }],
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "submit attempt: {graded}");
    assert_eq!(graded["attempt"]["state"], "graded");
    assert_eq!(graded["attempt"]["passed"], true);
    assert_eq!(graded["attempt"]["score_pct"], "100.00");
}
