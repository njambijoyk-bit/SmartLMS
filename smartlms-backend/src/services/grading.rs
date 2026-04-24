//! Auto-grading logic for objective question types.
//!
//! Pure functions — no DB access. Makes the grader trivial to unit-test.
//!
//! Supported auto-graded kinds (all-or-nothing unless stated otherwise):
//!
//!   * `mcq`          — response `{"option_id": "a"}` matches `answer.option_id`
//!   * `multi`        — response `{"option_ids": [...]}` must equal `answer.option_ids`
//!     as a set (order-independent), no partial credit
//!   * `true_false`   — response `{"value": true|false}` matches `answer.value`
//!   * `short_answer` — response `{"text": "..."}` must appear in `answer.accepted`;
//!     trimmed; case-insensitive iff `answer.ci` is true (default true)
//!   * `numeric`      — response `{"value": 3.14}` within `body.tolerance` (default 0)
//!     of `answer.value`
//!
//! Essay / code are returned with `graded_by = "pending"` and zero points so the
//! attempt enters the `submitted` state and a human grader can take over.

use rust_decimal::Decimal;
use std::collections::HashSet;

use crate::models::assessment::QuestionKind;

#[derive(Debug, Clone)]
pub struct GradeOutcome {
    /// `Some(true)` / `Some(false)` for auto-graded, `None` for pending.
    pub is_correct: Option<bool>,
    pub points_earned: Decimal,
    pub graded_by: &'static str,
    pub feedback: Option<String>,
}

impl GradeOutcome {
    pub fn pending(points_possible: Decimal) -> Self {
        let _ = points_possible;
        Self {
            is_correct: None,
            points_earned: Decimal::ZERO,
            graded_by: "pending",
            feedback: None,
        }
    }

    pub fn correct(points: Decimal) -> Self {
        Self {
            is_correct: Some(true),
            points_earned: points,
            graded_by: "auto",
            feedback: None,
        }
    }

    pub fn incorrect() -> Self {
        Self {
            is_correct: Some(false),
            points_earned: Decimal::ZERO,
            graded_by: "auto",
            feedback: None,
        }
    }
}

pub fn grade(
    kind: QuestionKind,
    body: &serde_json::Value,
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    match kind {
        QuestionKind::Mcq => grade_mcq(answer, response, points_possible),
        QuestionKind::Multi => grade_multi(answer, response, points_possible),
        QuestionKind::TrueFalse => grade_true_false(answer, response, points_possible),
        QuestionKind::ShortAnswer => grade_short_answer(answer, response, points_possible),
        QuestionKind::Numeric => grade_numeric(body, answer, response, points_possible),
        QuestionKind::Essay | QuestionKind::Code => GradeOutcome::pending(points_possible),
    }
}

fn grade_mcq(
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    let expected = answer.get("option_id").and_then(|v| v.as_str());
    let given = response.get("option_id").and_then(|v| v.as_str());
    match (expected, given) {
        (Some(e), Some(g)) if e == g => GradeOutcome::correct(points_possible),
        _ => GradeOutcome::incorrect(),
    }
}

fn grade_multi(
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    let to_set = |v: &serde_json::Value| -> Option<HashSet<String>> {
        v.get("option_ids")?
            .as_array()?
            .iter()
            .map(|x| x.as_str().map(|s| s.to_string()))
            .collect()
    };
    match (to_set(answer), to_set(response)) {
        (Some(e), Some(g)) if e == g => GradeOutcome::correct(points_possible),
        _ => GradeOutcome::incorrect(),
    }
}

fn grade_true_false(
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    let expected = answer.get("value").and_then(|v| v.as_bool());
    let given = response.get("value").and_then(|v| v.as_bool());
    match (expected, given) {
        (Some(e), Some(g)) if e == g => GradeOutcome::correct(points_possible),
        _ => GradeOutcome::incorrect(),
    }
}

fn grade_short_answer(
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    let accepted = match answer.get("accepted").and_then(|v| v.as_array()) {
        Some(a) => a
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
            .collect::<Vec<_>>(),
        None => return GradeOutcome::incorrect(),
    };
    let ci = answer.get("ci").and_then(|v| v.as_bool()).unwrap_or(true);
    let given = response
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let matched = accepted.iter().any(|candidate| {
        if ci {
            candidate.eq_ignore_ascii_case(&given)
        } else {
            candidate == &given
        }
    });
    if matched {
        GradeOutcome::correct(points_possible)
    } else {
        GradeOutcome::incorrect()
    }
}

