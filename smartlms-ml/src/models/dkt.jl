# J1 — Deep Knowledge Tracing (DKT+)
# Tracks how a student's knowledge of each concept evolves over time.
# Uses an LSTM to model temporal knowledge state and concept interdependencies.
# Reference: Piech et al. (2015), Zhang et al. (2017) DKT+

module DKT

using Flux
using Statistics
using LinearAlgebra

export DKTModel, train_dkt!, predict_knowledge_state, forgetting_curve_adjustment

# ─── Model Architecture ────────────────────────────────────────────────────────

"""
    DKTModel

Deep Knowledge Tracing model. Embeds (question, correctness) pairs into a
concept space, then uses an LSTM to track temporal knowledge evolution.
"""
struct DKTModel
    concept_embed::Flux.Embedding   # concept_count → embed_dim
    lstm::Flux.LSTM                  # embed_dim → hidden_dim
    output::Flux.Dense               # hidden_dim → concept_count (probability per concept)
end

Flux.@functor DKTModel

function DKTModel(concept_count::Int; embed_dim::Int=64, hidden_dim::Int=128)
    DKTModel(
        Flux.Embedding(concept_count * 2, embed_dim),  # *2 for (concept, correct/incorrect)
        Flux.LSTM(embed_dim, hidden_dim),
        Flux.Dense(hidden_dim, concept_count, σ),      # sigmoid → probability [0,1]
    )
end

"""
    (m::DKTModel)(sequence)

Forward pass. `sequence` is a vector of (concept_id, was_correct) pairs encoded
as a single integer: concept_id * 2 + was_correct.

Returns: matrix [concept_count × seq_len] — predicted mastery probability for
each concept at each time step.
"""
function (m::DKTModel)(sequence::Vector{Int})
    Flux.reset!(m.lstm)
    predictions = []
    for token in sequence
        embedded = m.concept_embed(token)
        hidden = m.lstm(embedded)
        pred = m.output(hidden)
        push!(predictions, pred)
    end
    hcat(predictions...)
end

# ─── Training ──────────────────────────────────────────────────────────────────

"""
    train_dkt!(model, sequences; epochs, lr)

Train on a dataset of student interaction sequences.

`sequences` is a Vector of NamedTuples: (tokens=[...], targets=[...])
  - tokens: encoded (concept, correct) pairs
  - targets: concept that was tested at each step (for loss computation)
"""
function train_dkt!(
    model::DKTModel,
    sequences;
    epochs::Int = 20,
    lr::Float64 = 0.001,
    verbose::Bool = true,
)
    opt = Flux.Adam(lr)
    opt_state = Flux.setup(opt, model)

    for epoch in 1:epochs
        total_loss = 0.0
        count = 0

        for seq in sequences
            isempty(seq.tokens) && continue

            loss, grads = Flux.withgradient(model) do m
                preds = m(seq.tokens)          # [concept_count × T]
                # Cross-entropy loss: for each step t, target concept index
                l = 0.0
                for (t, target_concept) in enumerate(seq.targets)
                    p = clamp(preds[target_concept, t], 1e-7, 1 - 1e-7)
                    l -= seq.was_correct[t] * log(p) + (1 - seq.was_correct[t]) * log(1 - p)
                end
                l / length(seq.targets)
            end

            Flux.update!(opt_state, model, grads[1])
            total_loss += loss
            count += 1
        end

        if verbose && epoch % 5 == 0
            @info "DKT Epoch $epoch | Avg Loss: $(round(total_loss / max(count,1), digits=4))"
        end
    end
end

# ─── Inference ─────────────────────────────────────────────────────────────────

"""
    KnowledgeState

Per-concept mastery probabilities with metadata.
"""
struct KnowledgeState
    student_id::String
    concept_masteries::Dict{String, Float64}   # concept_name → mastery [0,1]
    flagged_concepts::Vector{String}            # concepts below threshold
    predicted_cat_performance::Float64
    weakest_prerequisite::Union{String, Nothing}
    recommended_practice::Vector{String}
    computed_at::Float64  # unix timestamp
end

"""
    predict_knowledge_state(model, student_history, concept_names; threshold)

Given a student's interaction history, return their current knowledge state.

`student_history`: Vector of (concept_id::Int, was_correct::Bool) in chronological order
`concept_names`: mapping concept_id → human-readable name
"""
function predict_knowledge_state(
    model::DKTModel,
    student_history::Vector{Tuple{Int,Bool}},
    concept_names::Dict{Int, String};
    mastery_threshold::Float64 = 0.70,
)
    isempty(student_history) && return nothing

    tokens = [c * 2 + Int(correct) for (c, correct) in student_history]
    Flux.reset!(model.lstm)
    preds = model(tokens)

    # Take the last column — current knowledge state
    current_state = preds[:, end]

    masteries = Dict{String, Float64}()
    for (id, name) in concept_names
        id <= size(current_state, 1) || continue
        masteries[name] = Float64(current_state[id])
    end

    flagged = [name for (name, p) in masteries if p < mastery_threshold]
    predicted_perf = mean(values(masteries))
    weakest = isempty(flagged) ? nothing : argmin(Dict(n => masteries[n] for n in flagged))

    KnowledgeState(
        "",  # student_id filled by caller
        masteries,
        flagged,
        predicted_perf,
        weakest,
        flagged[1:min(3, length(flagged))],  # top 3 to practice
        time(),
    )
end

# ─── Forgetting Curve ──────────────────────────────────────────────────────────

"""
    forgetting_curve_adjustment(initial_mastery, decay_rate, days_since_last_interaction)

Ebbinghaus forgetting curve: m(t) = m₀ × e^(-k × t)

Returns adjusted mastery probability after accounting for time decay.
"""
function forgetting_curve_adjustment(
    initial_mastery::Float64,
    days_since_last_interaction::Int;
    decay_rate::Float64 = 0.05,
)
    decayed = initial_mastery * exp(-decay_rate * days_since_last_interaction)
    max(decayed, 0.0)
end

"""
    concepts_needing_refresh(knowledge_state, last_interactions, threshold)

Returns concept names that have decayed below threshold due to inactivity.
Used to surface "memory refresh" micro-sessions before exams.
"""
function concepts_needing_refresh(
    masteries::Dict{String, Float64},
    last_interactions::Dict{String, Int},  # concept_name → days_since
    threshold::Float64 = 0.60;
    decay_rate::Float64 = 0.05,
)
    refresh_needed = String[]
    for (name, mastery) in masteries
        days = get(last_interactions, name, 14)
        adjusted = forgetting_curve_adjustment(mastery, days; decay_rate)
        adjusted < threshold && push!(refresh_needed, name)
    end
    refresh_needed
end

# ─── Model persistence ─────────────────────────────────────────────────────────

function save_model(model::DKTModel, path::String)
    using BSON: @save
    @save path model
end

function load_model(path::String)::DKTModel
    using BSON: @load
    @load path model
    model
end

end # module DKT
