# J2 — Content Generation Engine
# J2A: Course skeleton generator, J2B: Question generation from content,
# J2C: Question bank analysis, J2D: Auto-summarisation and study guides

module ContentGeneration

export generate_course_skeleton, generate_questions_from_content,
       analyze_question_bank, generate_study_guide,
       CourseOutline, GeneratedQuestion, QuestionBankReport, StudyGuide

using Statistics

# ─── J2A — Course Skeleton Generator ──────────────────────────────────────────

"""
    LessonOutline

A single lesson in the generated course structure.
"""
struct LessonOutline
    number::Int
    title::String
    lesson_type::String      # "video" | "reading" | "quiz" | "assignment" | "discussion"
    estimated_duration_min::Int
    learning_outcomes::Vector{String}
    bloom_level::Int         # 1–6 (Bloom's taxonomy)
end

"""
    UnitOutline

A unit (group of modules/lessons) in the course.
"""
struct UnitOutline
    number::Int
    title::String
    description::String
    lessons::Vector{LessonOutline}
    unit_assessment::Bool
end

"""
    CourseOutline

A full course structure generated from title, level, and prerequisites.
"""
struct CourseOutline
    course_title::String
    course_level::String         # "introductory" | "intermediate" | "advanced" | "postgraduate"
    prerequisites::Vector{String}
    learning_outcomes::Vector{String}
    units::Vector{UnitOutline}
    estimated_total_hours::Int
    recommended_assessment_types::Vector{String}
    bloom_alignment::Dict{String, Int}  # bloom level → question count recommended
    generated_at::Float64
end

# Predefined templates for common course structures
const COURSE_TEMPLATES = Dict(
    "introductory" => (units=5, lessons_per_unit=4, assessment_frequency=2),
    "intermediate" => (units=7, lessons_per_unit=5, assessment_frequency=2),
    "advanced"     => (units=8, lessons_per_unit=6, assessment_frequency=3),
    "postgraduate" => (units=6, lessons_per_unit=8, assessment_frequency=2),
)

const LESSON_TYPE_SEQUENCE = ["video", "reading", "quiz", "video", "reading", "assignment"]
const BLOOM_LEVELS = ["Remember", "Understand", "Apply", "Analyse", "Evaluate", "Create"]

"""
    generate_course_skeleton(title, level, prerequisites, domain)

Generate a complete course structure with units, lessons, outcomes,
and assessment mapping. Instructor reviews and customises before publishing.
"""
function generate_course_skeleton(
    title::String,
    level::String;
    prerequisites::Vector{String} = String[],
    domain::String = "general",
)::CourseOutline
    template = get(COURSE_TEMPLATES, level, COURSE_TEMPLATES["intermediate"])

    # Generate course-level learning outcomes (Bloom's progression)
    outcomes = [
        "Identify and describe the core concepts of $(title)",
        "Explain the key principles and how they relate to each other",
        "Apply $(title) techniques to solve real-world problems",
        "Analyse and evaluate different approaches in $(domain)",
        "Design and create solutions using advanced $(title) concepts",
    ]

    # Generate units
    units = UnitOutline[]
    total_minutes = 0
    for u in 1:template.units
        # Generate lessons within each unit
        lessons = LessonOutline[]
        for l in 1:template.lessons_per_unit
            lesson_type = LESSON_TYPE_SEQUENCE[(l - 1) % length(LESSON_TYPE_SEQUENCE) + 1]
            bloom = min(1 + (u - 1) ÷ 2, 6)  # Bloom's level increases with unit
            duration = lesson_type == "video" ? 20 :
                       lesson_type == "reading" ? 30 :
                       lesson_type == "quiz" ? 15 :
                       lesson_type == "assignment" ? 60 : 20
            total_minutes += duration

            push!(lessons, LessonOutline(
                l,
                "$(title) — Unit $u, Lesson $l",
                lesson_type,
                duration,
                ["$(BLOOM_LEVELS[bloom]): $(title) concept $(u).$(l)"],
                bloom,
            ))
        end

        push!(units, UnitOutline(
            u,
            "Unit $u: $(title) — Part $u",
            "Core concepts for Unit $u of $(title)",
            lessons,
            u % template.assessment_frequency == 0,
        ))
    end

    bloom_alignment = Dict(
        "remember_understand" => template.units * 3,
        "apply" => template.units * 2,
        "analyse_evaluate" => template.units * 2,
        "create" => template.units,
    )

    CourseOutline(
        title, level, prerequisites, outcomes, units,
        total_minutes ÷ 60,
        ["CAT", "Assignment", "Project", "Final Exam"],
        bloom_alignment,
        time(),
    )
end

# ─── J2B — Question Generation from Content ───────────────────────────────────

