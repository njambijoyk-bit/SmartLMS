import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  ChevronLeft, ChevronRight, Flag, AlertTriangle,
  CheckCircle2, Save, Send, Eye, EyeOff, Maximize2, Minimize2,
  Calculator, HelpCircle, Bold, Italic, Underline,
  List, ListOrdered, Timer, AlertCircle
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

// Types matching backend models
interface Question {
  id: string;
  questionId: string;
  order: number;
  points: number;
  question: {
    id: string;
    questionText: string;
    questionType: QuestionType;
    options?: QuestionOption[];
    correctAnswer?: string;
    explanation?: string;
    difficulty?: string;
    tags?: string[];
  };
}

type QuestionType = 
  | 'MultipleChoice'
  | 'TrueFalse'
  | 'ShortAnswer'
  | 'LongAnswer'
  | 'Matching'
  | 'Ordering'
  | 'FillInBlank'
  | 'Code';

interface QuestionOption {
  id: string;
  text: string;
  isCorrect: boolean;
}

interface Assessment {
  id: string;
  title: string;
  description?: string;
  assessmentType: 'Quiz' | 'Exam' | 'Assignment' | 'Practice';
  courseId: string;
  courseName: string;
  timeLimitMinutes?: number;
  passingScore: number;
  shuffleQuestions: boolean;
  shuffleOptions: boolean;
  showResults: boolean;
  showResultsImmediately: boolean;
  allowRetries: boolean;
  maxRetries?: number;
  requireLockdownBrowser: boolean;
  dueDate?: string;
}

interface AnswerState {
  questionId: string;
  answerText?: string;
  selectedOptions?: string[];
  codeContent?: string;
  flagged?: boolean;
}

interface Attempt {
  id: string;
  assessmentId: string;
  userId: string;
  startedAt: string;
  submittedAt?: string;
  status: 'in_progress' | 'submitted' | 'graded' | 'expired';
  timeSpentSeconds: number;
  attemptNumber: number;
}

const QUESTION_TYPE_LABELS: Record<QuestionType, string> = {
  MultipleChoice: 'Multiple Choice',
  TrueFalse: 'True/False',
  ShortAnswer: 'Short Answer',
  LongAnswer: 'Essay',
  Matching: 'Matching',
  Ordering: 'Ordering',
  FillInBlank: 'Fill in the Blank',
  Code: 'Code',
};

