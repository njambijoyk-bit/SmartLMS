import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Clock, ChevronLeft, ChevronRight, Flag, Save, Send, AlertTriangle, Lock, Monitor, AlertCircle, Loader2 } from 'lucide-react';
import { Card } from '../ui/Card';
import { Button } from '../ui/Button';
import { Badge } from '../ui/Badge';
import { Textarea } from '../ui/Textarea';

interface Question {
  id: string;
  questionText: string;
  questionType: 'multiple_choice' | 'true_false' | 'short_answer' | 'essay' | 'code';
  options?: Array<{ id: string; text: string; isCorrect: boolean }>;
  points: number;
}

interface AssessmentTakerProps {
  assessmentId: string;
  title: string;
  description?: string;
  timeLimitMinutes?: number;
  questions: Question[];
  onSubmit: (answers: Record<string, any>) => void;
  onAutoSave?: (answers: Record<string, any>) => void;
  requireLockdown?: boolean;
}

export function AssessmentTaker({ assessmentId, title, description, timeLimitMinutes, questions, onSubmit, onAutoSave, requireLockdown = false }: AssessmentTakerProps) {
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [answers, setAnswers] = useState<Record<string, any>>({});
  const [flaggedQuestions, setFlaggedQuestions] = useState<Set<string>>(new Set());
  const [timeRemaining, setTimeRemaining] = useState<number>(timeLimitMinutes ? timeLimitMinutes * 60 : 0);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showConfirmSubmit, setShowConfirmSubmit] = useState(false);
  const [lockdownViolations, setLockdownViolations] = useState(0);
  const [showWarning, setShowWarning] = useState(false);
  const timerRef = useRef<NodeJS.Timeout>();
  const autoSaveRef = useRef<NodeJS.Timeout>();

  const currentQuestion = questions[currentQuestionIndex];
  const totalQuestions = questions.length;
  const answeredCount = Object.keys(answers).length;
  const progressPercent = (answeredCount / totalQuestions) * 100;

  useEffect(() => {
    if (timeLimitMinutes && timeRemaining > 0) {
      timerRef.current = setInterval(() => {
        setTimeRemaining(prev => { if (prev <= 1) { handleSubmit(); return 0; } return prev - 1; });
      }, 1000);
    }
    return () => clearInterval(timerRef.current);
  }, [timeLimitMinutes, timeRemaining]);

  useEffect(() => {
    if (onAutoSave) {
      autoSaveRef.current = setInterval(() => { onAutoSave(answers); }, 30000);
    }
    return () => clearInterval(autoSaveRef.current);
  }, [answers, onAutoSave]);

  useEffect(() => {
    if (!requireLockdown) return;
    const handleVisibilityChange = () => { if (document.hidden) { setLockdownViolations(prev => prev + 1); setShowWarning(true); setTimeout(() => setShowWarning(false), 5000); } };
    const handleBlur = () => { setLockdownViolations(prev => prev + 1); setShowWarning(true); setTimeout(() => setShowWarning(false), 5000); };
    document.addEventListener('visibilitychange', handleVisibilityChange);
    window.addEventListener('blur', handleBlur);
    return () => { document.removeEventListener('visibilitychange', handleVisibilityChange); window.removeEventListener('blur', handleBlur); };
  }, [requireLockdown]);

  const formatTime = (seconds: number) => { const hrs = Math.floor(seconds / 3600); const mins = Math.floor((seconds % 3600) / 60); const secs = seconds % 60; if (hrs > 0) { return `${hrs}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`; } return `${mins}:${secs.toString().padStart(2, '0')}`; };
  const handleAnswer = (questionId: string, answer: any) => { setAnswers(prev => ({ ...prev, [questionId]: answer })); };
  const toggleFlag = (questionId: string) => { setFlaggedQuestions(prev => { const next = new Set(prev); if (next.has(questionId)) { next.delete(questionId); } else { next.add(questionId); } return next; }); };
  const handleSubmit = async () => { setIsSubmitting(true); try { await onSubmit(answers); setShowConfirmSubmit(false); } catch (error) { console.error('Submission failed:', error); } finally { setIsSubmitting(false); } };
  const unansweredQuestions = questions.filter(q => !answers[q.id]);
  const hasUnanswered = unansweredQuestions.length > 0;

  return (
    <div className="min-h-screen bg-surface">
      <AnimatePresence>{showWarning && (<motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }} className="bg-warning text-white px-4 py-3 flex items-center justify-center gap-2"><AlertTriangle size={18} /><span className="font-medium">Warning: Tab switching detected ({lockdownViolations} violations)</span></motion.div>)}</AnimatePresence>
      <header className="sticky top-0 z-40 bg-surface border-b border-sand-200 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center gap-4">
              <div><h1 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">{title}</h1><p className="text-xs text-ink-tertiary">Question {currentQuestionIndex + 1} of {totalQuestions}</p></div>
              {timeLimitMinutes && (<div className={`flex items-center gap-2 px-3 py-1.5 rounded-lg ${timeRemaining < 300 ? 'bg-danger-light text-danger' : 'bg-sand-100 text-ink-secondary'}`}><Clock size={16} /><span className="font-mono font-semibold">{formatTime(timeRemaining)}</span></div>)}
              {requireLockdown && (<Badge variant="warning" className="flex items-center gap-1"><Lock size={12} /> Lockdown Active</Badge>)}
            </div>
            <div className="flex items-center gap-2">
              <Button variant="ghost" size="sm" onClick={() => toggleFlag(currentQuestion.id)} className={flaggedQuestions.has(currentQuestion.id) ? 'text-warning' : ''}><Flag size={16} className={flaggedQuestions.has(currentQuestion.id) ? 'fill-warning' : ''} /></Button>
              <Button variant="outline" size="sm" onClick={() => onAutoSave?.(answers)} disabled={isSubmitting}><Save size={16} className="mr-1" /> Save</Button>
              <Button size="sm" onClick={() => setShowConfirmSubmit(true)} disabled={isSubmitting}>{isSubmitting ? (<Loader2 size={16} className="animate-spin mr-2" />) : (<Send size={16} className="mr-2" />)} Submit</Button>
            </div>
          </div>
          <div className="h-1 bg-sand-200 rounded-full overflow-hidden mb-4"><div className="h-full bg-brand-500 transition-all duration-300" style={{ width: `${progressPercent}%` }} /></div>
        </div>
      </header>
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          <div className="lg:col-span-3">
            <Card padding="large" className="min-h-[500px]">
              <AnimatePresence mode="wait">
                <motion.div key={currentQuestion.id} initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -20 }} transition={{ duration: 0.2 }}>
                  <div className="flex items-start justify-between mb-6">
                    <div className="flex items-center gap-3">
                      <Badge variant="default" className="text-sm">{currentQuestion.points} points</Badge>
                      {flaggedQuestions.has(currentQuestion.id) && (<Badge variant="warning" className="flex items-center gap-1"><Flag size={12} className="fill-warning" /> Flagged</Badge>)}
                    </div>
                    <span className="text-sm text-ink-tertiary">{currentQuestion.questionType.replace('_', ' ').toUpperCase()}</span>
                  </div>
                  <p className="text-lg text-ink mb-6 leading-relaxed">{currentQuestion.questionText}</p>
                  {currentQuestion.questionType === 'multiple_choice' && (<div className="space-y-3">{currentQuestion.options?.map(option => (<label key={option.id} className={`flex items-center gap-3 p-4 rounded-xl border cursor-pointer transition-all ${answers[currentQuestion.id] === option.id ? 'border-brand-400 bg-brand-50' : 'border-sand-300 hover:border-brand-300'}`}><input type="radio" name={`question-${currentQuestion.id}`} value={option.id} checked={answers[currentQuestion.id] === option.id} onChange={e => handleAnswer(currentQuestion.id, e.target.value)} className="w-4 h-4 text-brand-600" /><span className="text-ink">{option.text}</span></label>))}</div>)}
                  {currentQuestion.questionType === 'true_false' && (<div className="grid grid-cols-2 gap-4"><button onClick={() => handleAnswer(currentQuestion.id, 'true')} className={`p-4 rounded-xl border font-medium transition-all ${answers[currentQuestion.id] === 'true' ? 'border-success bg-success-light text-success' : 'border-sand-300 hover:border-brand-300'}`}>True</button><button onClick={() => handleAnswer(currentQuestion.id, 'false')} className={`p-4 rounded-xl border font-medium transition-all ${answers[currentQuestion.id] === 'false' ? 'border-success bg-success-light text-success' : 'border-sand-300 hover:border-brand-300'}`}>False</button></div>)}
                  {(currentQuestion.questionType === 'short_answer' || currentQuestion.questionType === 'essay') && (<Textarea value={answers[currentQuestion.id] || ''} onChange={e => handleAnswer(currentQuestion.id, e.target.value)} placeholder="Type your answer here..." rows={currentQuestion.questionType === 'essay' ? 8 : 4} className="min-h-[200px]" />)}
                  {currentQuestion.questionType === 'code' && (<div><Textarea value={answers[currentQuestion.id] || ''} onChange={e => handleAnswer(currentQuestion.id, e.target.value)} placeholder="// Write your code here..." rows={12} className="font-mono text-sm min-h-[300px]" /><p className="text-xs text-ink-tertiary mt-2">Tip: Your code will be executed against test cases for auto-grading.</p></div>)}
                </motion.div>
              </AnimatePresence>
              <div className="flex items-center justify-between mt-8 pt-6 border-t">
                <Button variant="outline" onClick={() => setCurrentQuestionIndex(prev => Math.max(0, prev - 1))} disabled={currentQuestionIndex === 0}><ChevronLeft size={16} className="mr-2" /> Previous</Button>
                <div className="text-sm text-ink-tertiary">{answeredCount} of {totalQuestions} answered</div>
                <Button onClick={() => setCurrentQuestionIndex(prev => Math.min(totalQuestions - 1, prev + 1))} disabled={currentQuestionIndex === totalQuestions - 1}>Next <ChevronRight size={16} className="ml-2" /></Button>
              </div>
            </Card>
          </div>
          <div className="lg:col-span-1">
            <Card padding="normal" className="sticky top-24">
              <h3 className="font-semibold text-ink mb-4 flex items-center gap-2"><Monitor size={18} /> Questions</h3>
              <div className="grid grid-cols-3 gap-2 mb-4 text-xs">
                <div className="text-center p-2 rounded-lg bg-success-light"><div className="font-bold text-success">{answeredCount}</div><div className="text-ink-tertiary">Answered</div></div>
                <div className="text-center p-2 rounded-lg bg-sand-100"><div className="font-bold text-ink-secondary">{unansweredQuestions.length}</div><div className="text-ink-tertiary">Unanswered</div></div>
                <div className="text-center p-2 rounded-lg bg-warning-light"><div className="font-bold text-warning">{flaggedQuestions.size}</div><div className="text-ink-tertiary">Flagged</div></div>
              </div>
              <div className="grid grid-cols-5 gap-2">{questions.map((q, idx) => { const isAnswered = !!answers[q.id]; const isFlagged = flaggedQuestions.has(q.id); const isCurrent = idx === currentQuestionIndex; return (<button key={q.id} onClick={() => setCurrentQuestionIndex(idx)} className={`aspect-square rounded-lg text-sm font-medium flex items-center justify-center relative transition-all ${isCurrent ? 'ring-2 ring-brand-500 ring-offset-2' : ''} ${isAnswered ? 'bg-success text-white' : 'bg-sand-100 text-ink-secondary hover:bg-sand-200'}`}>{idx + 1}{isFlagged && (<Flag size={10} className="absolute -top-1 -right-1 text-warning fill-warning" />)}</button>); })}</div>
              <div className="mt-4 space-y-2 text-xs">
                <div className="flex items-center gap-2"><div className="w-3 h-3 rounded bg-success" /><span className="text-ink-tertiary">Answered</span></div>
                <div className="flex items-center gap-2"><div className="w-3 h-3 rounded bg-sand-200" /><span className="text-ink-tertiary">Unanswered</span></div>
                <div className="flex items-center gap-2"><Flag size={12} className="text-warning" /><span className="text-ink-tertiary">Flagged for review</span></div>
              </div>
              {hasUnanswered && (<div className="mt-4 p-3 rounded-lg bg-warning-light border border-warning/20"><div className="flex items-start gap-2"><AlertCircle size={16} className="text-warning mt-0.5 shrink-0" /><div><p className="text-xs font-medium text-warning">{unansweredQuestions.length} unanswered question(s)</p><p className="text-[11px] text-ink-tertiary mt-1">You can still submit, but make sure to review all questions.</p></div></div></div>)}
            </Card>
          </div>
        </div>
      </main>
      {showConfirmSubmit && (<div className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"><Card className="max-w-md w-full" padding="large"><div className="text-center"><div className="w-16 h-16 rounded-full bg-brand-100 flex items-center justify-center mx-auto mb-4"><Send size={32} className="text-brand-600" /></div><h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-2">Submit Assessment?</h2><p className="text-ink-secondary mb-6">{hasUnanswered ? `You have ${unansweredQuestions.length} unanswered question(s). Are you sure you want to submit?` : 'Once submitted, you cannot change your answers.'}</p><div className="flex gap-3 justify-center"><Button variant="outline" onClick={() => setShowConfirmSubmit(false)}>Continue Working</Button><Button onClick={handleSubmit} disabled={isSubmitting}>{isSubmitting ? (<><Loader2 size={16} className="animate-spin mr-2" /> Submitting...</>) : (<><Send size={16} className="mr-2" /> Yes, Submit</>)}</Button></div></div></Card></div>)}
    </div>
  );
}
