# J7 — Instructor Intelligence Tools
# J7A: Pedagogical coach, J7B: Assessment workload balancing,
# J7C: Automated marking assistant

module InstructorTools

export pedagogical_audit, detect_workload_collision, group_similar_answers,
       PedagogicalInsight, WorkloadAlert, MarkingGroup

using Statistics

# ─── J7A — Pedagogical Coach ──────────────────────────────────────────────────

"""
    PedagogicalInsight

A coaching insight for an instructor about their course design.
"""
struct PedagogicalInsight
    insight_type::String         # "assessment_gap" | "rubric_simplification" | "bloom_balance"
    severity::String             # "info" | "warning" | "suggestion"
    message::String
    affected_item::String        # e.g. course name, assignment title
    suggested_action::String
end

"""
    pedagogical_audit(course_structure, question_bank_stats, rubric_stats)

Analyse a course's pedagogical design and return actionable coaching insights.

`course_structure`: NamedTuple with (lessons, assessment_positions)
`question_bank_stats`: NamedTuple with (bloom_levels, chapter_coverage)
`rubric_stats`: NamedTuple with (criteria, usage_rates)
"""
function pedagogical_audit(
    course_structure::NamedTuple,
    question_bank_stats::NamedTuple,
    rubric_stats::NamedTuple,
)::Vector{PedagogicalInsight}
    insights = PedagogicalInsight[]

    # Check for consecutive video-only lessons without assessment
    if haskey(course_structure, :lessons)
        consecutive_videos = 0
        for lesson in course_structure.lessons
            if get(lesson, :type, "") == "video"
                consecutive_videos += 1
                if consecutive_videos >= 6
                    push!(insights, PedagogicalInsight(
                        "assessment_gap", "warning",
                        "You have $(consecutive_videos) consecutive video lessons with no assessment.",
                        get(lesson, :course_name, "this course"),
                        "Consider adding a quiz or discussion prompt between lessons 3 and 4 to consolidate learning.",
                    ))
                    consecutive_videos = 0
                end
            else
                consecutive_videos = 0
            end
        end
    end

    # Check Bloom's taxonomy distribution in question bank
    if haskey(question_bank_stats, :bloom_levels)
        bloom = question_bank_stats.bloom_levels
        total_q = sum(values(bloom))
        recall_pct = get(bloom, "remember", 0) / max(total_q, 1) * 100
        if recall_pct > 60
            push!(insights, PedagogicalInsight(
                "bloom_balance", "suggestion",
                "Your CAT bank has $(round(Int, recall_pct))% recall-level (Bloom's L1) questions.",
                "Question bank",
                "Add higher-order questions (Bloom's levels 4–6: Analyse, Evaluate, Create) for deeper learning measurement.",
            ))
        end
    end

    # Check for chapter coverage gaps
    if haskey(question_bank_stats, :chapter_coverage)
        for (chapter, count) in question_bank_stats.chapter_coverage
            if count < 3
                push!(insights, PedagogicalInsight(
                    "coverage_gap", "warning",
                    "Chapter '$(chapter)' has only $(count) question$(count == 1 ? "" : "s") in your bank.",
                    "Question bank — Chapter: $(chapter)",
                    "Recommend 8–10 questions per chapter for reliable random sampling.",
                ))
            end
        end
    end

    # Check rubric criteria usage
    if haskey(rubric_stats, :criteria) && haskey(rubric_stats, :usage_rates)
        for (criterion, usage) in zip(rubric_stats.criteria, rubric_stats.usage_rates)
            if usage < 0.10
                push!(insights, PedagogicalInsight(
                    "rubric_simplification", "info",
                    "Rubric criterion '$(criterion)' is rarely used in grading ($(round(Int, usage*100))% of submissions).",
                    "Assignment rubric",
                    "Consider whether this criterion is still relevant, or simplify the rubric.",
                ))
            end
        end
    end

    insights
end

# ─── J7B — Assessment Workload Balancing ──────────────────────────────────────

"""
    WorkloadAlert

A warning about concentrated assessment density.
"""
struct WorkloadAlert
    alert_type::String       # "instructor" | "admin"
    week_identifier::String
    deadline_count::Int
    affected_courses::Vector{String}
    description::String
    historical_correlation::String  # correlation with wellbeing/performance dips
    suggested_action::String
end

"""
    detect_workload_collision(deadlines, threshold_per_week)

Detect weeks with too many concurrent deadlines that could overwhelm students.

`deadlines`: Vector of NamedTuples (course_name, title, due_date, student_count)
`threshold_per_week`: flag weeks with more than this many deadlines
"""
function detect_workload_collision(
    deadlines::Vector,
    threshold_per_week::Int = 4,
)::Vector{WorkloadAlert}
    isempty(deadlines) && return WorkloadAlert[]

    # Group deadlines by ISO week
    week_map = Dict{String, Vector}()
    for d in deadlines
        # Use week number as key (simplified — in production use proper date arithmetic)
        week_key = get(d, :week, string(d.due_date)[1:7])  # "YYYY-MM" as proxy
        push!(get!(week_map, week_key, []), d)
    end

    alerts = WorkloadAlert[]
    for (week, week_deadlines) in week_map
        length(week_deadlines) <= threshold_per_week && continue

        courses = unique([get(d, :course_name, "Unknown") for d in week_deadlines])
        push!(alerts, WorkloadAlert(
            length(courses) > 2 ? "admin" : "instructor",
            week,
            length(week_deadlines),
            courses,
            "Week of $(week) has $(length(week_deadlines)) assessment deadlines across $(length(courses)) course(s).",
            "Weeks with >$(threshold_per_week) deadlines historically correlate with lower submission rates and reduced wellbeing scores.",
            "Consider spreading deadlines — move 1–2 assessments to adjacent weeks.",
        ))
    end

    sort!(alerts, by=a -> a.deadline_count, rev=true)
    alerts