"""
    GeneratedQuestion

A question generated from uploaded content.
"""
struct GeneratedQuestion
    id::String
    question_text::String
    question_type::String    # "mcq" | "short_answer" | "essay"
    bloom_level::Int
    bloom_label::String
    difficulty::String       # "easy" | "medium" | "hard"
    options::Vector{String}  # for MCQ only
    correct_answer::Union{String, Int}  # text for open, index for MCQ
    model_answer::String
    marking_criteria::Vector{String}
    topic_tag::String
    confidence::Float64      # how confident the model is in this question
    requires_review::Bool
end

"""
    extract_key_sentences(text, n)

Extract the top-n most information-dense sentences from a text.
Simple TF-IDF-based extractive summarisation.
"""
function extract_key_sentences(text::String, n::Int = 5)::Vector{String}
    sentences = filter(!isempty, strip.(split(text, r"[.!?]+")))
    isempty(sentences) && return String[]

    # Score each sentence by term frequency of significant words
    all_words = lowercase.(split(text, r"\s+"))
    word_freq = Dict{String, Int}()
    for w in all_words
        length(w) > 4 || continue
        word_freq[w] = get(word_freq, w, 0) + 1
    end

    scores = map(sentences) do s
        s_words = lowercase.(split(s, r"\s+"))
        score = sum(get(word_freq, w, 0) for w in s_words if length(w) > 4)
        score / max(length(s_words), 1)
    end

    sorted_idx = sortperm(scores, rev=true)
    sentences[sorted_idx[1:min(n, length(sorted_idx))]]
end

"""
    generate_questions_from_content(content_id, text; num_questions, bloom_distribution)

Generate questions from uploaded PDF/video transcript/webpage text.
Returns questions for instructor review before adding to question bank.
"""
function generate_questions_from_content(
    content_id::String,
    text::String;
    num_questions::Int = 10,
    bloom_distribution::Dict{Int,Int} = Dict(1=>3, 2=>3, 3=>2, 4=>1, 5=>1),
)::Vector{GeneratedQuestion}
    key_sentences = extract_key_sentences(text, num_questions * 2)
    isempty(key_sentences) && return GeneratedQuestion[]

    questions = GeneratedQuestion[]
    q_id = 1

    for (bloom_level, count) in bloom_distribution
        for _ in 1:count
            q_id > length(key_sentences) && break
            sentence = key_sentences[q_id]
            words = split(sentence, r"\s+")

            # Generate question based on Bloom's level
            (q_text, q_type, answer, criteria) = if bloom_level <= 2
                # Remember/Understand — MCQ
                q = "According to the text: '$(first(sentence, 80))'... which statement is correct?"
                options = [
                    sentence,
                    "The opposite of: $(first(sentence, 40))",
                    "A common misconception about this topic",
                    "An unrelated fact",
                ]
                (q, "mcq", 1, ["Correctly identifies the factual information from the text"])
            elseif bloom_level == 3
                # Apply — short answer
                topic = join(words[1:min(4, length(words))], " ")
                q = "How would you apply the concept of '$(topic)' in a practical scenario?"
                (q, "short_answer", sentence,
                 ["Demonstrates understanding of the concept", "Provides a concrete example", "Links theory to practice"])
            else
                # Analyse/Evaluate/Create — essay prompt
                q = "Critically analyse the following statement from the course material: '$(first(sentence, 100))...'"
                (q, "essay", sentence,
                 ["Shows analytical thinking", "Uses evidence to support argument",
                  "Considers alternative perspectives", "Demonstrates higher-order reasoning"])
            end

            push!(questions, GeneratedQuestion(
                "$(content_id)-q$(q_id)",
                q_text, q_type, bloom_level,
                BLOOM_LEVELS[bloom_level], "medium",
                q_type == "mcq" ? answer isa Vector ? answer : [answer, "Incorrect option 1", "Incorrect option 2", "Incorrect option 3"] : String[],
                q_type == "mcq" ? 1 : answer,
                sentence,
                criteria,
                "auto-generated",
                0.6,  # lower confidence — needs instructor review
                true, # always require instructor review for AI-generated questions
            ))
            q_id += 1
        end
    end

    questions
end

# ─── J2C — Question Bank Analysis ─────────────────────────────────────────────

"""
    QuestionBankReport

Analysis of the health of a question bank.
"""
struct QuestionBankReport
    total_questions::Int
    bloom_distribution::Dict{Int, Int}
    difficulty_distribution::Dict{String, Int}
    too_easy_questions::Vector{String}   # question IDs with >90% correct rate
    duplicate_candidates::Vector{Tuple{String,String,Float64}}  # (id1, id2, similarity)
    coverage_gaps::Vector{String}        # topics with fewer than 3 questions
    recommendations::Vector{String}
end

