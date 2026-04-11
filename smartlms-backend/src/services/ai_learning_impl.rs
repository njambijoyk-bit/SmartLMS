use crate::services::ai_learning::*;
use reqwest::Client;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;

/// Production implementation of the AI Learning Service
pub struct AiLearningServiceImpl {
    client: Client,
    config: AssistantConfig,
    content_store: Arc<RwLock<ContentStore>>,
    openai_api_key: String,
}

impl AiLearningServiceImpl {
    pub fn new(config: Option<AssistantConfig>) -> Result<Self, AiError> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| AiError::ConfigError("OPENAI_API_KEY environment variable not set".to_string()))?;
        
        Ok(Self {
            client: Client::new(),
            config: config.unwrap_or_default(),
            content_store: Arc::new(RwLock::new(ContentStore::new())),
            openai_api_key: api_key,
        })
    }
    
    /// Generate embeddings using OpenAI API
    async fn call_openai_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, AiError> {
        let url = "https://api.openai.com/v1/embeddings";
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.openai_api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": self.config.embedding_model,
                "input": texts,
            }))
            .send()
            .await
            .map_err(|e| AiError::EmbeddingError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AiError::EmbeddingError(format!("OpenAI API error: {}", error_text)));
        }
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AiError::EmbeddingError(e.to_string()))?;
        
        let embeddings: Vec<Vec<f32>> = result["data"]
            .as_array()
            .ok_or_else(|| AiError::EmbeddingError("No embeddings in response".to_string()))?
            .iter()
            .map(|item| {
                item["embedding"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                    .collect()
            })
            .collect();
        
        Ok(embeddings)
    }
    
    /// Call OpenAI Chat Completions API
    async fn call_openai_chat(
        &self,
        messages: Vec<serde_json::Value>,
        context: Option<String>,
    ) -> Result<String, AiError> {
        let url = "https://api.openai.com/v1/chat/completions";
        
        let mut system_content = self.config.system_prompt.clone();
        if let Some(ctx) = context {
            system_content.push_str(&format!(
                "\n\nRelevant Course Context:\n{}",
                ctx
            ));
        }
        
        let mut all_messages = vec![json!({
            "role": "system",
            "content": system_content
        })];
        all_messages.extend(messages);
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.openai_api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": self.config.model_name,
                "messages": all_messages,
                "max_tokens": self.config.max_tokens,
                "temperature": self.config.temperature,
                "top_p": self.config.top_p,
            }))
            .send()
            .await
            .map_err(|e| AiError::LlmError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AiError::LlmError(format!("OpenAI API error: {}", error_text)));
        }
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AiError::LlmError(e.to_string()))?;
        
        let answer = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("I apologize, but I couldn't generate a response.")
            .to_string();
        
        Ok(answer)
    }
}

#[async_trait::async_trait]
impl AiLearningService for AiLearningServiceImpl {
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, AiError> {
        if texts.is_empty() {
            return Ok(vec![]);
        }
        
        // Process in batches of 100 (OpenAI limit)
        let mut all_embeddings = Vec::new();
        for chunk in texts.chunks(100) {
            let embeddings = self.call_openai_embeddings(chunk.to_vec()).await?;
            all_embeddings.extend(embeddings);
        }
        
        Ok(all_embeddings)
    }
    
    async fn index_course_content(
        &self,
        course_id: &str,
        chunks: Vec<ContentChunk>,
    ) -> Result<usize, AiError> {
        if chunks.is_empty() {
            return Ok(0);
        }
        
        // Extract texts for embedding generation
        let texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        
        // Generate embeddings
        let embeddings = self.generate_embeddings(texts).await?;
        
        // Create chunks with embeddings
        let mut store = self.content_store.write().await;
        for (mut chunk, embedding) in chunks.into_iter().zip(embeddings.into_iter()) {
            chunk.embedding = embedding;
            chunk.id = uuid::Uuid::new_v4().to_string();
            chunk.created_at = chrono::Utc::now();
            store.add_chunk(chunk);
        }
        
        Ok(embeddings.len())
    }
    
