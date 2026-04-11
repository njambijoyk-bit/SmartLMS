import { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Plus, X, Save, Trash2, GripVertical, ChevronDown, ChevronUp,
  FileText, HelpCircle, Type, CheckSquare, Image as ImageIcon,
  Code, Upload, AlertCircle, Settings, Eye, Copy, Search, Filter,
  Tag, Hash, BookOpen, Clock, Award, Target, Shuffle, Lock, Unlock,
  Calendar, User, Users, Globe, Database, Download, UploadCloud
} from 'lucide-react';
import { Card } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Input } from '../ui/Input';
import { Label } from '../ui/Label';
import { Textarea } from '../ui/Textarea';
import { Select } from '../ui/Select';
import { Switch } from '../ui/Switch';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/Tabs';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '../ui/Dialog';
import { Slider } from '../ui/Slider';
import { useAuth } from '../../context/AuthContext';

// Types
export type QuestionType = 'multiple_choice' | 'true_false' | 'short_answer' | 'essay' | 'code' | 'file_upload';
export type Difficulty = 'easy' | 'medium' | 'hard';

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
  correctAnswer?: string;
  explanation?: string;
  points: number;
  difficulty: Difficulty;
  tags: string[];
  mediaUrl?: string;
  codeLanguage?: string;
  testCases?: Array<{ input: string; expectedOutput: string }>;
}

interface QuestionBank {
  id: string;
  name: string;
  description?: string;
  category?: string;
  courseId?: string;
  questionCount: number;
}

interface AssessmentQuestion extends Question {
  order: number;
  assessmentId?: string;
}

interface CreateAssessmentRequest {
  title: string;
  description?: string;
  assessmentType: 'cat' | 'assignment' | 'exam';
  courseId: string;
  moduleId?: string;
  courseGroupId?: string;
  timeLimitMinutes?: number;
  passingScore?: number;
  shuffleQuestions: boolean;
  shuffleOptions: boolean;
  showResultsImmediately: boolean;
  requireLockdownBrowser: boolean;
  allowLateSubmission: boolean;
  latePenaltyPercent?: number;
  startDate?: string;
  dueDate?: string;
  maxAttempts?: number;
}

// Mock data
const MOCK_QUESTION_BANKS: QuestionBank[] = [
  { id: '1', name: 'Data Structures Fundamentals', description: 'Core DS questions', category: 'CS', courseId: 'cs301', questionCount: 45 },
  { id: '2', name: 'Database Systems', description: 'SQL and normalization', category: 'CS', courseId: 'cs302', questionCount: 32 },
  { id: '3', name: 'Discrete Mathematics', description: 'Logic and set theory', category: 'MAT', courseId: 'mat301', questionCount: 28 },
];

const MOCK_QUESTIONS: Question[] = [
  {
    id: 'q1',
    bankId: '1',
    questionText: 'What is the time complexity of binary search?',
    questionType: 'multiple_choice',
    options: [
      { id: 'o1', text: 'O(n)', isCorrect: false },
      { id: 'o2', text: 'O(log n)', isCorrect: true },
      { id: 'o3', text: 'O(n log n)', isCorrect: false },
      { id: 'o4', text: 'O(1)', isCorrect: false },
    ],
    correctAnswer: 'o2',
    explanation: 'Binary search divides the search space in half with each iteration.',
    points: 2,
    difficulty: 'medium',
    tags: ['algorithms', 'complexity'],
  },
  {
    id: 'q2',
    bankId: '1',
    questionText: 'A hash table provides O(1) average case lookup time.',
    questionType: 'true_false',
    correctAnswer: 'true',
    explanation: 'With a good hash function and low load factor, hash tables provide constant-time lookup.',
    points: 1,
    difficulty: 'easy',
    tags: ['data structures', 'hashing'],
  },
  {
    id: 'q3',
    bankId: '2',
    questionText: 'Explain the difference between INNER JOIN and LEFT JOIN in SQL.',
    questionType: 'essay',
    points: 5,
    difficulty: 'medium',
    tags: ['sql', 'joins'],
  },
];

