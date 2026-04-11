use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a chunk of course content indexed for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub id: String,
    pub course_id: String,
    pub module_id: Option<String>,
    pub content_type: ContentType,
    pub title: String,
    pub text: String,
    pub embedding: Vec<f32>,
    pub metadata: ChunkMetadata,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    LectureNotes,
    VideoTranscript,
    Assignment,
    Quiz,
    Reading,
    Discussion,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChunkMetadata {
    pub page_number: Option<i32>,
    pub timestamp: Option<String>, // For video transcripts (e.g., "05:32")
    pub source_url: Option<String>,
    pub tags: Vec<String>,
}

/// Query request for the AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantQuery {
    pub user_id: String,
    pub course_id: String,
    pub question: String,
    pub conversation_history: Vec<Message>,
    pub context_limit: Option<usize>,
}

/// Response from the AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantResponse {
    pub answer: String,
    pub sources: Vec<SourceReference>,
    pub confidence_score: f32,
    pub suggested_followups: Vec<String>,
    pub conversation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReference {
    pub chunk_id: String,
    pub title: String,
    pub content_type: ContentType,
    pub excerpt: String,
    pub relevance_score: f32,
    pub module_id: Option<String>,
}

/// Chat message in conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

impl Message {
    pub fn user(content: String) -> Self {
        Self {
            role: MessageRole::User,
            content,
            timestamp: Utc::now(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: MessageRole::Assistant,
            content,
            timestamp: Utc::now(),
        }
    }

    pub fn system(content: String) -> Self {
        Self {
            role: MessageRole::System,
            content,
            timestamp: Utc::now(),
        }
    }
}

/// Study recommendation for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyRecommendation {
    pub user_id: String,
    pub course_id: String,
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub estimated_time_minutes: u32,
    pub related_content_ids: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationType {
    ReviewTopic,
    CompleteAssignment,
    WatchVideo,
    PracticeQuiz,
    ReadMaterial,
    JoinDiscussion,
    CatchUp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Performance metrics for a student in a course
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudentPerformance {
    pub user_id: String,
    pub course_id: String,
    pub overall_score: f32,
    pub completion_rate: f32,
    pub time_spent_minutes: u32,
    quiz_scores: HashMap<String, f32>, // quiz_id -> score
    assignment_scores: HashMap<String, f32>, // assignment_id -> score
    weak_topics: Vec<TopicPerformance>,
    strong_topics: Vec<TopicPerformance>,
    last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicPerformance {
    pub topic_name: String,
    pub mastery_score: f32,
    pub question_count: u32,
}

/// Configuration for the AI assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantConfig {
    pub model_name: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub context_window_size: usize,
    pub embedding_model: String,
    pub similarity_threshold: f32,
    pub max_sources: usize,
    pub system_prompt: String,
}

impl Default for AssistantConfig {
    fn default() -> Self {
        Self {
            model_name: "gpt-4o-mini".to_string(),
            max_tokens: 1024,
            temperature: 0.7,
            top_p: 0.9,
            context_window_size: 4,
            embedding_model: "text-embedding-3-small".to_string(),
            similarity_threshold: 0.6,
            max_sources: 5,
            system_prompt: r#"You are an intelligent learning assistant for SmartLMS. 
Your role is to help students understand course materials, clarify concepts, and guide their learning journey.

Guidelines:
- Base your answers primarily on the provided course context
- If the context doesn't contain enough information, acknowledge it and provide general guidance
- Break down complex concepts into simpler explanations
- Use examples when helpful
- Encourage critical thinking rather than just giving answers
- Be supportive and motivating
- Cite your sources when referencing specific course materials
- If asked about deadlines or grades, direct students to check their dashboard

Remember: Your goal is to facilitate learning, not replace it."#.to_string(),
        }
    }
}

/// Service trait for AI-powered learning assistance
#[async_trait::async_trait]
pub trait AiLearningService {
    /// Generate embeddings for content chunks
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, AiError>;
    
    /// Index course content for semantic search
    async fn index_course_content(
        &self,
        course_id: &str,
        chunks: Vec<ContentChunk>,
    ) -> Result<usize, AiError>;
    
    /// Search for relevant content based on a query
    async fn search_relevant_content(
        &self,
        course_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<ContentChunk>, AiError>;
    
    /// Process a student question and generate a response
    async fn answer_question(
        &self,
        query: AssistantQuery,
    ) -> Result<AssistantResponse, AiError>;
    
    /// Generate personalized study recommendations
    async fn generate_recommendations(
        &self,
        user_id: &str,
        course_id: &str,
        performance: &StudentPerformance,
    ) -> Result<Vec<StudyRecommendation>, AiError>;
    
    /// Analyze student performance and identify weak areas
    async fn analyze_performance(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<StudentPerformance, AiError>;
    
    /// Summarize a course module
    async fn summarize_module(
        &self,
        course_id: &str,
        module_id: &str,
    ) -> Result<String, AiError>;
    
    /// Generate practice questions based on course content
    async fn generate_practice_questions(
        &self,
        course_id: &str,
        topic: &str,
        count: usize,
        difficulty: Difficulty,
    ) -> Result<Vec<PracticeQuestion>, AiError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeQuestion {
    pub id: String,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<String>>, // For multiple choice
    pub correct_answer: String,
    pub explanation: String,
    pub difficulty: Difficulty,
    pub topic: String,
    pub source_chunk_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// Error types for AI service operations
#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("Embedding generation failed: {0}")]
    EmbeddingError(String),
    
    #[error("LLM API error: {0}")]
    LlmError(String),
    
    #[error("Vector database error: {0}")]
    VectorDbError(String),
    
    #[error("Content not found: {0}")]
    ContentNotFound(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<AiError> for crate::api::ApiError {
    fn from(err: AiError) -> Self {
        match err {
            AiError::EmbeddingError(msg) => crate::api::ApiError::BadRequest(msg),
            AiError::LlmError(msg) => crate::api::ApiError::ExternalServiceError(msg),
            AiError::VectorDbError(msg) => crate::api::ApiError::DatabaseError(msg),
            AiError::ContentNotFound(msg) => crate::api::ApiError::NotFound(msg),
            AiError::ConfigError(msg) => crate::api::ApiError::BadRequest(msg),
            AiError::RateLimitExceeded => crate::api::ApiError::RateLimitExceeded,
            AiError::AuthError(msg) => crate::api::ApiError::Unauthorized(msg),
            AiError::InternalError(msg) => crate::api::ApiError::InternalServerError(msg),
        }
    }
}

/// In-memory store for content chunks (production should use vector DB like Qdrant/Weaviate)
pub struct ContentStore {
    chunks: HashMap<String, ContentChunk>,
    course_index: HashMap<String, Vec<String>>, // course_id -> chunk_ids
}

impl ContentStore {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            course_index: HashMap::new(),
        }
    }
    
    pub fn add_chunk(&mut self, chunk: ContentChunk) {
        let course_id = chunk.course_id.clone();
        let chunk_id = chunk.id.clone();
        
        self.chunks.insert(chunk_id.clone(), chunk);
        self.course_index
            .entry(course_id)
            .or_insert_with(Vec::new)
            .push(chunk_id);
    }
    
    pub fn get_chunk(&self, chunk_id: &str) -> Option<&ContentChunk> {
        self.chunks.get(chunk_id)
    }
    
    pub fn get_course_chunks(&self, course_id: &str) -> Vec<&ContentChunk> {
        self.course_index
            .get(course_id)
            .map(|ids| ids.iter().filter_map(|id| self.chunks.get(id)).collect())
            .unwrap_or_default()
    }
    
    /// Simple cosine similarity search (production should use proper vector DB)
    pub fn similarity_search(
        &self,
        course_id: &str,
        query_embedding: &[f32],
        limit: usize,
        threshold: f32,
    ) -> Vec<(&ContentChunk, f32)> {
        let course_chunks = self.get_course_chunks(course_id);
        
        let mut scores: Vec<(&ContentChunk, f32)> = course_chunks
            .into_iter()
            .filter_map(|chunk| {
                let similarity = cosine_similarity(query_embedding, &chunk.embedding);
                if similarity >= threshold {
                    Some((chunk, similarity))
                } else {
                    None
                }
            })
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.into_iter().take(limit).collect()
    }
}

impl Default for ContentStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate cosine similarity between two vectors
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (magnitude_a * magnitude_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity - 1.0).abs() < 0.0001);
    }
    
    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!(similarity.abs() < 0.0001);
    }
    
    #[test]
    fn test_message_creation() {
        let msg = Message::user("What is photosynthesis?".to_string());
        assert_eq!(msg.role, MessageRole::User);
        assert!(msg.content.contains("photosynthesis"));
    }
}