"""
    analyze_question_bank(questions, performance_stats)

Analyse question bank quality and provide recommendations.

`performance_stats`: Dict of question_id → (correct_rate, attempt_count)
"""
function analyze_question_bank(
    questions::Vector,
    performance_stats::Dict{String, Tuple{Float64, Int}} = Dict{String, Tuple{Float64, Int}}(),
)::QuestionBankReport
    isempty(questions) && return QuestionBankReport(0, Dict(), Dict(), [], [], [], ["Question bank is empty."])

    # Bloom's distribution
    bloom_dist = Dict{Int, Int}()
    for q in questions
        bl = get(q, :bloom_level, 1)
        bloom_dist[bl] = get(bloom_dist, bl, 0) + 1
    end

    # Difficulty distribution
    diff_dist = Dict{String, Int}()
    for q in questions
        d = get(q, :difficulty, "medium")
        diff_dist[d] = get(diff_dist, d, 0) + 1
    end

    # Too-easy questions (> 90% correct rate)
    too_easy = [id for (id, (rate, _)) in performance_stats if rate > 0.90]

    # Topic coverage gaps
    topic_counts = Dict{String, Int}()
    for q in questions
        t = get(q, :topic, "general")
        topic_counts[t] = get(topic_counts, t, 0) + 1
    end
    gaps = [t for (t, c) in topic_counts if c < 3]

    # Recommendations
    recs = String[]
    total = length(questions)
    recall_pct = get(bloom_dist, 1, 0) / max(total, 1) * 100
    recall_pct > 60 && push!(recs, "$(round(Int,recall_pct))% of questions are recall-level. Add higher-order questions (Bloom's 4–6).")
    !isempty(too_easy) && push!(recs, "$(length(too_easy)) question(s) answered correctly by >90% of students — consider increasing difficulty.")
    !isempty(gaps) && push!(recs, "$(length(gaps)) topic(s) have fewer than 3 questions: $(join(gaps[1:min(3,end)], ", "))")
    total < 30 && push!(recs, "Question bank has only $(total) questions — build to at least 50 for reliable random sampling.")

    QuestionBankReport(total, bloom_dist, diff_dist, too_easy, [], gaps, recs)
end

# ─── J2D — Study Guide & Summarisation ────────────────────────────────────────

"""
    StudyGuide

Auto-generated study guide for a chapter/topic.
"""
struct StudyGuide
    topic::String
    chapter_summary::String
    key_concepts::Vector{NamedTuple}     # (term, definition, page_reference)
    concept_map::Vector{Tuple{String,String,String}}  # (concept_a, relationship, concept_b)
    likely_exam_topics::Vector{String}
    personalised_checklist::Vector{NamedTuple}  # (topic, student_mastery, priority)
    generated_at::Float64
end

"""
    generate_study_guide(topic, content_text; knowledge_state, question_bank_weights)

Generate a personalised study guide for a student based on:
- Content text (chapter/lecture notes)
- Student's current knowledge state from DKT
- Question bank weighting (what topics appear most)
"""
function generate_study_guide(
    topic::String,
    content_text::String;
    knowledge_state::Dict{String, Float64} = Dict{String,Float64}(),
    question_bank_weights::Dict{String, Float64} = Dict{String,Float64}(),
)::StudyGuide
    # Extractive summary
    key_sentences = extract_key_sentences(content_text, 5)
    summary = join(key_sentences, " ")
    length(summary) > 500 && (summary = first(summary, 500) * "...")

    # Key concepts: extract noun phrases (simplified)
    words = split(content_text, r"\s+")
    unique_words = unique(filter(w -> length(w) >= 6 && !startswith(w, r"[0-9]"),
                                  lowercase.(words)))
    top_concepts = unique_words[1:min(10, length(unique_words))]
    key_concepts = [(term=w, definition="Key concept related to $(topic)", page_reference="—") for w in top_concepts]

    # Likely exam topics based on question bank frequency
    sorted_topics = sort(collect(question_bank_weights), by=x -> x[2], rev=true)
    likely_topics = isempty(sorted_topics) ? ["Core concepts of $(topic)"] :
                    [t for (t, _) in sorted_topics[1:min(5, end)]]

    # Personalised checklist based on knowledge state
    checklist = if !isempty(knowledge_state)
        sorted_ks = sort(collect(knowledge_state), by=x -> x[2])
        [(topic=k, mastery=round(v*100, digits=0), priority=v < 0.5 ? "high" : v < 0.7 ? "medium" : "low")
         for (k, v) in sorted_ks]
    else
        [(topic=topic, mastery=0.0, priority="high")]
    end

    StudyGuide(
        topic, summary, key_concepts,
        [(topic, "relates to", t) for t in likely_topics[1:min(3,end)]],
        likely_topics, checklist, time(),
    )
end

end # module ContentGeneration
