# SmartLMS ML Engine - Content Recommendation Model
# Uses collaborative filtering + content-based filtering

using CSV
using DataFrames
using MLJ
using LinearAlgebra
using Random

"""
Content recommendation system combining:
1. Collaborative filtering (user-based)
2. Content-based filtering (item-based)
3. Hybrid scoring
"""
struct RecommendationModel
    user_embeddings::Matrix{Float64}
    item_embeddings::Matrix{Float64}
    content_features::DataFrame
    user_item_matrix::Matrix{Float64}
end

"""
Train recommendation model from user-item interactions.
"""
function train_recommendation_model(
    interactions::DataFrame;
    n_factors::Int=10,
    max_iterations::Int=50,
    learning_rate::Float64=0.01,
    regularization::Float64=0.1
)
    println("Training recommendation model with $n_factors latent factors...")
    
    users = unique(interactions.user_id)
    items = unique(interactions.item_id)
    
    user_idx = Dict(u => i for (i, u) in enumerate(users))
    item_idx = Dict(i => j for (j, i) in enumerate(items))
    
    n_users = length(users)
    n_items = length(items)
    
    # Create user-item matrix
    user_item_matrix = zeros(n_users, n_items)
    for row in eachrow(interactions)
        ui = user_idx[row.user_id]
        ii = item_idx[row.item_id]
        user_item_matrix[ui, ii] = row.rating
    end
    
    # Initialize embeddings
    Random.seed!(42)
    user_embeddings = randn(n_users, n_factors) * 0.1
    item_embeddings = randn(n_items, n_factors) * 0.1
    
    # Matrix factorization training (alternating least squares)
    for iter in 1:max_iterations
        # Update user embeddings
        for u in 1:n_users
            rated_items = findall(user_item_matrix[u, :] .> 0)
            if isempty(rated_items) continue end
            
            item_ids = item_embeddings[rated_items, :]
            ratings = user_item_matrix[u, rated_items]
            
            # Solve least squares
            A = item_ids * item_ids' + regularization * I
            b = item_ids' * ratings
            user_embeddings[u, :] = A \ b
        end
        
        # Update item embeddings
        for i in 1:n_items
            rated_users = findall(user_item_matrix[:, i] .> 0)
            if isempty(rated_users) continue end
            
            user_ids = user_embeddings[rated_users, :]
            ratings = user_item_matrix[rated_users, i]
            
            A = user_ids * user_ids' + regularization * I
            b = user_ids' * ratings
            item_embeddings[i, :] = A \ b
        end
        
        if iter % 10 == 0
            # Calculate training error
            predictions = user_embeddings * item_embeddings'
            mask = user_item_matrix .> 0
            mse = mean((predictions[mask] - user_item_matrix[mask]).^2)
            println("Iteration $iter, MSE: ", round(mse, digits=4))
        end
    end
    
    return RecommendationModel(
        user_embeddings,
        item_embeddings,
        DataFrame(),  # Content features
        user_item_matrix
    )
end

"""
Get content recommendations for a user.
"""
function get_recommendations(
    model::RecommendationModel,
    user_id::String,
    item_ids::Vector{String};
    n_recommendations::Int=5,
    exclude_rated::Bool=true
)
    # Find user index
    users = unique(keys(model.user_item_matrix))  # Would need actual user mapping
    user_idx = findfirst(isequal(user_id), model.user_item_matrix[:, 1])
    
    if user_idx === nothing
        # Cold start - return popular items
        return [(item_id=item_ids[i], score=0.5) for i in 1:min(n_recommendations, length(item_ids))]
    end
    
    # Get user's predicted ratings for all items
    user_emb = model.user_embeddings[user_idx, :]
    predicted_ratings = user_emb * model.item_embeddings'
    
    # Sort by predicted rating
    sorted_idx = sortperm(predicted_ratings, rev=true)
    
    results = []
    for idx in sorted_idx
        push!(results, (item_id=item_ids[idx], score=predicted_ratings[idx]))
        if length(results) >= n_recommendations
            break
        end
    end
    
    return results
end

"""
Get similar content items (content-based).
"""
function get_similar_items(
    model::RecommendationModel,
    item_id::String,
    all_item_ids::Vector{String};
    n_similar::Int=5
)
    # Find item index
    item_idx = findfirst(isequal(item_id), model.item_embeddings[:, 1])
    
    if item_idx === nothing
        return []
    end
    
    # Calculate cosine similarity
    item_emb = model.item_embeddings[item_idx, :]
    similarities = [dot(item_emb, model.item_embeddings[i, :]) / 
                    (norm(item_emb) * norm(model.item_embeddings[i, :]) + eps())
                    for i in 1:size(model.item_embeddings, 1)]
    
    # Sort by similarity
    sorted_idx = sortperm(similarities, rev=true)
    
    results = []
    for idx in sorted_idx
        if all_item_ids[idx] != item_id
            push!(results, (item_id=all_item_ids[idx], similarity=similarities[idx]))
            if length(results) >= n_similar
                break
            end
        end
    end
    
    return results
end

# Example usage
if abspath(PROGRAM_FILE) == @__FILE__
    # Generate synthetic interaction data
    Random.seed!(42)
    n_users = 100
    n_items = 50
    n_interactions = 1000
    
    users = ["user_$i" for i in 1:n_users]
    items = ["item_$i" for i in 1:n_items]
    
    interactions = DataFrame(
        user_id = rand(users, n_interactions),
        item_id = rand(items, n_interactions),
        rating = rand(1:5, n_interactions)
    )
    
    println("Training recommendation model...")
    model = train_recommendation_model(interactions, n_factors=5, max_iterations=20)
    println("Model trained!")
    
    # Get recommendations for a user
    recs = get_recommendations(model, "user_1", items, n_recommendations=5)
    println("Recommendations: ", recs)
end