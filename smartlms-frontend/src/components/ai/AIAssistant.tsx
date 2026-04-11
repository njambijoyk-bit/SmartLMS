import React, { useState, useRef, useEffect } from 'react';
import { Send, BookOpen, Lightbulb, Clock, TrendingUp, AlertCircle, CheckCircle, MessageSquare } from 'lucide-react';

interface Message {
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
}

interface SourceReference {
  chunk_id: string;
  title: string;
  content_type: string;
  excerpt: string;
  relevance_score: number;
  module_id?: string;
}

interface StudyRecommendation {
  user_id: string;
  course_id: string;
  recommendation_type: string;
  title: string;
  description: string;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  estimated_time_minutes: number;
  related_content_ids: string[];
  reason: string;
}

interface AssistantResponse {
  answer: string;
  sources: SourceReference[];
  confidence_score: number;
  suggested_followups: string[];
  conversation_id: string;
}

interface AIAssistantProps {
  courseId: string;
  userId: string;
}

const AIAssistant: React.FC<AIAssistantProps> = ({ courseId, userId }) => {
  const [messages, setMessages] = useState<Message[]>([
    {
      role: 'assistant',
      content: "Hi! I'm your AI learning assistant. I can help you understand course materials, clarify concepts, and guide your learning journey. What would you like to know?",
      timestamp: new Date().toISOString(),
    },
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [showSources, setShowSources] = useState(false);
  const [currentSources, setCurrentSources] = useState<SourceReference[]>([]);
  const [showRecommendations, setShowRecommendations] = useState(false);
  const [recommendations, setRecommendations] = useState<StudyRecommendation[]>([]);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const sendMessage = async (question: string) => {
    if (!question.trim()) return;

    const userMessage: Message = {
      role: 'user',
      content: question,
      timestamp: new Date().toISOString(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInputValue('');
    setIsLoading(true);

    try {
      const response = await fetch('/api/ai/ask', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          course_id: courseId,
          question: question,
          conversation_history: messages.slice(-4), // Last 4 messages for context
        }),
      });

      if (!response.ok) throw new Error('Failed to get response');

      const result = await response.json();
      
      if (result.success) {
        const assistantMessage: Message = {
          role: 'assistant',
          content: result.data.answer,
          timestamp: new Date().toISOString(),
        };
        
        setMessages((prev) => [...prev, assistantMessage]);
        setCurrentSources(result.data.sources || []);
        setShowSources(result.data.sources && result.data.sources.length > 0);
      } else {
        throw new Error(result.error || 'Something went wrong');
      }
    } catch (error) {
      const errorMessage: Message = {
        role: 'assistant',
        content: "I apologize, but I'm having trouble connecting right now. Please try again in a moment.",
        timestamp: new Date().toISOString(),
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    sendMessage(inputValue);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage(inputValue);
    }
  };

  const loadRecommendations = async () => {
    try {
      const response = await fetch('/api/ai/recommendations', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ course_id: courseId }),
      });

      if (!response.ok) throw new Error('Failed to load recommendations');

      const result = await response.json();
      
      if (result.success) {
        setRecommendations(result.data);
        setShowRecommendations(true);
      }
    } catch (error) {
      console.error('Error loading recommendations:', error);
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'urgent': return 'bg-red-100 text-red-800 border-red-300';
      case 'high': return 'bg-orange-100 text-orange-800 border-orange-300';
      case 'medium': return 'bg-yellow-100 text-yellow-800 border-yellow-300';
      case 'low': return 'bg-green-100 text-green-800 border-green-300';
      default: return 'bg-gray-100 text-gray-800 border-gray-300';
    }
  };

  const getRecommendationIcon = (type: string) => {
    switch (type) {
      case 'ReviewTopic': return <AlertCircle className="w-5 h-5" />;
      case 'PracticeQuiz': return <CheckCircle className="w-5 h-5" />;
      case 'WatchVideo': return <BookOpen className="w-5 h-5" />;
      case 'CatchUp': return <Clock className="w-5 h-5" />;
      default: return <Lightbulb className="w-5 h-5" />;
    }
  };

  return (
    <div className="flex flex-col h-full bg-white rounded-lg shadow-lg">
      {/* Header */}
      <div className="p-4 border-b border-gray-200 bg-gradient-to-r from-blue-600 to-indigo-600 rounded-t-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-3">
            <div className="p-2 bg-white/20 rounded-lg">
              <MessageSquare className="w-6 h-6 text-white" />
            </div>
            <div>
              <h2 className="text-xl font-bold text-white">AI Learning Assistant</h2>
              <p className="text-sm text-blue-100">Your 24/7 study companion</p>
            </div>
          </div>
          <button
            onClick={loadRecommendations}
            className="flex items-center space-x-2 px-3 py-2 bg-white/20 hover:bg-white/30 rounded-lg transition-colors"
          >
            <TrendingUp className="w-4 h-4 text-white" />
            <span className="text-sm text-white">My Recommendations</span>
          </button>
        </div>
      </div>

      {/* Recommendations Panel */}
      {showRecommendations && (
        <div className="p-4 bg-blue-50 border-b border-blue-200">
          <div className="flex items-center justify-between mb-3">
            <h3 className="font-semibold text-blue-900">Personalized Study Recommendations</h3>
            <button
              onClick={() => setShowRecommendations(false)}
              className="text-blue-600 hover:text-blue-800"
            >
              ✕
            </button>
          </div>
          <div className="space-y-2 max-h-48 overflow-y-auto">
            {recommendations.map((rec, index) => (
              <div key={index} className="p-3 bg-white rounded-lg border border-blue-200">
                <div className="flex items-start space-x-2">
                  <span className={`mt-1 ${getPriorityColor(rec.priority).replace('bg-', 'text-').split(' ')[1]}`}>
                    {getRecommendationIcon(rec.recommendation_type)}
                  </span>
                  <div className="flex-1">
                    <div className="flex items-center space-x-2">
                      <h4 className="font-medium text-gray-900">{rec.title}</h4>
                      <span className={`px-2 py-0.5 text-xs rounded-full border ${getPriorityColor(rec.priority)}`}>
                        {rec.priority.toUpperCase()}
                      </span>
                    </div>
                    <p className="text-sm text-gray-600 mt-1">{rec.description}</p>
                    <div className="flex items-center space-x-4 mt-2 text-xs text-gray-500">
                      <span className="flex items-center">
                        <Clock className="w-3 h-3 mr-1" />
                        {rec.estimated_time_minutes} min
                      </span>
                      <span>{rec.reason}</span>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Messages Container */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50">
        {messages.map((message, index) => (
          <div
            key={index}
            className={`flex ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[80%] rounded-2xl px-4 py-3 ${
                message.role === 'user'
                  ? 'bg-blue-600 text-white'
                  : 'bg-white text-gray-800 shadow-md'
              }`}
            >
              <p className="whitespace-pre-wrap">{message.content}</p>
              <p className={`text-xs mt-2 ${message.role === 'user' ? 'text-blue-100' : 'text-gray-400'}`}>
                {new Date(message.timestamp).toLocaleTimeString()}
              </p>
            </div>
          </div>
        ))}
        
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-white rounded-2xl px-4 py-3 shadow-md">
              <div className="flex space-x-2">
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
              </div>
            </div>
          </div>
        )}
        
        <div ref={messagesEndRef} />
      </div>

      {/* Sources Panel */}
      {showSources && currentSources.length > 0 && (
        <div className="p-4 bg-gray-50 border-t border-gray-200">
          <div className="flex items-center justify-between mb-2">
            <h4 className="text-sm font-semibold text-gray-700 flex items-center">
              <BookOpen className="w-4 h-4 mr-2" />
              Sources ({currentSources.length})
            </h4>
            <button
              onClick={() => setShowSources(false)}
              className="text-gray-500 hover:text-gray-700 text-sm"
            >
              Hide
            </button>
          </div>
          <div className="space-y-2 max-h-32 overflow-y-auto">
            {currentSources.map((source, index) => (
              <div key={source.chunk_id} className="p-2 bg-white rounded border border-gray-200 text-sm">
                <div className="flex items-center justify-between">
                  <span className="font-medium text-blue-600">{source.title}</span>
                  <span className="text-xs text-gray-500">{(source.relevance_score * 100).toFixed(0)}% match</span>
                </div>
                <p className="text-gray-600 mt-1 line-clamp-2">{source.excerpt}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Input Area */}
      <div className="p-4 border-t border-gray-200 bg-white rounded-b-lg">
        {/* Suggested Follow-ups */}
        {messages.length > 1 && !isLoading && (
          <div className="mb-3">
            <p className="text-xs text-gray-500 mb-2">Suggested follow-ups:</p>
            <div className="flex flex-wrap gap-2">
              {['Can you explain this in simpler terms?', 'What are the key takeaways?', 'How does this relate to earlier topics?'].map((suggestion, index) => (
                <button
                  key={index}
                  onClick={() => sendMessage(suggestion)}
                  className="px-3 py-1.5 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-full transition-colors"
                >
                  {suggestion}
                </button>
              ))}
            </div>
          </div>
        )}
        
        <form onSubmit={handleSubmit} className="flex space-x-2">
          <input
            ref={inputRef}
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Ask anything about your course..."
            className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            disabled={isLoading}
          />
          <button
            type="submit"
            disabled={isLoading || !inputValue.trim()}
            className="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors flex items-center space-x-2"
          >
            <Send className="w-4 h-4" />
            <span>Send</span>
          </button>
        </form>
        <p className="text-xs text-gray-400 mt-2 text-center">
          Press Enter to send • AI responses are based on your course materials
        </p>
      </div>
    </div>
  );
};

export default AIAssistant;