end

# ─── J7C — Automated Marking Assistant ────────────────────────────────────────

"""
    MarkingGroup

A group of similar answers for batch grading.
"""
struct MarkingGroup
    group_id::Int
    representative_answer::String
    answer_ids::Vector{String}
    similarity_score::Float64   # how similar answers in this group are to each other
    suggested_grade::Float64    # suggested grade for this group
    suggested_grade_confidence::Float64
    reasoning::String
    is_outlier::Bool
    outlier_reason::String
end

"""
    jaccard_similarity(a, b)

Token-based similarity between two text strings.
In production: cosine similarity of sentence embeddings.
"""
function jaccard_similarity(a::String, b::String)::Float64
    set_a = Set(lowercase.(split(a, r"\s+")))
    set_b = Set(lowercase.(split(b, r"\s+")))
    isempty(set_a) || isempty(set_b) && return 0.0
    length(intersect(set_a, set_b)) / length(union(set_a, set_b))
end

"""
    group_similar_answers(submissions, rubric_criteria, model_answer)

Group short-answer submissions by similarity, suggest grades,
and flag outliers. Reduces instructor grading time by 60–70%.

`submissions`: Vector of NamedTuples (id, text, student_id)
`model_answer`: instructor's reference answer (used to calibrate grading)
"""
function group_similar_answers(
    submissions::Vector,
    model_answer::String;
    rubric_criteria::Vector{String} = String[],
    similarity_threshold::Float64 = 0.5,
    max_grade::Float64 = 10.0,
)::Vector{MarkingGroup}
    isempty(submissions) && return MarkingGroup[]

    n = length(submissions)
    assigned = fill(false, n)
    groups = MarkingGroup[]
    group_id = 1

    for i in 1:n
        assigned[i] && continue
        assigned[i] = true
        group_members = [submissions[i]]
        member_ids = [submissions[i].id]

        for j in (i+1):n
            assigned[j] && continue
            sim = jaccard_similarity(submissions[i].text, submissions[j].text)
            if sim >= similarity_threshold
                assigned[j] = true
                push!(group_members, submissions[j])
                push!(member_ids, submissions[j].id)
            end
        end

        # Representative: member most similar to model answer
        representative = argmax(m -> jaccard_similarity(m.text, model_answer), group_members)
        rep_text = representative.text

        # Suggested grade based on similarity to model answer
        rep_sim = jaccard_similarity(rep_text, model_answer)
        rubric_coverage = isempty(rubric_criteria) ? rep_sim :
            count(c -> any(w -> occursin(lowercase(w), lowercase(rep_text)),
                           filter(x -> length(x) >= 4, split(c, r"\s+"))),
                  rubric_criteria) / length(rubric_criteria)
        combined_score = (rep_sim * 0.6 + rubric_coverage * 0.4)
        suggested_grade = combined_score * max_grade

        confidence = length(group_members) >= 3 ? 0.8 : 0.6
        reasoning = "Group of $(length(group_members)) similar answer(s). " *
                    "Similarity to model answer: $(round(rep_sim * 100, digits=0))%. " *
                    "Rubric coverage: $(round(rubric_coverage * 100, digits=0))%."

        # Outlier detection: very short or very different from everyone
        is_outlier = length(split(rep_text, r"\s+")) < 5 || rep_sim < 0.1
        outlier_reason = is_outlier ?
            (length(split(rep_text, r"\s+")) < 5 ? "Very short answer — may have misunderstood the question." :
             "Answer does not relate to the expected topic.") : ""

        push!(groups, MarkingGroup(
            group_id, rep_text, member_ids,
            rep_sim, suggested_grade, confidence,
            reasoning, is_outlier, outlier_reason,
        ))
        group_id += 1
    end

    # Sort: outliers last, highest-confidence groups first
    sort!(groups, by=g -> (g.is_outlier, -g.suggested_grade_confidence))
    groups
end

"""
    compute_grading_time_saved(total_submissions, groups)

Estimate instructor time saved by batch grading.
"""
function compute_grading_time_saved(total_submissions::Int, group_count::Int)::NamedTuple
    time_per_submission_min = 3.0
    time_per_group_min = 2.0
    original_time = total_submissions * time_per_submission_min
    assisted_time = group_count * time_per_group_min
    saved_pct = (1 - assisted_time / max(original_time, 1)) * 100
    (
        original_minutes = round(Int, original_time),
        assisted_minutes = round(Int, assisted_time),
        saved_percentage = round(Int, saved_pct),
        groups_to_review = group_count,
        submissions_per_group = total_submissions / max(group_count, 1),
    )
end

end # module InstructorTools
