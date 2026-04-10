import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Plus, Trash2, Edit2, Eye, Save, X, Check,
  ChevronUp, ChevronDown, Copy, Settings, FileText,
  HelpCircle, Bold, Italic, Underline, Link as LinkIcon,
  Image as ImageIcon, Code, BookOpen, Search,
  Upload, Award, Unlock, Users, Clock, List
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Input } from '../../components/ui/Input';

// Types matching backend models
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

interface Question {
  id: string;
  bankId?: string;
  questionText: string;
  questionType: QuestionType;
  options?: QuestionOption[];
  correctAnswer: string;
  explanation?: string;
  points?: number;
  difficulty?: 'Easy' | 'Medium' | 'Hard';
  tags?: string[];
}

interface AssessmentQuestion {
  id: string;
  questionId: string;
  question: Question;
  order: number;
  points: number;
}

interface Assessment {
  id?: string;
  title: string;
  description?: string;
  assessmentType: 'Quiz' | 'Exam' | 'Assignment' | 'Practice';
  courseId: string;
  courseGroupId?: string;
  moduleId?: string;
  timeLimitMinutes?: number;
  passingScore: number;
  shuffleQuestions: boolean;
  shuffleOptions: boolean;
  showResults: boolean;
  showResultsImmediately: boolean;
  allowRetries: boolean;
  maxRetries?: number;
  requireLockdownBrowser: boolean;
  allowLateSubmission: boolean;
  latePenaltyPercent: number;
  startTime?: string;
  dueDate?: string;
  endTime?: string;
  isPublished: boolean;
  status: 'draft' | 'published' | 'archived';
}

const QUESTION_TYPE_OPTIONS: { value: QuestionType; label: string; icon: React.ReactNode }[] = [
  { value: 'MultipleChoice', label: 'Multiple Choice', icon: <List size={16} /> },
  { value: 'TrueFalse', label: 'True/False', icon: <Check size={16} /> },
  { value: 'ShortAnswer', label: 'Short Answer', icon: <FileText size={16} /> },
  { value: 'LongAnswer', label: 'Essay', icon: <BookOpen size={16} /> },
  { value: 'Matching', label: 'Matching', icon: <LinkIcon size={16} /> },
  { value: 'Ordering', label: 'Ordering', icon: <ChevronUp size={16} /> },
  { value: 'FillInBlank', label: 'Fill in the Blank', icon: <HelpCircle size={16} /> },
  { value: 'Code', label: 'Code', icon: <Code size={16} /> },
];