const QUESTION_TYPE_META: Record<QuestionType, { label: string; icon: React.ReactNode; color: string }> = {
  multiple_choice: { label: 'Multiple Choice', icon: <CheckSquare size={16} />, color: 'bg-brand-500' },
  true_false: { label: 'True/False', icon: <HelpCircle size={16} />, color: 'bg-accent-400' },
  short_answer: { label: 'Short Answer', icon: <Type size={16} />, color: 'bg-info-500' },
  essay: { label: 'Essay', icon: <FileText size={16} />, color: 'bg-purple-500' },
  code: { label: 'Code', icon: <Code size={16} />, color: 'bg-gold-500' },
  file_upload: { label: 'File Upload', icon: <Upload size={16} />, color: 'bg-success-500' },
};

const DIFFICULTY_META: Record<Difficulty, { label: string; color: string }> = {
  easy: { label: 'Easy', color: 'text-success bg-success-light' },
  medium: { label: 'Medium', color: 'text-warning bg-warning-light' },
  hard: { label: 'Hard', color: 'text-danger bg-danger-light' },
};

// Components
interface QuestionBuilderProps {
  question?: Question;
  onSave: (question: Question) => void;
  onCancel: () => void;
}

export function QuestionBuilder({ question, onSave, onCancel }: QuestionBuilderProps) {
  const [questionText, setQuestionText] = useState(question?.questionText || '');
  const [questionType, setQuestionType] = useState<QuestionType>(question?.questionType || 'multiple_choice');
  const [options, setOptions] = useState<QuestionOption[]>(question?.options || []);
  const [correctAnswer, setCorrectAnswer] = useState(question?.correctAnswer || '');
  const [explanation, setExplanation] = useState(question?.explanation || '');
  const [points, setPoints] = useState(question?.points || 1);
  const [difficulty, setDifficulty] = useState<Difficulty>(question?.difficulty || 'medium');
  const [tags, setTags] = useState<string[]>(question?.tags || []);
  const [tagInput, setTagInput] = useState('');

  const addOption = () => {
    const newOption: QuestionOption = {
      id: `opt-${Date.now()}`,
      text: '',
      isCorrect: false,
    };
    setOptions([...options, newOption]);
  };

  const updateOption = (id: string, text: string) => {
    setOptions(options.map(o => o.id === id ? { ...o, text } : o));
  };

  const removeOption = (id: string) => {
    setOptions(options.filter(o => o.id !== id));
  };

  const toggleCorrect = (id: string) => {
    if (questionType === 'multiple_choice') {
      setOptions(options.map(o => ({ ...o, isCorrect: o.id === id })));
      setCorrectAnswer(id);
    } else if (questionType === 'true_false') {
      setCorrectAnswer(id);
    }
  };

  const addTag = () => {
    if (tagInput.trim() && !tags.includes(tagInput.trim())) {
      setTags([...tags, tagInput.trim()]);
      setTagInput('');
    }
  };

  const removeTag = (tag: string) => {
    setTags(tags.filter(t => t !== tag));
  };

  const handleSave = () => {
    if (!questionText.trim()) return;

    const newQuestion: Question = {
      id: question?.id || `q-${Date.now()}`,
      questionText,
      questionType,
      options: ['multiple_choice', 'true_false'].includes(questionType) ? options : undefined,
      correctAnswer,
      explanation,
      points,
      difficulty,
      tags,
    };

    onSave(newQuestion);
  };

  return (
    <div className="space-y-6">
      {/* Question Type */}
      <div>
        <Label>Question Type</Label>
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-2 mt-2">
          {(Object.keys(QUESTION_TYPE_META) as QuestionType[]).map(type => {
            const meta = QUESTION_TYPE_META[type];
            return (
              <button
                key={type}
                onClick={() => setQuestionType(type)}
                className={`p-3 rounded-lg border flex items-center gap-2 transition-all cursor-pointer ${
                  questionType === type
                    ? 'border-brand-400 bg-brand-50 text-brand-700'
                    : 'border-sand-300 hover:border-brand-300'
                }`}
              >
                {meta.icon}
                <span className="text-sm font-medium">{meta.label}</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Question Text */}
      <div>
        <Label htmlFor="question-text">Question *</Label>
        <Textarea
          id="question-text"
          value={questionText}
          onChange={e => setQuestionText(e.target.value)}
          placeholder="Enter your question here..."
          rows={3}
          className="mt-1"
        />
      </div>

      {/* Options for MCQ/TF */}
      {['multiple_choice', 'true_false'].includes(questionType) && (
        <div>
          <Label>Answer Options</Label>
          <div className="space-y-2 mt-2">
            {questionType === 'true_false' ? (
              <>
                <button
                  onClick={() => toggleCorrect('true')}
                  className={`w-full p-3 rounded-lg border flex items-center justify-between cursor-pointer ${
                    correctAnswer === 'true'
                      ? 'border-success bg-success-light'
                      : 'border-sand-300 hover:border-brand-300'
                  }`}
                >
                  <span>True</span>
                  {correctAnswer === 'true' && <CheckSquare size={18} className="text-success" />}
                </button>
                <button
                  onClick={() => toggleCorrect('false')}
                  className={`w-full p-3 rounded-lg border flex items-center justify-between cursor-pointer ${
                    correctAnswer === 'false'
                      ? 'border-success bg-success-light'
                      : 'border-sand-300 hover:border-brand-300'
                  }`}
                >
                  <span>False</span>
                  {correctAnswer === 'false' && <CheckSquare size={18} className="text-success" />}
                </button>
              </>
            ) : (
              <>
                <AnimatePresence>
                  {options.map((option, idx) => (
                    <motion.div
                      key={option.id}
                      initial={{ opacity: 0, y: -8 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: -8 }}
                      className="flex items-center gap-2"
                    >
                      <button
                        onClick={() => toggleCorrect(option.id)}
                        className={`p-2 rounded-lg border cursor-pointer shrink-0 ${
                          option.isCorrect
                            ? 'border-success bg-success-light text-success'
                            : 'border-sand-300 hover:border-brand-300'
                        }`}
                        title="Mark as correct"
                      >
                        <CheckSquare size={18} />
                      </button>
                      <Input
                        value={option.text}
                        onChange={e => updateOption(option.id, e.target.value)}
                        placeholder={`Option ${idx + 1}`}
                        className="flex-1"
                      />
                      <button
                        onClick={() => removeOption(option.id)}
                        className="p-2 rounded-lg hover:bg-danger-light text-danger cursor-pointer"
                      >
                        <Trash2 size={18} />
                      </button>
                    </motion.div>
                  ))}
                </AnimatePresence>
                <Button variant="outline" size="sm" onClick={addOption} className="w-full mt-2">
                  <Plus size={16} /> Add Option
                </Button>
              </>
            )}
          </div>
        </div>
      )}

      {/* Points & Difficulty */}
      <div className="grid grid-cols-2 gap-4">
        <div>
          <Label htmlFor="points">Points</Label>
          <Input
            id="points"
            type="number"
            min="1"
            value={points}
            onChange={e => setPoints(parseInt(e.target.value) || 1)}
            className="mt-1"
          />
        </div>
        <div>
          <Label>Difficulty</Label>
          <div className="flex gap-2 mt-2">
            {(Object.keys(DIFFICULTY_META) as Difficulty[]).map(diff => (
              <button
                key={diff}
                onClick={() => setDifficulty(diff)}
                className={`flex-1 px-3 py-2 rounded-lg text-xs font-medium border cursor-pointer ${
                  difficulty === diff
                    ? DIFFICULTY_META[diff].color + ' border-current'
                    : 'border-sand-300 hover:border-brand-300'
                }`}
              >
                {DIFFICULTY_META[diff].label}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Tags */}
      <div>
        <Label>Tags</Label>
        <div className="flex gap-2 mt-2 flex-wrap">
          {tags.map(tag => (
            <Badge key={tag} variant="default" className="cursor-pointer" onClick={() => removeTag(tag)}>
              <Tag size={12} className="mr-1" /> {tag} <X size={12} className="ml-1" />
            </Badge>
          ))}
          <Input
            value={tagInput}
            onChange={e => setTagInput(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && (e.preventDefault(), addTag())}
            placeholder="Add tag..."
            className="flex-1 min-w-[120px]"
          />
          <Button variant="outline" size="sm" onClick={addTag}>
            <Plus size={16} />
          </Button>
        </div>
      </div>

      {/* Explanation */}
      <div>
        <Label htmlFor="explanation">Explanation (Optional)</Label>
        <Textarea
          id="explanation"
          value={explanation}
          onChange={e => setExplanation(e.target.value)}
          placeholder="Provide feedback or explanation for this answer..."
          rows={2}
          className="mt-1"
        />
      </div>

      {/* Actions */}
      <div className="flex justify-end gap-2 pt-4 border-t">
        <Button variant="outline" onClick={onCancel}>
          Cancel
        </Button>
        <Button onClick={handleSave} disabled={!questionText.trim()}>
          <Save size={16} className="mr-2" /> Save Question
        </Button>
      </div>
    </div>
  );
}

interface AssessmentBuilderProps {
  assessment?: any;
  onSave: (assessment: CreateAssessmentRequest) => void;
  onCancel: () => void;
}

export function AssessmentBuilder({ assessment, onSave, onCancel }: AssessmentBuilderProps) {
  const { user } = useAuth();
  const [title, setTitle] = useState(assessment?.title || '');
  const [description, setDescription] = useState(assessment?.description || '');
  const [assessmentType, setAssessmentType] = useState<'cat' | 'assignment' | 'exam'>(
    assessment?.assessmentType || 'cat'
  );
  const [courseId, setCourseId] = useState(assessment?.courseId || '');
  const [timeLimitMinutes, setTimeLimitMinutes] = useState(assessment?.timeLimitMinutes || 0);
  const [passingScore, setPassingScore] = useState(assessment?.passingScore || 60);
  const [shuffleQuestions, setShuffleQuestions] = useState(assessment?.shuffleQuestions || false);
  const [shuffleOptions, setShuffleOptions] = useState(assessment?.shuffleOptions || false);
  const [showResultsImmediately, setShowResultsImmediately] = useState(
    assessment?.showResultsImmediately || true
  );
  const [requireLockdownBrowser, setRequireLockdownBrowser] = useState(
    assessment?.requireLockdownBrowser || false
  );
  const [allowLateSubmission, setAllowLateSubmission] = useState(
    assessment?.allowLateSubmission || false
  );
  const [latePenaltyPercent, setLatePenaltyPercent] = useState(assessment?.latePenaltyPercent || 10);
  const [startDate, setStartDate] = useState(assessment?.startDate || '');
  const [dueDate, setDueDate] = useState(assessment?.dueDate || '');
  const [maxAttempts, setMaxAttempts] = useState(assessment?.maxAttempts || 1);

  const handleSave = () => {
    if (!title.trim() || !courseId) return;

    const newAssessment: CreateAssessmentRequest = {
      title,
      description,
      assessmentType,
      courseId,
      timeLimitMinutes: timeLimitMinutes > 0 ? timeLimitMinutes : undefined,
      passingScore,
      shuffleQuestions,
      shuffleOptions,
      showResultsImmediately,
      requireLockdownBrowser,
      allowLateSubmission,
      latePenaltyPercent: allowLateSubmission ? latePenaltyPercent : undefined,
      startDate: startDate || undefined,
      dueDate: dueDate || undefined,
      maxAttempts: maxAttempts > 1 ? maxAttempts : undefined,
    };

    onSave(newAssessment);
  };

  return (
    <div className="space-y-6">
      {/* Basic Info */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <Label htmlFor="title">Assessment Title *</Label>
          <Input
            id="title"
            value={title}
            onChange={e => setTitle(e.target.value)}
            placeholder="e.g., CAT 2 - Data Structures"
            className="mt-1"
          />
        </div>
        <div>
          <Label htmlFor="type">Assessment Type *</Label>
          <Select
            id="type"
            value={assessmentType}
            onChange={e => setAssessmentType(e.target.value as 'cat' | 'assignment' | 'exam')}
            className="mt-1"
          >
            <option value="cat">CAT (Continuous Assessment Test)</option>
            <option value="assignment">Assignment</option>
            <option value="exam">Exam</option>
          </Select>
        </div>
      </div>

      <div>
        <Label htmlFor="description">Description</Label>
        <Textarea
          id="description"
          value={description}
          onChange={e => setDescription(e.target.value)}
          placeholder="Instructions for students..."
          rows={3}
          className="mt-1"
        />
      </div>

      <div>
        <Label htmlFor="course">Course *</Label>
        <Select
          id="course"
          value={courseId}
          onChange={e => setCourseId(e.target.value)}
          className="mt-1"
        >
          <option value="">Select a course...</option>
          <option value="cs301">CS301 - Data Structures & Algorithms</option>
          <option value="cs302">CS302 - Database Systems</option>
          <option value="mat301">MAT301 - Discrete Mathematics</option>
        </Select>
      </div>

      {/* Timing */}
      <Card padding="normal" className="bg-surface-raised">
        <h3 className="font-semibold text-ink mb-4 flex items-center gap-2">
          <Clock size={18} /> Timing & Availability
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <Label htmlFor="timeLimit">Time Limit (minutes)</Label>
            <Input
              id="timeLimit"
              type="number"
              min="0"
              value={timeLimitMinutes}
              onChange={e => setTimeLimitMinutes(parseInt(e.target.value) || 0)}
              className="mt-1"
              placeholder="0 = no limit"
            />
          </div>
          <div>
            <Label htmlFor="startDate">Start Date</Label>
            <Input
              id="startDate"
              type="datetime-local"
              value={startDate}
              onChange={e => setStartDate(e.target.value)}
              className="mt-1"
            />
          </div>
          <div>
            <Label htmlFor="dueDate">Due Date</Label>
            <Input
              id="dueDate"
              type="datetime-local"
              value={dueDate}
              onChange={e => setDueDate(e.target.value)}
              className="mt-1"
            />
          </div>
        </div>
      </Card>

      {/* Grading */}
      <Card padding="normal" className="bg-surface-raised">
        <h3 className="font-semibold text-ink mb-4 flex items-center gap-2">
          <Award size={18} /> Grading Settings
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <Label htmlFor="passingScore">Passing Score (%)</Label>
            <Input
              id="passingScore"
              type="number"
              min="0"
              max="100"
              value={passingScore}
              onChange={e => setPassingScore(parseInt(e.target.value) || 60)}
              className="mt-1"
            />
          </div>
          <div>
            <Label htmlFor="maxAttempts">Max Attempts</Label>
            <Input
              id="maxAttempts"
              type="number"
              min="1"
              value={maxAttempts}
              onChange={e => setMaxAttempts(parseInt(e.target.value) || 1)}
              className="mt-1"
            />
          </div>
        </div>
        <div className="mt-4">
          <Label htmlFor="latePenalty">Late Penalty (%)</Label>
          <div className="flex items-center gap-4 mt-2">
            <Switch checked={allowLateSubmission} onCheckedChange={setAllowLateSubmission} />
            <span className="text-sm text-ink-secondary">Allow late submissions</span>
          </div>
          {allowLateSubmission && (
            <Slider
              value={[latePenaltyPercent]}
              onValueChange={v => setLatePenaltyPercent(v[0])}
              min={0}
              max={100}
              step={5}
              className="mt-3"
            />
            <p className="text-xs text-ink-tertiary mt-1">
              {latePenaltyPercent}% deduction per late day
            </p>
          )}
        </div>
      </Card>

      {/* Security & Display */}
      <Card padding="normal" className="bg-surface-raised">
        <h3 className="font-semibold text-ink mb-4 flex items-center gap-2">
          <Lock size={18} /> Security & Display Options
        </h3>
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="shuffleQuestions" className="cursor-pointer">
                Shuffle Questions
              </Label>
              <p className="text-xs text-ink-tertiary">Randomize question order for each student</p>
            </div>
            <Switch
              id="shuffleQuestions"
              checked={shuffleQuestions}
              onCheckedChange={setShuffleQuestions}
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="shuffleOptions" className="cursor-pointer">
                Shuffle Options
              </Label>
              <p className="text-xs text-ink-tertiary">Randomize answer choices (MCQ only)</p>
            </div>
            <Switch
              id="shuffleOptions"
              checked={shuffleOptions}
              onCheckedChange={setShuffleOptions}
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="showResults" className="cursor-pointer">
                Show Results Immediately
              </Label>
              <p className="text-xs text-ink-tertiary">Display scores right after submission</p>
            </div>
            <Switch
              id="showResults"
              checked={showResultsImmediately}
              onCheckedChange={setShowResultsImmediately}
            />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="lockdown" className="cursor-pointer">
                Require Lockdown Browser
              </Label>
              <p className="text-xs text-ink-tertiary">Prevent tab switching and copying</p>
            </div>
            <Switch
              id="lockdown"
              checked={requireLockdownBrowser}
              onCheckedChange={setRequireLockdownBrowser}
            />
          </div>
        </div>
      </Card>

      {/* Actions */}
      <div className="flex justify-end gap-2 pt-4 border-t">
        <Button variant="outline" onClick={onCancel}>
          Cancel
        </Button>
        <Button onClick={handleSave} disabled={!title.trim() || !courseId}>
          <Save size={16} className="mr-2" /> Save Assessment
        </Button>
      </div>
    </div>
  );
}

interface QuestionBankManagerProps {
  banks: QuestionBank[];
  onSelectBank: (bankId: string) => void;
  onCreateBank: (name: string, description?: string) => void;
}

export function QuestionBankManager({ banks, onSelectBank, onCreateBank }: QuestionBankManagerProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [newBankName, setNewBankName] = useState('');
  const [newBankDescription, setNewBankDescription] = useState('');

  const filteredBanks = banks.filter(
    bank =>
      bank.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      bank.description?.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleCreate = () => {
    if (newBankName.trim()) {
      onCreateBank(newBankName, newBankDescription);
      setNewBankName('');
      setNewBankDescription('');
      setShowCreateDialog(false);
    }
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="relative flex-1 max-w-md">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
          <Input
            value={searchTerm}
            onChange={e => setSearchTerm(e.target.value)}
            placeholder="Search question banks..."
            className="pl-9"
          />
        </div>
        <Dialog open={showCreateDialog} onOpenChange={setShowCreateDialog}>
          <DialogTrigger asChild>
            <Button>
              <Plus size={16} className="mr-2" /> New Bank
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create Question Bank</DialogTitle>
            </DialogHeader>
            <div className="space-y-4 py-4">
              <div>
                <Label htmlFor="bankName">Bank Name *</Label>
                <Input
                  id="bankName"
                  value={newBankName}
                  onChange={e => setNewBankName(e.target.value)}
                  placeholder="e.g., Data Structures Fundamentals"
                  className="mt-1"
                />
              </div>
              <div>
                <Label htmlFor="bankDesc">Description</Label>
                <Textarea
                  id="bankDesc"
                  value={newBankDescription}
                  onChange={e => setNewBankDescription(e.target.value)}
                  placeholder="Brief description of this question bank..."
                  rows={3}
                  className="mt-1"
                />
              </div>
              <div className="flex justify-end gap-2">
                <Button variant="outline" onClick={() => setShowCreateDialog(false)}>
                  Cancel
                </Button>
                <Button onClick={handleCreate} disabled={!newBankName.trim()}>
                  Create Bank
                </Button>
              </div>
            </div>
          </DialogContent>
        </Dialog>
      </div>

      {/* Banks Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredBanks.map(bank => (
          <Card
            key={bank.id}
            hover
            className="cursor-pointer group"
            onClick={() => onSelectBank(bank.id)}
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 rounded-lg bg-brand-100 flex items-center justify-center">
                  <Database size={20} className="text-brand-600" />
                </div>
                <div>
                  <h3 className="font-semibold text-ink group-hover:text-brand-600 transition-colors">
                    {bank.name}
                  </h3>
                  <p className="text-sm text-ink-tertiary mt-1">{bank.description}</p>
                  <div className="flex items-center gap-3 mt-2">
                    <Badge variant="default" className="text-xs">
                      <Hash size={12} className="mr-1" /> {bank.questionCount} questions
                    </Badge>
                    {bank.category && (
                      <Badge variant="outline" className="text-xs">
                        <BookOpen size={12} className="mr-1" /> {bank.category}
                      </Badge>
                    )}
                  </div>
                </div>
              </div>
              <ChevronDown size={18} className="text-ink-tertiary group-hover:text-brand-600" />
            </div>
          </Card>
        ))}
      </div>

      {filteredBanks.length === 0 && (
        <Card className="text-center py-12">
          <Database size={36} className="mx-auto text-ink-placeholder mb-3" />
          <p className="text-ink-tertiary">No question banks found</p>
          <Button variant="link" onClick={() => setShowCreateDialog(true)} className="mt-2">
            Create your first question bank
          </Button>
        </Card>
      )}
    </div>
  );
}
