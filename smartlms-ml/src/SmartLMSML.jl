# SmartLMS ML Engine - Main Module
# Julia AI/ML tier: adaptive learning, dropout prediction, content generation,
# essay grading, NLP, instructor tools, and DKT knowledge tracing.

module SmartLMSML

using Pkg

# ── J4 / J6: Existing models ────────────────────────────────────────────────
export train_dropout_model, predict_dropout
export train_recommendation_model, get_recommendations
export train_performance_model, predict_performance

include("models/dropout.jl")
include("models/recommendation.jl")
include("models/performance.jl")

# ── J1: Deep Knowledge Tracing (DKT+) ───────────────────────────────────────
export DKT
include("models/dkt.jl")
using .DKT: predict_knowledge_state, forgetting_curve_adjustment

# ── J2: Content Generation Engine ───────────────────────────────────────────
export ContentGeneration
include("models/content_generation.jl")
using .ContentGeneration: generate_course_skeleton, generate_questions_from_content,
                           analyze_question_bank, generate_study_guide

# ── J3A: Automated Essay Grading ────────────────────────────────────────────
export EssayGrader
include("models/essay_grader.jl")
using .EssayGrader: grade_essay, explain_grade

# ── J5: NLP Suite ────────────────────────────────────────────────────────────
export NLP
include("models/nlp.jl")
using .NLP: semantic_search, analyze_forum_sentiment, generate_discussion_response,
             writing_feedback

# ── J7: Instructor Intelligence Tools ───────────────────────────────────────
export InstructorTools
include("models/instructor_tools.jl")
using .InstructorTools: pedagogical_audit, detect_workload_collision, group_similar_answers

end # module SmartLMSML
