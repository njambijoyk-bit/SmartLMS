# J5 — Natural Language Processing Suite
# J5A: Semantic search, J5B: Forum sentiment analysis,
# J5C: Automated discussion facilitation, J5D: Real-time writing feedback

module NLP

export semantic_search, analyze_forum_sentiment, generate_discussion_response,
       writing_feedback, SentimentResult, SearchResult, WritingFeedback

using Statistics

# ─── J5A — Semantic Search ─────────────────────────────────────────────────────

"""
    SearchResult

A single search result with relevance score and source metadata.
"""
struct SearchResult
    id::String
    source_type::String    # "course_content" | "forum" | "library" | "past_exam"
    title::String
    excerpt::String
    relevance_score::Float64
    url::String
end

"""
    term_frequency_idf(query_terms, document_terms)

Compute a simple TF-IDF similarity score between a query and a document.
In production this is replaced by a dense vector similarity using a
sentence-transformer model (e.g. all-MiniLM-L6-v2 via Transformers.jl).
"""
function term_frequency_idf(
    query_terms::Vector{String},
    document_terms::Vector{String},
)::Float64
    isempty(query_terms) || isempty(document_terms) && return 0.0
    doc_set = Set(lowercase.(document_terms))
    matches = count(t -> lowercase(t) in doc_set, query_terms)
    matches / length(query_terms)
end

"""
    semantic_search(query, documents; top_k)

Search across course content, forum threads, and library resources.
Returns top-k results ranked by relevance.

`documents`: Vector of NamedTuples with (id, source_type, title, content, url)
"""
function semantic_search(
    query::String,
    documents::Vector;
    top_k::Int = 10,
)::Vector{SearchResult}
    isempty(documents) && return SearchResult[]

    query_terms = split(lowercase(query), r"\s+")

    scored = map(documents) do doc
        doc_terms = split(lowercase(doc.content), r"\s+")
        score = term_frequency_idf(query_terms, doc_terms)

        # Boost by source type relevance (course content > forum > library)
        boost = if doc.source_type == "course_content"; 1.2
                elseif doc.source_type == "past_exam"; 1.1
                elseif doc.source_type == "forum"; 0.9
                else 1.0 end

        excerpt_start = 1
        excerpt_end = min(200, length(doc.content))
        excerpt = doc.content[excerpt_start:excerpt_end] * (length(doc.content) > 200 ? "..." : "")

        (score * boost, SearchResult(doc.id, doc.source_type, doc.title, excerpt, score * boost, doc.url))
    end

    sorted = sort(scored, by=x -> x[1], rev=true)
    [r for (_, r) in sorted[1:min(top_k, length(sorted))]]
end

# ─── J5B — Forum Sentiment Analysis ───────────────────────────────────────────

"""
    SentimentResult

Sentiment classification with category-specific flags.
"""
struct SentimentResult
    text_id::String
    sentiment::String          # "positive" | "neutral" | "negative"
    confidence::Float64
    confusion_score::Float64   # 0-1 — signals academic confusion
    stress_score::Float64      # 0-1 — signals high stress / distress
    toxicity_score::Float64    # 0-1 — signals policy-violating content
    flags::Vector{String}      # human-readable flag descriptions
    action_required::String    # "none" | "instructor" | "counsellor" | "admin"
end

# Lexicon-based classifier (neural model in production)
const NEGATIVE_WORDS = Set(["confused", "lost", "struggling", "failing", "hopeless",
    "unfair", "wrong", "terrible", "awful", "hate", "impossible", "useless",
    "waste", "boring", "give up"])
const CONFUSION_PHRASES = ["don't understand", "doesn't make sense", "how do i",
    "what does", "what is", "i'm confused", "confused about", "not sure"]
const STRESS_PHRASES = ["can't sleep", "too much work", "overwhelmed", "breaking down",
    "give up", "can't take", "no time", "burnout", "burned out", "exhausted"]
const TOXIC_WORDS = Set(["stupid", "idiot", "dumb", "hate", "racist", "sexist",
    "cheat", "plagiar"])

