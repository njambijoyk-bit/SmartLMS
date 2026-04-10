# SmartLMS ML Engine - Main Module
# Provides adaptive learning capabilities

module SmartLMSML

using Pkg

# Export main functions
export train_dropout_model, predict_dropout
export train_recommendation_model, get_recommendations
export train_performance_model, predict_performance

# Include submodules
include("models/dropout.jl")
include("models/recommendation.jl")
include("models/performance.jl")

end # module