fn grade_numeric(
    body: &serde_json::Value,
    answer: &serde_json::Value,
    response: &serde_json::Value,
    points_possible: Decimal,
) -> GradeOutcome {
    let expected = match answer.get("value").and_then(|v| v.as_f64()) {
        Some(v) => v,
        None => return GradeOutcome::incorrect(),
    };
    let given = match response.get("value").and_then(|v| v.as_f64()) {
        Some(v) => v,
        None => return GradeOutcome::incorrect(),
    };
    let tolerance = body
        .get("tolerance")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        .abs();
    if (given - expected).abs() <= tolerance {
        GradeOutcome::correct(points_possible)
    } else {
        GradeOutcome::incorrect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn pts() -> Decimal {
        Decimal::from(5)
    }

    #[test]
    fn mcq_correct_earns_full_points() {
        let out = grade(
            QuestionKind::Mcq,
            &json!({}),
            &json!({"option_id": "a"}),
            &json!({"option_id": "a"}),
            pts(),
        );
        assert_eq!(out.is_correct, Some(true));
        assert_eq!(out.points_earned, pts());
    }

    #[test]
    fn mcq_wrong_earns_zero() {
        let out = grade(
            QuestionKind::Mcq,
            &json!({}),
            &json!({"option_id": "a"}),
            &json!({"option_id": "b"}),
            pts(),
        );
        assert_eq!(out.is_correct, Some(false));
        assert_eq!(out.points_earned, Decimal::ZERO);
    }

    #[test]
    fn multi_set_equality_order_insensitive() {
        let ans = json!({"option_ids": ["a", "b"]});
        let good = grade(
            QuestionKind::Multi,
            &json!({}),
            &ans,
            &json!({"option_ids": ["b", "a"]}),
            pts(),
        );
        assert_eq!(good.is_correct, Some(true));

        let bad = grade(
            QuestionKind::Multi,
            &json!({}),
            &ans,
            &json!({"option_ids": ["a"]}),
            pts(),
        );
        assert_eq!(bad.is_correct, Some(false));
    }

    #[test]
    fn true_false_works() {
        let out = grade(
            QuestionKind::TrueFalse,
            &json!({}),
            &json!({"value": true}),
            &json!({"value": true}),
            pts(),
        );
        assert_eq!(out.is_correct, Some(true));
    }

    #[test]
    fn short_answer_case_insensitive_by_default() {
        let ans = json!({"accepted": ["Paris"]});
        let out = grade(
            QuestionKind::ShortAnswer,
            &json!({}),
            &ans,
            &json!({"text": "  paris  "}),
            pts(),
        );
        assert_eq!(out.is_correct, Some(true));
    }

    #[test]
    fn short_answer_case_sensitive_when_ci_false() {
        let ans = json!({"accepted": ["Paris"], "ci": false});
        let bad = grade(
            QuestionKind::ShortAnswer,
            &json!({}),
            &ans,
            &json!({"text": "paris"}),
            pts(),
        );
        assert_eq!(bad.is_correct, Some(false));
        let good = grade(
            QuestionKind::ShortAnswer,
            &json!({}),
            &ans,
            &json!({"text": "Paris"}),
            pts(),
        );
        assert_eq!(good.is_correct, Some(true));
    }

    #[test]
    fn numeric_within_tolerance() {
        let body = json!({"tolerance": 0.05});
        let ans = json!({"value": 42.0});
        let ok = grade(
            QuestionKind::Numeric,
            &body,
            &ans,
            &json!({"value": 42.04}),
            pts(),
        );
        assert_eq!(ok.is_correct, Some(true));
        let bad = grade(
            QuestionKind::Numeric,
            &body,
            &ans,
            &json!({"value": 42.2}),
            pts(),
        );
        assert_eq!(bad.is_correct, Some(false));
    }

    #[test]
    fn essay_is_pending() {
        let out = grade(
            QuestionKind::Essay,
            &json!({}),
            &json!({}),
            &json!({"text": "..."}),
            pts(),
        );
        assert_eq!(out.is_correct, None);
        assert_eq!(out.graded_by, "pending");
        assert_eq!(out.points_earned, Decimal::ZERO);
    }
}