"""
    analyze_forum_sentiment(post_id, text)

Classify a forum post and flag it for the appropriate audience.
- Confusion → instructor
- High stress / distress → counsellor
- Negative cohort trend → admin
"""
function analyze_forum_sentiment(post_id::String, text::String)::SentimentResult
    text_lower = lowercase(text)
    words = Set(split(text_lower, r"\s+"))

    neg_count = count(w -> w in NEGATIVE_WORDS, words)
    confusion_score = count(p -> occursin(p, text_lower), CONFUSION_PHRASES) / 5.0
    stress_score = count(p -> occursin(p, text_lower), STRESS_PHRASES) / 5.0
    toxicity_score = count(w -> w in TOXIC_WORDS, words) / 3.0

    # Overall sentiment
    total_words = max(length(words), 1)
    neg_ratio = neg_count / total_words
    sentiment = neg_ratio > 0.08 ? "negative" : neg_ratio > 0.03 ? "neutral" : "positive"
    confidence = clamp(0.5 + abs(neg_ratio - 0.05) * 5, 0.5, 0.95)

    flags = String[]
    confusion_score > 0.3 && push!(flags, "Student appears confused about the topic")
    stress_score > 0.3 && push!(flags, "Post suggests high stress or distress")
    toxicity_score > 0.3 && push!(flags, "Post may violate community guidelines")

    action = if toxicity_score > 0.4; "admin"
             elseif stress_score > 0.4; "counsellor"
             elseif confusion_score > 0.3; "instructor"
             else "none" end

    SentimentResult(
        post_id, sentiment,
        clamp(confidence, 0.0, 1.0),
        clamp(confusion_score, 0.0, 1.0),
        clamp(stress_score, 0.0, 1.0),
        clamp(toxicity_score, 0.0, 1.0),
        flags, action,
    )
end

"""
    aggregate_course_sentiment(results)

Aggregate sentiment across a course forum.
Returns (avg_confusion, avg_stress, trend) for instructor dashboard.
"""
function aggregate_course_sentiment(results::Vector{SentimentResult})
    isempty(results) && return (0.0, 0.0, "stable")
    avg_confusion = mean(r.confusion_score for r in results)
    avg_stress = mean(r.stress_score for r in results)
    negative_pct = count(r -> r.sentiment == "negative", results) / length(results)
    trend = negative_pct > 0.3 ? "declining" : negative_pct < 0.1 ? "positive" : "stable"
    (avg_confusion, avg_stress, trend)
end

# ─── J5C — Automated Discussion Facilitation ──────────────────────────────────

"""
    generate_discussion_response(thread_id, question, course_context, resources)

When a thread has gone 48h without instructor response, Julia can:
1. Draft a suggested response for instructor approval
2. Post a curated resource link as "SmartLMS Assistant"
Julia never posts as if it is the instructor.
"""
function generate_discussion_response(
    thread_id::String,
    question::String,
    course_context::String;
    available_resources::Vector{NamedTuple} = NamedTuple[],
)::NamedTuple
    question_lower = lowercase(question)

    # Find the most relevant resource
    best_resource = if !isempty(available_resources)
        q_terms = split(question_lower, r"\s+")
        scored = [(count(t -> occursin(t, lowercase(r.title * " " * r.description)), q_terms), r)
                  for r in available_resources]
        sort!(scored, by=x -> x[1], rev=true)
        first(scored)[2]
    else
        nothing
    end

    # Draft response (template-based — full LLM generation in production)
    draft = if !isnothing(best_resource)
        """Hi! This looks like a question about $(course_context).

You might find this resource helpful: **[$(best_resource.title)]($(best_resource.url))**

_This response was drafted by SmartLMS Assistant and is pending instructor review._"""
    else
        """Thanks for your question about $(course_context). Your instructor will respond soon.

In the meantime, check the course materials for related content.

_This response was drafted by SmartLMS Assistant and is pending instructor review._"""
    end

    (
        thread_id = thread_id,
        draft_response = draft,
        confidence = isnothing(best_resource) ? 0.4 : 0.7,
        recommended_resource = best_resource,
        action = "queue_for_instructor_approval",
        label = "SmartLMS Assistant",  # never posts as instructor
    )
end

# ─── J5D — Real-time Writing Feedback ─────────────────────────────────────────