export function AssessmentTakerPage({ assessmentId }: { assessmentId: string }) {
  const { user } = useAuth();
  
  // Mock data - will be replaced with API calls
  const [assessment, setAssessment] = useState<Assessment | null>(null);
  const [questions, setQuestions] = useState<Question[]>([]);
  const [attempt, setAttempt] = useState<Attempt | null>(null);
  const [answers, setAnswers] = useState<Record<string, AnswerState>>({});
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [timeRemaining, setTimeRemaining] = useState<number | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [autoSaveStatus, setAutoSaveStatus] = useState<'saved' | 'saving' | 'error'>('saved');
  const [showConfirmSubmit, setShowConfirmSubmit] = useState(false);
  const [lockdownMode, setLockdownMode] = useState(false);
  const [fullscreenMode, setFullscreenMode] = useState(false);
  const [warningCount, setWarningCount] = useState(0);
  
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const autoSaveRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Initialize attempt
  useEffect(() => {
    async function startAttempt() {
      try {
        // TODO: Call API to start attempt
        // const response = await api.post(`/assessments/${assessmentId}/start`);
        
        // Mock data for development
        const mockAssessment: Assessment = {
          id: assessmentId,
          title: 'CAT 2 — Binary Trees & Hash Tables',
          description: 'This assessment covers binary search trees, AVL trees, hash tables, collision resolution, and complexity analysis.',
          assessmentType: 'Quiz',
          courseId: 'cs301',
          courseName: 'Data Structures & Algorithms',
          timeLimitMinutes: 45,
          passingScore: 50,
          shuffleQuestions: true,
          shuffleOptions: true,
          showResults: true,
          showResultsImmediately: false,
          allowRetries: true,
          maxRetries: 2,
          requireLockdownBrowser: false,
          dueDate: new Date(Date.now() + 86400000).toISOString(),
        };

        const mockQuestions: Question[] = [
          {
            id: 'aq1',
            questionId: 'q1',
            order: 1,
            points: 5,
            question: {
              id: 'q1',
              questionText: 'What is the time complexity of searching in a balanced binary search tree?',
              questionType: 'MultipleChoice',
              options: [
                { id: 'o1', text: 'O(n)', isCorrect: false },
                { id: 'o2', text: 'O(log n)', isCorrect: true },
                { id: 'o3', text: 'O(n log n)', isCorrect: false },
                { id: 'o4', text: 'O(1)', isCorrect: false },
              ],
              difficulty: 'Medium',
              tags: ['BST', 'Complexity'],
            },
          },
          {
            id: 'aq2',
            questionId: 'q2',
            order: 2,
            points: 5,
            question: {
              id: 'q2',
              questionText: 'Which of the following is NOT a valid hash collision resolution technique?',
              questionType: 'MultipleChoice',
              options: [
                { id: 'o1', text: 'Chaining', isCorrect: false },
                { id: 'o2', text: 'Linear Probing', isCorrect: false },
                { id: 'o3', text: 'Binary Search', isCorrect: true },
                { id: 'o4', text: 'Quadratic Probing', isCorrect: false },
              ],
              difficulty: 'Easy',
              tags: ['Hash Tables'],
            },
          },
          {
            id: 'aq3',
            questionId: 'q3',
            order: 3,
            points: 10,
            question: {
              id: 'q3',
              questionText: 'Explain the difference between a binary search tree and an AVL tree. When would you choose one over the other?',
              questionType: 'LongAnswer',
              difficulty: 'Hard',
              tags: ['BST', 'AVL', 'Theory'],
            },
          },
          {
            id: 'aq4',
            questionId: 'q4',
            order: 4,
            points: 5,
            question: {
              id: 'q4',
              questionText: 'A hash table uses chaining for collision resolution. If we insert n keys into a hash table with m slots, what is the average-case time complexity for search?',
              questionType: 'MultipleChoice',
              options: [
                { id: 'o1', text: 'O(1)', isCorrect: true },
                { id: 'o2', text: 'O(n/m)', isCorrect: false },
                { id: 'o3', text: 'O(n)', isCorrect: false },
                { id: 'o4', text: 'O(log n)', isCorrect: false },
              ],
              difficulty: 'Medium',
              tags: ['Hash Tables', 'Complexity'],
            },
          },
        ];

        const mockAttempt: Attempt = {
          id: 'attempt-1',
          assessmentId,
          userId: user?.id || 'user-1',
          startedAt: new Date().toISOString(),
          status: 'in_progress',
          timeSpentSeconds: 0,
          attemptNumber: 1,
        };

        setAssessment(mockAssessment);
        setQuestions(mockQuestions);
        setAttempt(mockAttempt);
        setTimeRemaining(mockAssessment.timeLimitMinutes ? mockAssessment.timeLimitMinutes * 60 : null);

      } catch (error) {
        console.error('Failed to start attempt:', error);
      }
    }

    startAttempt();
  }, [assessmentId, user?.id]);

  // Timer countdown
  useEffect(() => {
    if (timeRemaining !== null && timeRemaining > 0 && attempt?.status === 'in_progress') {
      timerRef.current = setInterval(() => {
        setTimeRemaining(prev => {
          if (prev === null || prev <= 1) {
            handleSubmitAttempt(true); // Auto-submit on timeout
            return 0;
          }
          return prev - 1;
        });
      }, 1000);
    }

    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, [timeRemaining, attempt?.status]);

  // Auto-save every 30 seconds
  useEffect(() => {
    if (attempt?.status === 'in_progress') {
      autoSaveRef.current = setInterval(() => {
        handleAutoSave();
      }, 30000);
    }

    return () => {
      if (autoSaveRef.current) clearInterval(autoSaveRef.current);
    };
  }, [answers, attempt?.status]);

  // Lockdown browser detection
  useEffect(() => {
    if (!assessment?.requireLockdownBrowser) return;

    const handleVisibilityChange = () => {
      if (document.hidden) {
        setWarningCount(prev => prev + 1);
        // Could trigger auto-submit after N warnings
      }
    };

    const handleBlur = () => {
      setWarningCount(prev => prev + 1);
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    window.addEventListener('blur', handleBlur);

    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      window.removeEventListener('blur', handleBlur);
    };
  }, [assessment?.requireLockdownBrowser]);

  const handleAutoSave = async () => {
    setAutoSaveStatus('saving');
    try {
      // TODO: Call API to save answers
      // await api.post(`/assessments/attempts/${attempt?.id}/save`, { answers });
      setTimeout(() => setAutoSaveStatus('saved'), 500);
    } catch (error) {
      setAutoSaveStatus('error');
      console.error('Auto-save failed:', error);
    }
  };

  const handleAnswerChange = (questionId: string, answer: Partial<AnswerState>) => {
    setAnswers(prev => ({
      ...prev,
      [questionId]: {
        ...(prev[questionId] || {}),
        ...answer,
      },
    }));
  };

  const handleOptionSelect = (questionId: string, optionId: string, isMultiple: boolean) => {
    const currentAnswer = answers[questionId];
    const selected = currentAnswer?.selectedOptions || [];

    if (isMultiple) {
      const newSelected = selected.includes(optionId)
        ? selected.filter(id => id !== optionId)
        : [...selected, optionId];
      handleAnswerChange(questionId, { selectedOptions: newSelected });
    } else {
      handleAnswerChange(questionId, { selectedOptions: [optionId] });
    }
  };

  const handleFlagQuestion = (questionId: string) => {
    const currentAnswer = answers[questionId];
    handleAnswerChange(questionId, { flagged: !currentAnswer?.flagged });
  };

  const handleSubmitAttempt = async (isTimeout = false) => {
    if (isSubmitting) return;
    
    if (!isTimeout && !showConfirmSubmit) {
      setShowConfirmSubmit(true);
      return;
    }

    setIsSubmitting(true);
    try {
      // TODO: Call API to submit attempt
      // await api.post(`/assessments/attempts/${attempt?.id}/submit`);
      
      setAttempt(prev => prev ? { ...prev, status: 'submitted', submittedAt: new Date().toISOString() } : null);
      setShowConfirmSubmit(false);
    } catch (error) {
      console.error('Submission failed:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  const toggleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
      setFullscreenMode(true);
    } else {
      document.exitFullscreen();
      setFullscreenMode(false);
    }
  };

  const formatTime = (seconds: number) => {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hrs > 0) {
      return `${hrs}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    }
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const getTimeColor = () => {
    if (!timeRemaining) return 'text-ink';
    const ratio = timeRemaining / ((assessment?.timeLimitMinutes || 0) * 60);
    if (ratio < 0.1) return 'text-danger';
    if (ratio < 0.25) return 'text-warning';
    return 'text-success';
  };

  const getQuestionProgress = () => {
    const answered = Object.keys(answers).filter(qId => {
      const ans = answers[qId];
      return ans.selectedOptions?.length || ans.answerText?.trim() || ans.codeContent?.trim();
    }).length;
    return { answered, total: questions.length };
  };

  const currentQuestion = questions[currentQuestionIndex];

  if (!assessment || !attempt) {
    return (
      <div className="flex items-center justify-center min-h-[60vh]">
        <div className="text-center">
          <div className="w-12 h-12 border-4 border-brand-200 border-t-brand-500 rounded-full animate-spin mx-auto mb-4" />
          <p className="text-ink-tertiary">Loading assessment...</p>
        </div>
      </div>
    );
  }

  if (attempt.status === 'submitted') {
    return (
      <div className="max-w-2xl mx-auto py-12">
        <Card className="text-center p-8">
          <motion.div
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            className="w-20 h-20 bg-success rounded-full flex items-center justify-center mx-auto mb-6"
          >
            <CheckCircle2 size={40} className="text-white" />
          </motion.div>
          <h2 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink mb-2">
            Assessment Submitted!
          </h2>
          <p className="text-ink-tertiary mb-6">
            Your answers have been submitted successfully.
          </p>
          <div className="bg-sand-100 rounded-lg p-4 mb-6">
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <div className="text-ink-tertiary">Time Spent</div>
                <div className="font-bold font-[family-name:var(--font-display)]">
                  {formatTime(attempt.timeSpentSeconds)}
                </div>
              </div>
              <div>
                <div className="text-ink-tertiary">Questions Answered</div>
                <div className="font-bold font-[family-name:var(--font-display)]">
                  {getQuestionProgress().answered}/{questions.length}
                </div>
              </div>
            </div>
          </div>
          {!assessment.showResultsImmediately ? (
            <p className="text-sm text-ink-tertiary">
              Results will be available after the due date or when your instructor releases them.
            </p>
          ) : (
            <Button>View Results</Button>
          )}
        </Card>
      </div>
    );
  }

  const progress = getQuestionProgress();

  return (
    <div className="space-y-4">
      {/* Header */}
      <Card className="p-4 sticky top-0 z-10">
        <div className="flex items-center justify-between gap-4">
          {/* Left: Assessment info */}
          <div className="flex items-center gap-4">
            <div>
              <h1 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink truncate max-w-md">
                {assessment.title}
              </h1>
              <p className="text-xs text-ink-tertiary">{assessment.courseName}</p>
            </div>
            <Badge variant={assessment.assessmentType === 'Exam' ? 'danger' : 'default'}>
              {assessment.assessmentType}
            </Badge>
          </div>

          {/* Center: Timer */}
          {timeRemaining !== null && (
            <div className={`flex items-center gap-2 px-4 py-2 rounded-lg bg-sand-100 ${getTimeColor()}`}>
              <Timer size={18} />
              <span className="text-lg font-mono font-bold">{formatTime(timeRemaining)}</span>
              {timeRemaining < 300 && (
                <AlertCircle size={16} className="animate-pulse" />
              )}
            </div>
          )}

          {/* Right: Actions */}
          <div className="flex items-center gap-2">
            {/* Auto-save status */}
            <div className="flex items-center gap-1 text-xs text-ink-tertiary">
              {autoSaveStatus === 'saving' && <Save size={12} className="animate-spin" />}
              {autoSaveStatus === 'saved' && <CheckCircle2 size={12} />}
              {autoSaveStatus === 'error' && <AlertTriangle size={12} className="text-danger" />}
              <span>{autoSaveStatus === 'saving' ? 'Saving...' : autoSaveStatus === 'saved' ? 'Saved' : 'Error'}</span>
            </div>

            {assessment.requireLockdownBrowser && (
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setLockdownMode(!lockdownMode)}
                className={lockdownMode ? 'text-success' : ''}
              >
                {lockdownMode ? <EyeOff size={16} /> : <Eye size={16} />}
              </Button>
            )}

            <Button variant="ghost" size="sm" onClick={toggleFullscreen}>
              {fullscreenMode ? <Minimize2 size={16} /> : <Maximize2 size={16} />}
            </Button>

            <Button variant="ghost" size="sm">
              <HelpCircle size={16} />
            </Button>

            <Button
              size="sm"
              onClick={() => handleSubmitAttempt(false)}
              disabled={isSubmitting}
            >
              <Send size={16} />
              Submit
            </Button>
          </div>
        </div>

        {/* Progress bar */}
        <div className="mt-3">
          <div className="flex items-center justify-between text-xs text-ink-tertiary mb-1">
            <span>Progress: {progress.answered}/{progress.total} answered</span>
            <span>{Math.round((progress.answered / progress.total) * 100)}%</span>
          </div>
          <div className="h-2 bg-sand-200 rounded-full overflow-hidden">
            <motion.div
              className="h-full bg-brand-500 rounded-full"
              initial={{ width: 0 }}
              animate={{ width: `${(progress.answered / progress.total) * 100}%` }}
              transition={{ duration: 0.3 }}
            />
          </div>
        </div>
      </Card>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
        {/* Main content: Question */}
        <div className="lg:col-span-3">
          <Card className="p-6 min-h-[500px]">
            <AnimatePresence mode="wait">
              <motion.div
                key={currentQuestion.id}
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: -20 }}
                transition={{ duration: 0.2 }}
              >
                {/* Question header */}
                <div className="flex items-start justify-between mb-6">
                  <div className="flex items-center gap-3">
                    <Badge variant="default">
                      Question {currentQuestion.order} of {questions.length}
                    </Badge>
                    <Badge variant="accent">
                      {QUESTION_TYPE_LABELS[currentQuestion.question.questionType]}
                    </Badge>
                    <span className="text-sm text-ink-tertiary">
                      {currentQuestion.points} {currentQuestion.points === 1 ? 'point' : 'points'}
                    </span>
                  </div>
                  <button
                    onClick={() => handleFlagQuestion(currentQuestion.questionId)}
                    className={`p-2 rounded-lg hover:bg-sand-100 transition-colors cursor-pointer ${
                      answers[currentQuestion.questionId]?.flagged ? 'text-warning' : 'text-ink-tertiary'
                    }`}
                  >
                    <Flag size={18} fill={answers[currentQuestion.questionId]?.flagged ? 'currentColor' : 'none'} />
                  </button>
                </div>

                {/* Question text */}
                <div className="prose prose-sm max-w-none mb-6">
                  <p className="text-base text-ink leading-relaxed">
                    {currentQuestion.question.questionText}
                  </p>
                </div>

                {/* Question type specific rendering */}
                {currentQuestion.question.questionType === 'MultipleChoice' && (
                  <div className="space-y-3">
                    {(currentQuestion.question.options || []).map(option => (
                      <label
                        key={option.id}
                        className={`flex items-center gap-3 p-4 rounded-lg border-2 cursor-pointer transition-all ${
                          answers[currentQuestion.questionId]?.selectedOptions?.includes(option.id)
                            ? 'border-brand-500 bg-brand-50'
                            : 'border-sand-200 hover:border-brand-300 hover:bg-sand-50'
                        }`}
                      >
                        <input
                          type="radio"
                          name={`question-${currentQuestion.questionId}`}
                          value={option.id}
                          checked={answers[currentQuestion.questionId]?.selectedOptions?.includes(option.id) || false}
                          onChange={() => handleOptionSelect(currentQuestion.questionId, option.id, false)}
                          className="w-4 h-4 text-brand-600"
                        />
                        <span className="text-ink">{option.text}</span>
                      </label>
                    ))}
                  </div>
                )}

                {currentQuestion.question.questionType === 'LongAnswer' && (
                  <div>
                    <div className="flex items-center gap-2 mb-2">
                      <Button variant="ghost" size="sm"><Bold size={14} /></Button>
                      <Button variant="ghost" size="sm"><Italic size={14} /></Button>
                      <Button variant="ghost" size="sm"><Underline size={14} /></Button>
                      <Button variant="ghost" size="sm"><List size={14} /></Button>
                      <Button variant="ghost" size="sm"><ListOrdered size={14} /></Button>
                    </div>
                    <textarea
                      value={answers[currentQuestion.questionId]?.answerText || ''}
                      onChange={(e) => handleAnswerChange(currentQuestion.questionId, { answerText: e.target.value })}
                      placeholder="Type your answer here..."
                      className="w-full h-64 p-4 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400 resize-none"
                    />
                    <div className="flex items-center justify-between mt-2 text-xs text-ink-tertiary">
                      <span>Word count: {(answers[currentQuestion.questionId]?.answerText || '').split(/\s+/).filter(w => w).length}</span>
                      <span>Characters: {(answers[currentQuestion.questionId]?.answerText || '').length}</span>
                    </div>
                  </div>
                )}

                {currentQuestion.question.questionType === 'ShortAnswer' && (
                  <input
                    type="text"
                    value={answers[currentQuestion.questionId]?.answerText || ''}
                    onChange={(e) => handleAnswerChange(currentQuestion.questionId, { answerText: e.target.value })}
                    placeholder="Type your answer here..."
                    className="w-full p-4 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
                  />
                )}

                {currentQuestion.question.questionType === 'Code' && (
                  <div>
                    <div className="flex items-center gap-2 mb-2">
                      <select className="px-3 py-1.5 border border-sand-300 rounded-lg text-sm bg-white">
                        <option>Python</option>
                        <option>Java</option>
                        <option>C++</option>
                        <option>JavaScript</option>
                      </select>
                      <Button variant="ghost" size="sm"><Calculator size={14} /> Run Code</Button>
                    </div>
                    <textarea
                      value={answers[currentQuestion.questionId]?.codeContent || ''}
                      onChange={(e) => handleAnswerChange(currentQuestion.questionId, { codeContent: e.target.value })}
                      placeholder="// Write your code here..."
                      className="w-full h-64 p-4 font-mono text-sm border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400 resize-none bg-surface-sunken"
                    />
                  </div>
                )}

                {/* Navigation buttons */}
                <div className="flex items-center justify-between mt-8 pt-6 border-t border-sand-200">
                  <Button
                    variant="outline"
                    onClick={() => setCurrentQuestionIndex(prev => Math.max(0, prev - 1))}
                    disabled={currentQuestionIndex === 0}
                  >
                    <ChevronLeft size={16} /> Previous
                  </Button>

                  <div className="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      onClick={() => handleFlagQuestion(currentQuestion.questionId)}
                      className={answers[currentQuestion.questionId]?.flagged ? 'text-warning' : ''}
                    >
                      <Flag size={16} /> {answers[currentQuestion.questionId]?.flagged ? 'Flagged' : 'Flag for Review'}
                    </Button>
                  </div>

                  {currentQuestionIndex < questions.length - 1 ? (
                    <Button
                      onClick={() => setCurrentQuestionIndex(prev => prev + 1)}
                    >
                      Next <ChevronRight size={16} />
                    </Button>
                  ) : (
                    <Button
                      onClick={() => handleSubmitAttempt(false)}
                      variant="primary"
                    >
                      <Send size={16} /> Submit Assessment
                    </Button>
                  )}
                </div>
              </motion.div>
            </AnimatePresence>
          </Card>
        </div>

        {/* Sidebar: Question navigator */}
        <div className="lg:col-span-1">
          <Card className="p-4 sticky top-48">
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">
              Question Navigator
            </h3>
            
            <div className="grid grid-cols-4 gap-2 mb-4">
              {questions.map((q, idx) => {
                const isAnswered = !!answers[q.questionId]?.selectedOptions?.length || 
                                   !!answers[q.questionId]?.answerText?.trim() ||
                                   !!answers[q.questionId]?.codeContent?.trim();
                const isCurrent = idx === currentQuestionIndex;
                const isFlagged = answers[q.questionId]?.flagged;

                return (
                  <button
                    key={q.id}
                    onClick={() => setCurrentQuestionIndex(idx)}
                    className={`aspect-square rounded-lg text-sm font-medium transition-all ${
                      isCurrent
                        ? 'bg-brand-500 text-white ring-2 ring-brand-300'
                        : isAnswered
                        ? 'bg-success text-white'
                        : isFlagged
                        ? 'bg-warning text-white'
                        : 'bg-sand-100 text-ink hover:bg-sand-200'
                    }`}
                  >
                    {q.order}
                  </button>
                );
              })}
            </div>

            <div className="space-y-2 text-xs">
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 rounded bg-brand-500" />
                <span className="text-ink-tertiary">Current</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 rounded bg-success" />
                <span className="text-ink-tertiary">Answered</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 rounded bg-warning" />
                <span className="text-ink-tertiary">Flagged</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 rounded bg-sand-100 border border-sand-300" />
                <span className="text-ink-tertiary">Not visited</span>
              </div>
            </div>

            {/* Time info */}
            {timeRemaining !== null && (
              <div className="mt-6 pt-4 border-t border-sand-200">
                <div className="text-xs text-ink-tertiary mb-1">Time Remaining</div>
                <div className={`text-lg font-mono font-bold ${getTimeColor()}`}>
                  {formatTime(timeRemaining)}
                </div>
              </div>
            )}

            {/* Attempt info */}
            <div className="mt-4 pt-4 border-t border-sand-200">
              <div className="text-xs text-ink-tertiary">Attempt #{attempt.attemptNumber}</div>
              <div className="text-xs text-ink-tertiary mt-1">
                Started: {new Date(attempt.startedAt).toLocaleTimeString()}
              </div>
            </div>

            {/* Warnings for lockdown mode */}
            {warningCount > 0 && (
              <div className="mt-4 p-3 bg-warning-light rounded-lg border border-warning/20">
                <div className="flex items-center gap-2 text-warning text-xs">
                  <AlertTriangle size={14} />
                  <span>{warningCount} warning(s) detected</span>
                </div>
                <p className="text-[10px] text-ink-tertiary mt-1">
                  Leaving the assessment page may result in automatic submission.
                </p>
              </div>
            )}
          </Card>
        </div>
      </div>

      {/* Submit confirmation modal */}
      <AnimatePresence>
        {showConfirmSubmit && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
            onClick={() => setShowConfirmSubmit(false)}
          >
            <motion.div
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.9, opacity: 0 }}
              onClick={e => e.stopPropagation()}
              className="bg-white rounded-xl p-6 max-w-md w-full mx-4"
            >
              <div className="flex items-center gap-3 mb-4">
                <div className="w-12 h-12 rounded-full bg-warning-light flex items-center justify-center">
                  <AlertTriangle size={24} className="text-warning" />
                </div>
                <div>
                  <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">
                    Submit Assessment?
                  </h3>
                  <p className="text-sm text-ink-tertiary">
                    You won't be able to change your answers after submission.
                  </p>
                </div>
              </div>

              <div className="bg-sand-100 rounded-lg p-4 mb-6">
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <div className="text-ink-tertiary">Questions Answered</div>
                    <div className="font-bold">{progress.answered}/{progress.total}</div>
                  </div>
                  <div>
                    <div className="text-ink-tertiary">Flagged for Review</div>
                    <div className="font-bold">
                      {Object.values(answers).filter(a => a.flagged).length}
                    </div>
                  </div>
                </div>
              </div>

              <div className="flex gap-3">
                <Button
                  variant="outline"
                  onClick={() => setShowConfirmSubmit(false)}
                  className="flex-1"
                >
                  Continue Working
                </Button>
                <Button
                  variant="danger"
                  onClick={() => handleSubmitAttempt(true)}
                  disabled={isSubmitting}
                  className="flex-1"
                >
                  {isSubmitting ? 'Submitting...' : 'Yes, Submit'}
                </Button>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