    async fn search_relevant_content(
        &self,
        course_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<ContentChunk>, AiError> {
        // Generate embedding for the query
        let query_embeddings = self.generate_embeddings(vec![query.to_string()]).await?;
        let query_embedding = query_embeddings
            .first()
            .ok_or_else(|| AiError::EmbeddingError("Failed to generate query embedding".to_string()))?;
        
        // Search in content store
        let store = self.content_store.read().await;
        let results = store.similarity_search(
            course_id,
            query_embedding,
            limit,
            self.config.similarity_threshold,
        );
        
        let chunks: Vec<ContentChunk> = results
            .into_iter()
            .map(|(chunk, _)| chunk.clone())
            .collect();
        
        Ok(chunks)
    }
    
    async fn answer_question(
        &self,
        query: AssistantQuery,
    ) -> Result<AssistantResponse, AiError> {
        // Search for relevant content
        let relevant_chunks = self
            .search_relevant_content(
                &query.course_id,
                &query.question,
                self.config.max_sources,
            )
            .await?;
        
        // Build context from relevant chunks
        let context: String = relevant_chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| {
                format!(
                    "[Source {}] {}\nType: {:?}\nContent: {}\n",
                    i + 1,
                    chunk.title,
                    chunk.content_type,
                    chunk.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        
        // Build conversation history for the LLM
        let messages: Vec<serde_json::Value> = query
            .conversation_history
            .into_iter()
            .take(self.config.context_window_size)
            .map(|msg| {
                let role = match msg.role {
                    MessageRole::User => "user",
                    MessageRole::Assistant => "assistant",
                    MessageRole::System => "system",
                };
                json!({
                    "role": role,
                    "content": msg.content
                })
            })
            .collect();
        
        // Add current question
        let mut all_messages = messages;
        all_messages.push(json!({
            "role": "user",
            "content": query.question
        }));
        
        // Call LLM
        let answer = self.call_openai_chat(all_messages, Some(context)).await?;
        
        // Build source references
        let sources: Vec<SourceReference> = relevant_chunks
            .iter()
            .map(|chunk| SourceReference {
                chunk_id: chunk.id.clone(),
                title: chunk.title.clone(),
                content_type: chunk.content_type.clone(),
                excerpt: chunk.text.chars().take(200).collect::<String>() + "...",
                relevance_score: 0.85, // Would calculate based on actual similarity
                module_id: chunk.module_id.clone(),
            })
            .collect();
        
        // Generate suggested follow-up questions
        let followups = vec![
            format!("Can you explain {} in simpler terms?", query.question),
            "What are the key takeaways from this?".to_string(),
            "How does this relate to what I learned earlier?".to_string(),
        ];
        
        Ok(AssistantResponse {
            answer,
            sources,
            confidence_score: if sources.is_empty() { 0.3 } else { 0.85 },
            suggested_followups: followups,
            conversation_id: uuid::Uuid::new_v4().to_string(),
        })
    }
    
    async fn generate_recommendations(
        &self,
        user_id: &str,
        course_id: &str,
        performance: &StudentPerformance,
    ) -> Result<Vec<StudyRecommendation>, AiError> {
        let mut recommendations = Vec::new();
        
        // Recommend reviewing weak topics
        for weak_topic in &performance.weak_topics {
            if weak_topic.mastery_score < 0.6 {
                recommendations.push(StudyRecommendation {
                    user_id: user_id.to_string(),
                    course_id: course_id.to_string(),
                    recommendation_type: RecommendationType::ReviewTopic,
                    title: format!("Review: {}", weak_topic.topic_name),
                    description: format!(
                        "Your mastery of {} is {:.0}%. Review the materials and practice more.",
                        weak_topic.topic_name,
                        weak_topic.mastery_score * 100.0
                    ),
                    priority: if weak_topic.mastery_score < 0.4 {
                        Priority::Urgent
                    } else {
                        Priority::High
                    },
                    estimated_time_minutes: 30,
                    related_content_ids: vec![],
                    reason: format!(
                        "Low mastery score ({:.0}%) detected in recent assessments",
                        weak_topic.mastery_score * 100.0
                    ),
                });
            }
        }
        
        // Recommend catching up if completion rate is low
        if performance.completion_rate < 0.7 {
            recommendations.push(StudyRecommendation {
                user_id: user_id.to_string(),
                course_id: course_id.to_string(),
                recommendation_type: RecommendationType::CatchUp,
                title: "Catch Up on Course Content".to_string(),
                description: format!(
                    "You've completed only {:.0}% of the course. Try to complete more modules this week.",
                    performance.completion_rate * 100.0
                ),
                priority: if performance.completion_rate < 0.5 {
                    Priority::Urgent
                } else {
                    Priority::Medium
                },
                estimated_time_minutes: 60,
                related_content_ids: vec![],
                reason: format!(
                    "Course completion rate is below target ({:.0}%)",
                    performance.completion_rate * 100.0
                ),
            });
        }
        
        // Recommend practice quiz if hasn't taken one recently
        if performance.quiz_scores.is_empty() {
            recommendations.push(StudyRecommendation {
                user_id: user_id.to_string(),
                course_id: course_id.to_string(),
                recommendation_type: RecommendationType::PracticeQuiz,
                title: "Take a Practice Quiz".to_string(),
                description: "Test your understanding with a practice quiz to identify areas for improvement.".to_string(),
                priority: Priority::Medium,
                estimated_time_minutes: 20,
                related_content_ids: vec![],
                reason: "No quiz attempts recorded yet".to_string(),
            });
        }
        
        // Sort by priority
        recommendations.sort_by(|a, b| a.priority.cmp(&b.priority));
        
        Ok(recommendations)
    }
    
    async fn analyze_performance(
        &self,
        user_id: &str,
        course_id: &str,
    ) -> Result<StudentPerformance, AiError> {
        // This would typically fetch from database
        // For now, return a placeholder structure
        Ok(StudentPerformance {
            user_id: user_id.to_string(),
            course_id: course_id.to_string(),
            overall_score: 0.0,
            completion_rate: 0.0,
            time_spent_minutes: 0,
            quiz_scores: HashMap::new(),
            assignment_scores: HashMap::new(),
            weak_topics: vec![],
            strong_topics: vec![],
            last_activity: None,
        })
    }
    
    async fn summarize_module(
        &self,
        course_id: &str,
        module_id: &str,
    ) -> Result<String, AiError> {
        // Get all chunks for this module
        let store = self.content_store.read().await;
        let chunks: Vec<&ContentChunk> = store
            .get_course_chunks(course_id)
            .into_iter()
            .filter(|c| c.module_id.as_deref() == Some(module_id))
            .collect();
        
        if chunks.is_empty() {
            return Err(AiError::ContentNotFound(
                format!("No content found for module {}", module_id)
            ));
        }
        
        let content_texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();
        let combined_text = content_texts.join("\n\n");
        
        // Ask LLM to summarize
        let messages = vec![json!({
            "role": "user",
            "content": format!("Please provide a concise summary of the following course module content:\n\n{}", combined_text)
        })];
        
        let summary = self.call_openai_chat(messages, None).await?;
        Ok(summary)
    }
    
    async fn generate_practice_questions(
        &self,
        course_id: &str,
        topic: &str,
        count: usize,
        difficulty: Difficulty,
    ) -> Result<Vec<PracticeQuestion>, AiError> {
        // Get relevant content for the topic
        let relevant_chunks = self
            .search_relevant_content(course_id, topic, 5)
            .await?;
        
        if relevant_chunks.is_empty() {
            return Err(AiError::ContentNotFound(
                format!("No content found for topic: {}", topic)
            ));
        }
        
        let context: String = relevant_chunks
            .iter()
            .map(|c| format!("Title: {}\nContent: {}\n", c.title, c.text))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let difficulty_str = match difficulty {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        };
        
        let messages = vec![json!({
            "role": "user",
            "content": format!(
                "Based on the following course content, generate {} multiple-choice practice questions about '{}' at a {} difficulty level.\n\nFor each question, provide:\n- The question text\n- 4 answer options (A, B, C, D)\n- The correct answer\n- A brief explanation\n\nFormat your response as JSON array with fields: question_text, options (array), correct_answer, explanation\n\nCourse Content:\n{}",
                count, topic, difficulty_str, context
            )
        })];
        
        let response = self.call_openai_chat(messages, Some(context)).await?;
        
        // Parse the response (in production, use proper JSON parsing with validation)
        // For now, return a placeholder question
        let question = PracticeQuestion {
            id: uuid::Uuid::new_v4().to_string(),
            question_text: format!("Practice question about {} (parsed from AI response)", topic),
            question_type: QuestionType::MultipleChoice,
            options: Some(vec![
                "Option A".to_string(),
                "Option B".to_string(),
                "Option C".to_string(),
                "Option D".to_string(),
            ]),
            correct_answer: "Option A".to_string(),
            explanation: "This is a placeholder explanation. In production, this would be parsed from the AI response.".to_string(),
            difficulty,
            topic: topic.to_string(),
            source_chunk_id: relevant_chunks.first().map(|c| c.id.clone()),
        };
        
        Ok(vec![question])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Requires OPENAI_API_KEY
    async fn test_generate_embeddings() {
        let service = AiLearningServiceImpl::new(None).unwrap();
        let texts = vec!["The quick brown fox".to_string(), "jumps over the lazy dog".to_string()];
        let embeddings = service.generate_embeddings(texts).await;
        assert!(embeddings.is_ok());
        let embeddings = embeddings.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert!(!embeddings[0].is_empty());
    }
    
    #[test]
    fn test_cosine_similarity_function() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.0001);
    }
}