"""
    WritingFeedback

Real-time writing coaching — not AI writing the assignment, but coaching
the student to write better.
"""
struct WritingFeedback
    clarity_issues::Vector{NamedTuple}      # (sentence, issue, suggestion)
    structure_feedback::Vector{String}
    academic_language::Vector{NamedTuple}   # (word, suggested_alternative)
    rubric_coverage::NamedTuple             # (covered_count, total, missing)
    word_count_pacing::String
    overall_coaching_tip::String
end

"""
    writing_feedback(draft_text, rubric_criteria, target_word_count)

Provide real-time coaching as a student writes their assignment.
This is AI coaching to write better — not AI writing the assignment.
"""
function writing_feedback(
    draft_text::String;
    rubric_criteria::Vector{String} = String[],
    target_word_count::Int = 1000,
)::WritingFeedback
    words = split(strip(draft_text), r"\s+")
    word_count = length(words)
    sentences = filter(!isempty, strip.(split(draft_text, r"[.!?]+")))

    # Clarity: flag very long sentences
    clarity_issues = NamedTuple[]
    for s in sentences
        s_words = split(strip(s), r"\s+")
        if length(s_words) > 50
            push!(clarity_issues, (
                sentence = first(s, 80) * "...",
                issue = "This sentence is $(length(s_words)) words long — consider splitting it",
                suggestion = "Break into two sentences at a natural pause point",
            ))
        end
    end

    # Structure feedback
    structure_feedback = String[]
    if length(split(draft_text, r"\n\n+")) < 3
        push!(structure_feedback, "Consider organising into distinct paragraphs: introduction, body, conclusion")
    end
    paragraphs = split(draft_text, r"\n\n+")
    if !isempty(paragraphs)
        first_para = first(paragraphs[1], 200)
        !occursin(r"[Ii] (will|argue|discuss|explore|examine|show)", first_para) &&
            push!(structure_feedback, "Your introduction could be clearer — state your main argument explicitly")
    end

    # Academic language suggestions
    informal_to_formal = Dict(
        "really important" => "significant",
        "a lot of" => "numerous",
        "shows that" => "demonstrates that",
        "because of this" => "consequently",
        "get better" => "improve",
        "find out" => "determine",
        "kind of" => "somewhat",
        "very big" => "substantial",
    )
    academic_suggestions = NamedTuple[]
    for (informal, formal) in informal_to_formal
        if occursin(informal, lowercase(draft_text))
            push!(academic_suggestions, (word = informal, suggestion = "Consider '$(formal)' instead"))
        end
    end

    # Rubric coverage
    covered = count(c -> any(w -> occursin(lowercase(w), lowercase(draft_text)),
                             filter(x -> length(x) >= 5, split(c, r"\s+"))),
                    rubric_criteria)
    missing_criteria = rubric_criteria[findall(c -> !any(
        w -> occursin(lowercase(w), lowercase(draft_text)),
        filter(x -> length(x) >= 5, split(c, r"\s+"))), rubric_criteria)]
    rubric_cov = (covered_count = covered, total = length(rubric_criteria), missing = missing_criteria)

    # Word count pacing
    pct = word_count / target_word_count * 100
    pacing = if pct < 25; "You're at $(word_count) words — you have plenty of room to develop your ideas"
             elseif pct < 50; "Good start — $(word_count)/$(target_word_count) words written"
             elseif pct < 80; "On track — $(word_count)/$(target_word_count) words"
             elseif pct < 100; "Almost there — $(word_count)/$(target_word_count) words, start wrapping up"
             else "Over the target ($(word_count)/$(target_word_count) words) — consider editing for conciseness" end

    # Overall coaching tip
    tip = if !isempty(clarity_issues); "Focus next on breaking up long sentences for clarity."
          elseif !isempty(missing_criteria); "Make sure to address: $(join(missing_criteria[1:min(2,end)], ", "))"
          elseif !isempty(academic_suggestions); "Strengthen your academic register by replacing informal phrases."
          else "Looking good — review your argument structure and add more evidence." end

    WritingFeedback(clarity_issues, structure_feedback, academic_suggestions, rubric_cov, pacing, tip)
end

end # module NLP