export function AssessmentBuilderPage({ 
  assessmentId, 
  courseId 
}: { 
  assessmentId?: string; 
  courseId: string;
}) {
  // State - user available for future auth checks
  const [activeTab, setActiveTab] = useState<'questions' | 'settings' | 'preview'>('questions');
  const [assessment, setAssessment] = useState<Assessment>({
    title: '',
    description: '',
    assessmentType: 'Quiz',
    courseId,
    passingScore: 50,
    shuffleQuestions: true,
    shuffleOptions: true,
    showResults: true,
    showResultsImmediately: false,
    allowRetries: false,
    requireLockdownBrowser: false,
    allowLateSubmission: false,
    latePenaltyPercent: 10,
    isPublished: false,
    status: 'draft',
  });
  const [questions, setQuestions] = useState<AssessmentQuestion[]>([]);
  const [selectedQuestion, setSelectedQuestion] = useState<string | null>(null);
  const [isEditingQuestion, setIsEditingQuestion] = useState(false);
  const [showQuestionBank, setShowQuestionBank] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [showPublishModal, setShowPublishModal] = useState(false);

  // Load existing assessment if editing
  useEffect(() => {
    if (assessmentId) {
      // TODO: Load from API
      // For now, use mock data
      setAssessment(prev => ({
        ...prev,
        title: 'CAT 2 — Data Structures',
        description: 'Comprehensive assessment on binary trees, hash tables, and graph algorithms.',
        timeLimitMinutes: 45,
        dueDate: new Date(Date.now() + 7 * 86400000).toISOString(),
      }));
      
      setQuestions([
        {
          id: 'aq1',
          questionId: 'q1',
          order: 1,
          points: 5,
          question: {
            id: 'q1',
            questionText: 'What is the time complexity of searching in a balanced BST?',
            questionType: 'MultipleChoice',
            options: [
              { id: 'o1', text: 'O(n)', isCorrect: false },
              { id: 'o2', text: 'O(log n)', isCorrect: true },
              { id: 'o3', text: 'O(n log n)', isCorrect: false },
              { id: 'o4', text: 'O(1)', isCorrect: false },
            ],
            correctAnswer: 'o2',
            difficulty: 'Medium',
            tags: ['BST', 'Complexity'],
          },
        },
      ]);
    }
  }, [assessmentId]);

  const handleAddQuestion = () => {
    const newQuestion: AssessmentQuestion = {
      id: `aq-${Date.now()}`,
      questionId: `q-${Date.now()}`,
      order: questions.length + 1,
      points: 5,
      question: {
        id: `q-${Date.now()}`,
        questionText: '',
        questionType: 'MultipleChoice',
        options: [
          { id: 'o1', text: '', isCorrect: false },
          { id: 'o2', text: '', isCorrect: false },
          { id: 'o3', text: '', isCorrect: false },
          { id: 'o4', text: '', isCorrect: false },
        ],
        correctAnswer: '',
        difficulty: 'Medium',
        tags: [],
      },
    };
    setQuestions([...questions, newQuestion]);
    setSelectedQuestion(newQuestion.id);
    setIsEditingQuestion(true);
  };

  const handleAddFromBank = () => {
    setShowQuestionBank(true);
  };

  const handleDeleteQuestion = (questionId: string) => {
    setQuestions(questions.filter(q => q.id !== questionId));
    if (selectedQuestion === questionId) {
      setSelectedQuestion(null);
      setIsEditingQuestion(false);
    }
  };

  const handleDuplicateQuestion = (question: AssessmentQuestion) => {
    const duplicate: AssessmentQuestion = {
      ...question,
      id: `aq-${Date.now()}`,
      questionId: `q-${Date.now()}`,
      order: questions.length + 1,
      question: {
        ...question.question,
        id: `q-${Date.now()}`,
        options: question.question.options?.map(opt => ({
          ...opt,
          id: `o-${Date.now()}-${Math.random()}`,
        })),
      },
    };
    setQuestions([...questions, duplicate]);
  };

  const handleMoveQuestion = (index: number, direction: 'up' | 'down') => {
    const newQuestions = [...questions];
    const newIndex = direction === 'up' ? index - 1 : index + 1;
    
    if (newIndex < 0 || newIndex >= newQuestions.length) return;
    
    [newQuestions[index], newQuestions[newIndex]] = [newQuestions[newIndex], newQuestions[index]];
    newQuestions.forEach((q, i) => q.order = i + 1);
    
    setQuestions(newQuestions);
  };

  const handleUpdateQuestion = (updatedQuestion: AssessmentQuestion) => {
    setQuestions(questions.map(q => 
      q.id === updatedQuestion.id ? updatedQuestion : q
    ));
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      // TODO: Call API to save assessment
      // await api.post('/assessments', { assessment, questions });
      setTimeout(() => setIsSaving(false), 1000);
    } catch (error) {
      console.error('Save failed:', error);
      setIsSaving(false);
    }
  };

  const handlePublish = async () => {
    try {
      // TODO: Call API to publish
      setAssessment(prev => ({ ...prev, isPublished: true, status: 'published' }));
      setShowPublishModal(false);
    } catch (error) {
      console.error('Publish failed:', error);
    }
  };

  const totalPoints = questions.reduce((sum, q) => sum + q.points, 0);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">
            {assessmentId ? 'Edit Assessment' : 'Create Assessment'}
          </h1>
          <p className="text-sm text-ink-tertiary mt-1">
            Build your assessment with questions from the question bank or create new ones
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => setActiveTab('preview')}>
            <Eye size={16} /> Preview
          </Button>
          <Button variant="outline" onClick={handleSave} disabled={isSaving}>
            {isSaving ? <Save size={16} className="animate-spin" /> : <Save size={16} />}
            {isSaving ? 'Saving...' : 'Save Draft'}
          </Button>
          {!assessment.isPublished ? (
            <Button onClick={() => setShowPublishModal(true)}>
              <Unlock size={16} /> Publish
            </Button>
          ) : (
            <Badge variant="success">Published</Badge>
          )}
        </div>
      </div>

      {/* Stats bar */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center">
              <FileText size={18} className="text-brand-600" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">
                {questions.length}
              </div>
              <div className="text-xs text-ink-tertiary">Questions</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center">
              <Award size={18} className="text-success" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">
                {totalPoints}
              </div>
              <div className="text-xs text-ink-tertiary">Total Points</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-info-light flex items-center justify-center">
              <Clock size={18} className="text-info" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">
                {assessment.timeLimitMinutes || '--'}
              </div>
              <div className="text-xs text-ink-tertiary">Minutes</div>
            </div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center">
              <Users size={18} className="text-warning" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">
                {assessment.passingScore}%
              </div>
              <div className="text-xs text-ink-tertiary">Passing Score</div>
            </div>
          </div>
        </Card>
      </div>

      {/* Tabs */}
      <div className="flex gap-2 border-b border-sand-200">
        {[
          { id: 'questions', label: 'Questions', icon: <FileText size={16} /> },
          { id: 'settings', label: 'Settings', icon: <Settings size={16} /> },
          { id: 'preview', label: 'Preview', icon: <Eye size={16} /> },
        ].map(tab => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as any)}
            className={`flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors cursor-pointer ${
              activeTab === tab.id
                ? 'border-brand-500 text-brand-600'
                : 'border-transparent text-ink-tertiary hover:text-ink'
            }`}
          >
            {tab.icon}
            {tab.label}
          </button>
        ))}
      </div>

      {/* Content */}
      {activeTab === 'questions' && (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Question list */}
          <div className="lg:col-span-1 space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink">
                Questions ({questions.length})
              </h2>
              <div className="flex gap-2">
                <Button variant="outline" size="sm" onClick={handleAddFromBank}>
                  <Upload size={14} /> From Bank
                </Button>
                <Button size="sm" onClick={handleAddQuestion}>
                  <Plus size={14} /> Add New
                </Button>
              </div>
            </div>

            <div className="space-y-2">
              <AnimatePresence>
                {questions.map((q, idx) => (
                  <motion.div
                    key={q.id}
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 10 }}
                    layout
                  >
                    <Card
                      className={`p-3 cursor-pointer transition-all ${
                        selectedQuestion === q.id ? 'ring-2 ring-brand-300 bg-brand-50' : 'hover:bg-sand-50'
                      }`}
                      onClick={() => {
                        setSelectedQuestion(q.id);
                        setIsEditingQuestion(true);
                      }}
                    >
                      <div className="flex items-start gap-3">
                        <div className="flex flex-col gap-1">
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              handleMoveQuestion(idx, 'up');
                            }}
                            disabled={idx === 0}
                            className="p-1 hover:bg-sand-100 rounded cursor-pointer disabled:opacity-30"
                          >
                            <ChevronUp size={14} />
                          </button>
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              handleMoveQuestion(idx, 'down');
                            }}
                            disabled={idx === questions.length - 1}
                            className="p-1 hover:bg-sand-100 rounded cursor-pointer disabled:opacity-30"
                          >
                            <ChevronDown size={14} />
                          </button>
                        </div>

                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1">
                            <Badge variant="default" className="text-xs">
                              Q{q.order}
                            </Badge>
                            <Badge variant="accent" className="text-xs">
                              {q.question.questionType.replace(/([A-Z])/g, ' $1').trim()}
                            </Badge>
                            <span className="text-xs text-ink-tertiary">{q.points} pts</span>
                          </div>
                          <p className="text-sm text-ink truncate">
                            {q.question.questionText || 'Untitled Question'}
                          </p>
                        </div>

                        <div className="flex gap-1">
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              handleDuplicateQuestion(q);
                            }}
                            className="p-1.5 hover:bg-sand-100 rounded text-ink-tertiary cursor-pointer"
                          >
                            <Copy size={14} />
                          </button>
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              handleDeleteQuestion(q.id);
                            }}
                            className="p-1.5 hover:bg-danger-light rounded text-danger cursor-pointer"
                          >
                            <Trash2 size={14} />
                          </button>
                        </div>
                      </div>
                    </Card>
                  </motion.div>
                ))}
              </AnimatePresence>

              {questions.length === 0 && (
                <Card className="p-8 text-center">
                  <FileText size={36} className="mx-auto text-ink-placeholder mb-3" />
                  <p className="text-ink-tertiary mb-4">No questions added yet</p>
                  <Button onClick={handleAddQuestion}>
                    <Plus size={16} /> Add Your First Question
                  </Button>
                </Card>
              )}
            </div>
          </div>

          {/* Question editor */}
          <div className="lg:col-span-2">
            {selectedQuestion && isEditingQuestion ? (
              <QuestionEditor
                question={questions.find(q => q.id === selectedQuestion)!}
                onUpdate={handleUpdateQuestion}
                onClose={() => {
                  setIsEditingQuestion(false);
                  setSelectedQuestion(null);
                }}
              />
            ) : (
              <Card className="p-8 text-center">
                <div className="max-w-md mx-auto">
                  <Edit2 size={48} className="mx-auto text-ink-placeholder mb-4" />
                  <h3 className="text-lg font-semibold font-[family-name:var(--font-display)] text-ink mb-2">
                    Select a Question to Edit
                  </h3>
                  <p className="text-ink-tertiary mb-4">
                    Click on a question from the list to edit its content, or add a new question
                  </p>
                  <Button onClick={handleAddQuestion}>
                    <Plus size={16} /> Create New Question
                  </Button>
                </div>
              </Card>
            )}
          </div>
        </div>
      )}

      {activeTab === 'settings' && (
        <AssessmentSettings
          assessment={assessment}
          onChange={setAssessment}
        />
      )}

      {activeTab === 'preview' && (
        <Card className="p-6">
          <div className="max-w-2xl mx-auto">
            <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-2">
              {assessment.title || 'Untitled Assessment'}
            </h2>
            {assessment.description && (
              <p className="text-ink-tertiary mb-6">{assessment.description}</p>
            )}
            
            <div className="grid grid-cols-2 gap-4 mb-6">
              <div>
                <div className="text-xs text-ink-tertiary">Type</div>
                <div className="font-semibold">{assessment.assessmentType}</div>
              </div>
              <div>
                <div className="text-xs text-ink-tertiary">Time Limit</div>
                <div className="font-semibold">{assessment.timeLimitMinutes || 'None'} minutes</div>
              </div>
              <div>
                <div className="text-xs text-ink-tertiary">Total Points</div>
                <div className="font-semibold">{totalPoints}</div>
              </div>
              <div>
                <div className="text-xs text-ink-tertiary">Passing Score</div>
                <div className="font-semibold">{assessment.passingScore}%</div>
              </div>
            </div>

            <div className="border-t border-sand-200 pt-6">
              <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">
                Questions ({questions.length})
              </h3>
              <div className="space-y-4">
                {questions.map((q) => (
                  <div key={q.id} className="p-4 bg-sand-50 rounded-lg">
                    <div className="flex items-center gap-2 mb-2">
                      <Badge variant="default">Question {q.order}</Badge>
                      <span className="text-xs text-ink-tertiary">{q.points} points</span>
                    </div>
                    <p className="text-sm text-ink mb-3">{q.question.questionText}</p>
                    {q.question.options && (
                      <div className="space-y-2">
                        {q.question.options.map((opt, optIdx) => (
                          <div
                            key={opt.id}
                            className={`flex items-center gap-2 text-sm p-2 rounded ${
                              opt.isCorrect ? 'bg-success-light text-success' : 'bg-white'
                            }`}
                          >
                            <div className="w-4 h-4 rounded border border-sand-300 flex items-center justify-center text-xs">
                              {String.fromCharCode(65 + optIdx)}
                            </div>
                            {opt.text || 'Option text'}
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>
        </Card>
      )}

      {/* Question Bank Modal */}
      <AnimatePresence>
        {showQuestionBank && (
          <QuestionBankModal
            onClose={() => setShowQuestionBank(false)}
            onSelect={(questions) => {
              const newQuestions: AssessmentQuestion[] = questions.map((q, idx) => ({
                id: `aq-${Date.now()}-${idx}`,
                questionId: q.id,
                order: questions.length + idx + 1,
                points: q.points || 5,
                question: q,
              }));
              setQuestions(prev => [...prev, ...newQuestions]);
              setShowQuestionBank(false);
            }}
          />
        )}
      </AnimatePresence>

      {/* Publish Modal */}
      <AnimatePresence>
        {showPublishModal && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
            onClick={() => setShowPublishModal(false)}
          >
            <motion.div
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              exit={{ scale: 0.9, opacity: 0 }}
              onClick={e => e.stopPropagation()}
              className="bg-white rounded-xl p-6 max-w-md w-full mx-4"
            >
              <div className="flex items-center gap-3 mb-4">
                <div className="w-12 h-12 rounded-full bg-success-light flex items-center justify-center">
                  <Unlock size={24} className="text-success" />
                </div>
                <div>
                  <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">
                    Publish Assessment?
                  </h3>
                  <p className="text-sm text-ink-tertiary">
                    Students will be able to see and take this assessment
                  </p>
                </div>
              </div>

              <div className="bg-sand-100 rounded-lg p-4 mb-6">
                <div className="space-y-2 text-sm">
                  <div className="flex items-center gap-2">
                    <Check size={16} className="text-success" />
                    <span>{questions.length} questions ready</span>
                  </div>
                  <div className="flex items-center gap-2">
                    <Check size={16} className="text-success" />
                    <span>Total: {totalPoints} points</span>
                  </div>
                  {assessment.dueDate && (
                    <div className="flex items-center gap-2">
                      <Check size={16} className="text-success" />
                      <span>Due: {new Date(assessment.dueDate).toLocaleDateString()}</span>
                    </div>
                  )}
                </div>
              </div>

              <div className="flex gap-3">
                <Button
                  variant="outline"
                  onClick={() => setShowPublishModal(false)}
                  className="flex-1"
                >
                  Cancel
                </Button>
                <Button onClick={handlePublish} className="flex-1">
                  Publish Now
                </Button>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

// Question Editor Component
function QuestionEditor({
  question,
  onUpdate,
  onClose,
}: {
  question: AssessmentQuestion;
  onUpdate: (q: AssessmentQuestion) => void;
  onClose: () => void;
}) {
  const [localQuestion, setLocalQuestion] = useState(question);

  const updateField = (field: keyof Question, value: any) => {
    const updated = {
      ...localQuestion,
      question: { ...localQuestion.question, [field]: value },
    };
    setLocalQuestion(updated);
    onUpdate(updated);
  };

  const updateOption = (optionId: string, field: keyof QuestionOption, value: any) => {
    const updatedOptions = localQuestion.question.options?.map(opt =>
      opt.id === optionId ? { ...opt, [field]: value } : opt
    );
    updateField('options', updatedOptions);
  };

  const addOption = () => {
    const newOption: QuestionOption = {
      id: `o-${Date.now()}`,
      text: '',
      isCorrect: false,
    };
    updateField('options', [...(localQuestion.question.options || []), newOption]);
  };

  const removeOption = (optionId: string) => {
    updateField(
      'options',
      localQuestion.question.options?.filter(opt => opt.id !== optionId)
    );
  };

  return (
    <Card className="p-6">
      <div className="flex items-start justify-between mb-6">
        <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">
          Edit Question
        </h3>
        <button onClick={onClose} className="p-2 hover:bg-sand-100 rounded cursor-pointer">
          <X size={20} />
        </button>
      </div>

      <div className="space-y-6">
        {/* Question type */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">Question Type</label>
          <select
            value={localQuestion.question.questionType}
            onChange={(e) => updateField('questionType', e.target.value as QuestionType)}
            className="w-full p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
          >
            {QUESTION_TYPE_OPTIONS.map(opt => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>

        {/* Question text */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">Question Text</label>
          <div className="flex items-center gap-2 mb-2">
            <Button variant="ghost" size="sm"><Bold size={14} /></Button>
            <Button variant="ghost" size="sm"><Italic size={14} /></Button>
            <Button variant="ghost" size="sm"><Underline size={14} /></Button>
            <Button variant="ghost" size="sm"><LinkIcon size={14} /></Button>
            <Button variant="ghost" size="sm"><ImageIcon size={14} /></Button>
            <Button variant="ghost" size="sm"><Code size={14} /></Button>
          </div>
          <textarea
            value={localQuestion.question.questionText}
            onChange={(e) => updateField('questionText', e.target.value)}
            placeholder="Enter your question here..."
            className="w-full h-32 p-4 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 resize-none"
          />
        </div>

        {/* Options for MCQ/TrueFalse */}
        {(localQuestion.question.questionType === 'MultipleChoice' || 
          localQuestion.question.questionType === 'TrueFalse') && (
          <div>
            <label className="block text-sm font-medium text-ink mb-2">
              Answer Options (select the correct one)
            </label>
            <div className="space-y-3">
              {localQuestion.question.options?.map((option, idx) => (
                <div key={option.id} className="flex items-center gap-3">
                  <input
                    type="radio"
                    name="correct-answer"
                    checked={option.isCorrect}
                    onChange={() => {
                      localQuestion.question.options?.forEach(opt => {
                        updateOption(opt.id, 'isCorrect', opt.id === option.id);
                      });
                    }}
                    className="w-4 h-4 text-success"
                  />
                  <input
                    type="text"
                    value={option.text}
                    onChange={(e) => updateOption(option.id, 'text', e.target.value)}
                    placeholder={`Option ${String.fromCharCode(65 + idx)}`}
                    className="flex-1 p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
                  />
                  {localQuestion.question.options!.length > 2 && (
                    <button
                      onClick={() => removeOption(option.id)}
                      className="p-2 hover:bg-danger-light rounded text-danger cursor-pointer"
                    >
                      <Trash2 size={16} />
                    </button>
                  )}
                </div>
              ))}
            </div>
            {localQuestion.question.options!.length < 6 && (
              <Button variant="outline" size="sm" onClick={addOption} className="mt-3">
                <Plus size={14} /> Add Option
              </Button>
            )}
          </div>
        )}

        {/* Points */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">Points</label>
          <input
            type="number"
            value={localQuestion.points}
            onChange={(e) => onUpdate({ ...localQuestion, points: parseInt(e.target.value) || 0 })}
            className="w-32 p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
          />
        </div>

        {/* Difficulty */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">Difficulty</label>
          <select
            value={localQuestion.question.difficulty}
            onChange={(e) => updateField('difficulty', e.target.value)}
            className="w-full p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
          >
            <option value="Easy">Easy</option>
            <option value="Medium">Medium</option>
            <option value="Hard">Hard</option>
          </select>
        </div>

        {/* Explanation */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">
            Explanation (shown after submission)
          </label>
          <textarea
            value={localQuestion.question.explanation || ''}
            onChange={(e) => updateField('explanation', e.target.value)}
            placeholder="Explain why the correct answer is correct..."
            className="w-full h-24 p-4 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 resize-none"
          />
        </div>

        {/* Tags */}
        <div>
          <label className="block text-sm font-medium text-ink mb-2">Tags</label>
          <Input
            value={localQuestion.question.tags?.join(', ') || ''}
            onChange={(e) => updateField('tags', e.target.value.split(',').map(t => t.trim()))}
            placeholder="e.g., BST, Complexity, Week 5"
          />
        </div>
      </div>
    </Card>
  );
}

// Assessment Settings Component
function AssessmentSettings({
  assessment,
  onChange,
}: {
  assessment: Assessment;
  onChange: (a: Assessment) => void;
}) {
  return (
    <div className="max-w-3xl space-y-6">
      {/* Basic Info */}
      <Card className="p-6">
        <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink mb-4">
          Basic Information
        </h3>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Title</label>
            <Input
              value={assessment.title}
              onChange={(e) => onChange({ ...assessment, title: e.target.value })}
              placeholder="Assessment Title"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Description</label>
            <textarea
              value={assessment.description || ''}
              onChange={(e) => onChange({ ...assessment, description: e.target.value })}
              placeholder="Instructions for students..."
              className="w-full h-24 p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300 resize-none"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Type</label>
            <select
              value={assessment.assessmentType}
              onChange={(e) => onChange({ ...assessment, assessmentType: e.target.value as any })}
              className="w-full p-3 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
            >
              <option value="Quiz">Quiz</option>
              <option value="Exam">Exam</option>
              <option value="Assignment">Assignment</option>
              <option value="Practice">Practice</option>
            </select>
          </div>
        </div>
      </Card>

      {/* Timing */}
      <Card className="p-6">
        <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink mb-4">
          Timing & Availability
        </h3>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Time Limit (minutes)</label>
            <Input
              type="number"
              value={assessment.timeLimitMinutes || ''}
              onChange={(e) => onChange({ ...assessment, timeLimitMinutes: parseInt(e.target.value) || undefined })}
              placeholder="e.g., 45"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Passing Score (%)</label>
            <Input
              type="number"
              min="0"
              max="100"
              value={assessment.passingScore}
              onChange={(e) => onChange({ ...assessment, passingScore: parseInt(e.target.value) || 0 })}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Available From</label>
            <Input
              type="datetime-local"
              value={assessment.startTime || ''}
              onChange={(e) => onChange({ ...assessment, startTime: e.target.value })}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Due Date</label>
            <Input
              type="datetime-local"
              value={assessment.dueDate || ''}
              onChange={(e) => onChange({ ...assessment, dueDate: e.target.value })}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-ink mb-2">Available Until</label>
            <Input
              type="datetime-local"
              value={assessment.endTime || ''}
              onChange={(e) => onChange({ ...assessment, endTime: e.target.value })}
            />
          </div>
        </div>
      </Card>

      {/* Advanced Settings */}
      <Card className="p-6">
        <h3 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink mb-4">
          Advanced Settings
        </h3>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Shuffle Questions</div>
              <div className="text-xs text-ink-tertiary">Randomize question order for each student</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.shuffleQuestions}
              onChange={(e) => onChange({ ...assessment, shuffleQuestions: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Shuffle Options</div>
              <div className="text-xs text-ink-tertiary">Randomize answer choices for multiple choice</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.shuffleOptions}
              onChange={(e) => onChange({ ...assessment, shuffleOptions: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Show Results</div>
              <div className="text-xs text-ink-tertiary">Show scores and feedback to students</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.showResults}
              onChange={(e) => onChange({ ...assessment, showResults: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Show Results Immediately</div>
              <div className="text-xs text-ink-tertiary">Display results right after submission</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.showResultsImmediately}
              onChange={(e) => onChange({ ...assessment, showResultsImmediately: e.target.checked })}
              disabled={!assessment.showResults}
              className="w-5 h-5 text-brand-600 rounded disabled:opacity-50"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Allow Retries</div>
              <div className="text-xs text-ink-tertiary">Let students retake the assessment</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.allowRetries}
              onChange={(e) => onChange({ ...assessment, allowRetries: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          {assessment.allowRetries && (
            <div>
              <label className="block text-sm font-medium text-ink mb-2">Max Retries</label>
              <Input
                type="number"
                min="1"
                value={assessment.maxRetries || 2}
                onChange={(e) => onChange({ ...assessment, maxRetries: parseInt(e.target.value) || 2 })}
                className="w-32"
              />
            </div>
          )}
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Require Lockdown Browser</div>
              <div className="text-xs text-ink-tertiary">Prevent students from opening other tabs</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.requireLockdownBrowser}
              onChange={(e) => onChange({ ...assessment, requireLockdownBrowser: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <div className="text-sm font-medium text-ink">Allow Late Submission</div>
              <div className="text-xs text-ink-tertiary">Accept submissions after the due date</div>
            </div>
            <input
              type="checkbox"
              checked={assessment.allowLateSubmission}
              onChange={(e) => onChange({ ...assessment, allowLateSubmission: e.target.checked })}
              className="w-5 h-5 text-brand-600 rounded"
            />
          </div>
          {assessment.allowLateSubmission && (
            <div>
              <label className="block text-sm font-medium text-ink mb-2">Late Penalty (%)</label>
              <Input
                type="number"
                min="0"
                max="100"
                value={assessment.latePenaltyPercent}
                onChange={(e) => onChange({ ...assessment, latePenaltyPercent: parseInt(e.target.value) || 0 })}
                className="w-32"
              />
            </div>
          )}
        </div>
      </Card>
    </div>
  );
}

// Question Bank Modal
function QuestionBankModal({
  onClose,
  onSelect,
}: {
  onClose: () => void;
  onSelect: (questions: Question[]) => void;
}) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedQuestions, setSelectedQuestions] = useState<string[]>([]);
  const [filterType, setFilterType] = useState<QuestionType | 'all'>('all');

  // Mock question bank data
  const mockQuestions: Question[] = [
    {
      id: 'qb1',
      bankId: 'bank-1',
      questionText: 'What is the time complexity of binary search?',
      questionType: 'MultipleChoice',
      options: [
        { id: 'o1', text: 'O(n)', isCorrect: false },
        { id: 'o2', text: 'O(log n)', isCorrect: true },
        { id: 'o3', text: 'O(n²)', isCorrect: false },
      ],
      correctAnswer: 'o2',
      difficulty: 'Easy',
      tags: ['Search', 'Complexity'],
      points: 5,
    },
    {
      id: 'qb2',
      bankId: 'bank-1',
      questionText: 'Define polymorphism in object-oriented programming.',
      questionType: 'LongAnswer',
      correctAnswer: '',
      difficulty: 'Medium',
      tags: ['OOP', 'Theory'],
      points: 10,
    },
  ];

  const filteredQuestions = mockQuestions.filter(q => {
    if (filterType !== 'all' && q.questionType !== filterType) return false;
    if (searchQuery && !q.questionText.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    return true;
  });

  const toggleSelect = (questionId: string) => {
    setSelectedQuestions(prev =>
      prev.includes(questionId)
        ? prev.filter(id => id !== questionId)
        : [...prev, questionId]
    );
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.9, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.9, opacity: 0 }}
        onClick={e => e.stopPropagation()}
        className="bg-white rounded-xl w-full max-w-4xl max-h-[80vh] overflow-hidden flex flex-col"
      >
        {/* Header */}
        <div className="p-6 border-b border-sand-200 flex items-center justify-between">
          <div>
            <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink">
              Question Bank
            </h2>
            <p className="text-sm text-ink-tertiary">
              Select questions to add to your assessment
            </p>
          </div>
          <button onClick={onClose} className="p-2 hover:bg-sand-100 rounded cursor-pointer">
            <X size={20} />
          </button>
        </div>

        {/* Filters */}
        <div className="p-4 border-b border-sand-200 flex gap-3">
          <div className="relative flex-1">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <Input
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search questions..."
              className="pl-10"
            />
          </div>
          <select
            value={filterType}
            onChange={(e) => setFilterType(e.target.value as any)}
            className="px-4 py-2 border border-sand-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-brand-300"
          >
            <option value="all">All Types</option>
            {QUESTION_TYPE_OPTIONS.map(opt => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>

        {/* Question list */}
        <div className="flex-1 overflow-y-auto p-4">
          <div className="space-y-3">
            {filteredQuestions.map(q => (
              <Card
                key={q.id}
                className={`p-4 cursor-pointer transition-all ${
                  selectedQuestions.includes(q.id)
                    ? 'ring-2 ring-brand-300 bg-brand-50'
                    : 'hover:bg-sand-50'
                }`}
                onClick={() => toggleSelect(q.id)}
              >
                <div className="flex items-start gap-3">
                  <div className={`w-5 h-5 rounded border-2 flex items-center justify-center mt-0.5 ${
                    selectedQuestions.includes(q.id)
                      ? 'border-brand-500 bg-brand-500'
                      : 'border-sand-300'
                  }`}>
                    {selectedQuestions.includes(q.id) && <Check size={14} className="text-white" />}
                  </div>
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-2">
                      <Badge variant="default">{q.questionType.replace(/([A-Z])/g, ' $1').trim()}</Badge>
                      <Badge variant="accent">{q.difficulty}</Badge>
                      <span className="text-xs text-ink-tertiary">{q.points} points</span>
                    </div>
                    <p className="text-sm text-ink mb-2">{q.questionText}</p>
                    {q.tags && q.tags.length > 0 && (
                      <div className="flex gap-1 flex-wrap">
                        {q.tags.map(tag => (
                          <span key={tag} className="text-xs px-2 py-0.5 bg-sand-100 rounded text-ink-tertiary">
                            #{tag}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              </Card>
            ))}
          </div>
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-sand-200 flex items-center justify-between">
          <div className="text-sm text-ink-tertiary">
            {selectedQuestions.length} question(s) selected
          </div>
          <div className="flex gap-3">
            <Button variant="outline" onClick={onClose}>Cancel</Button>
            <Button
              onClick={() => {
                const selected = mockQuestions.filter(q => selectedQuestions.includes(q.id));
                onSelect(selected);
              }}
              disabled={selectedQuestions.length === 0}
            >
              Add {selectedQuestions.length} Question(s)
            </Button>
          </div>
        </div>
      </motion.div>
    </motion.div>
  );
}
