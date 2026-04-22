import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  ClipboardCheck, Plus, Search, Filter, Clock, CheckCircle2,
  AlertTriangle, FileText, Timer, ChevronRight,
  Play, Eye, Users, BarChart2, Lock, Unlock, MoreHorizontal,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type Tab = 'all' | 'cat' | 'assignment' | 'exam';
type AssessmentStatus = 'upcoming' | 'active' | 'submitted' | 'graded' | 'draft';

interface Assessment {
  id: string;
  title: string;
  course: string;
  courseCode: string;
  type: 'cat' | 'assignment' | 'exam';
  status: AssessmentStatus;
  dueDate: string;
  openDate?: string;
  totalMarks: number;
  duration?: number; // minutes
  submissionCount?: number;
  totalStudents?: number;
  score?: number;
  attempts?: number;
  maxAttempts?: number;
  isLocked?: boolean;
}

const ASSESSMENTS: Assessment[] = [
  { id: '1', title: 'CAT 2 — Binary Trees & Hash Tables', course: 'Data Structures & Algorithms', courseCode: 'CS301', type: 'cat', status: 'active', dueDate: 'Tomorrow, 2:00 PM', openDate: 'Today, 8:00 AM', totalMarks: 30, duration: 45, submissionCount: 67, totalStudents: 145, isLocked: false },
  { id: '2', title: 'Assignment 3 — ER Diagram Normalisation', course: 'Database Systems', courseCode: 'CS302', type: 'assignment', status: 'upcoming', dueDate: 'Apr 10, 11:59 PM', totalMarks: 20, submissionCount: 0, totalStudents: 198 },
  { id: '3', title: 'CAT 1 — Logic & Set Theory', course: 'Discrete Mathematics', courseCode: 'MAT301', type: 'cat', status: 'graded', dueDate: 'Mar 22, 2:00 PM', totalMarks: 30, duration: 45, submissionCount: 132, totalStudents: 132, score: 88 },
  { id: '4', title: 'Lab Report 5 — TCP/IP Analysis', course: 'Computer Networks', courseCode: 'CS305', type: 'assignment', status: 'upcoming', dueDate: 'Apr 12, 5:00 PM', totalMarks: 15, submissionCount: 0, totalStudents: 89 },
  { id: '5', title: 'End of Semester Exam', course: 'Discrete Mathematics', courseCode: 'MAT301', type: 'exam', status: 'upcoming', dueDate: 'May 2, 9:00 AM', totalMarks: 70, duration: 180, totalStudents: 132, isLocked: true },
  { id: '6', title: 'CAT 1 — Network Layers', course: 'Computer Networks', courseCode: 'CS305', type: 'cat', status: 'graded', dueDate: 'Mar 15, 2:00 PM', totalMarks: 30, duration: 45, submissionCount: 89, totalStudents: 89, score: 74 },
  { id: '7', title: 'Assignment 1 — Algorithm Analysis', course: 'Data Structures & Algorithms', courseCode: 'CS301', type: 'assignment', status: 'graded', dueDate: 'Mar 5, 11:59 PM', totalMarks: 20, submissionCount: 140, totalStudents: 145, score: 82 },
  { id: '8', title: 'Mid-Semester Exam', course: 'Database Systems', courseCode: 'CS302', type: 'exam', status: 'draft', dueDate: 'Apr 18, 9:00 AM', totalMarks: 70, duration: 120, totalStudents: 198, isLocked: true },
];

const STATUS_META: Record<AssessmentStatus, { label: string; variant: 'success' | 'warning' | 'danger' | 'default' | 'accent'; icon: React.ReactNode }> = {
  upcoming: { label: 'Upcoming', variant: 'default', icon: <Clock size={12} /> },
  active: { label: 'Active', variant: 'success', icon: <Play size={12} /> },
  submitted: { label: 'Submitted', variant: 'accent', icon: <CheckCircle2 size={12} /> },
  graded: { label: 'Graded', variant: 'default', icon: <CheckCircle2 size={12} /> },
  draft: { label: 'Draft', variant: 'warning', icon: <FileText size={12} /> },
};

const TYPE_META = {
  cat: { label: 'CAT', color: 'bg-brand-500', lightColor: 'bg-brand-50 text-brand-600 border border-brand-100' },
  assignment: { label: 'Assignment', color: 'bg-accent-400', lightColor: 'bg-accent-50 text-accent-600 border border-accent-100' },
  exam: { label: 'Exam', color: 'bg-gold-500', lightColor: 'bg-gold-50 text-gold-600 border border-gold-100' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };


export function AssessmentsPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('all');
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const filtered = ASSESSMENTS.filter(a => {
    if (tab !== 'all' && a.type !== tab) return false;
    if (search && !a.title.toLowerCase().includes(search.toLowerCase()) && !a.course.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const stats = {
    active: ASSESSMENTS.filter(a => a.status === 'active').length,
    upcoming: ASSESSMENTS.filter(a => a.status === 'upcoming').length,
    graded: ASSESSMENTS.filter(a => a.status === 'graded').length,
    draft: ASSESSMENTS.filter(a => a.status === 'draft').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Assessments</h1>
            <p className="text-sm text-ink-tertiary mt-1">CATs, assignments, and exams across all courses</p>
          </div>
          {isInstructor && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><FileText size={15} /> Question Bank</Button>
              <Button size="sm"><Plus size={15} /> Create Assessment</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Quick stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.08 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center shrink-0">
            <Play size={18} className="text-success" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.active}</div>
            <div className="text-xs text-ink-tertiary">Active now</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-info-light flex items-center justify-center shrink-0">
            <Clock size={18} className="text-info" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.upcoming}</div>
            <div className="text-xs text-ink-tertiary">Upcoming</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-surface-sunken flex items-center justify-center shrink-0">
            <CheckCircle2 size={18} className="text-ink-tertiary" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.graded}</div>
            <div className="text-xs text-ink-tertiary">Graded</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center shrink-0">
            <AlertTriangle size={18} className="text-warning" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.draft}</div>
            <div className="text-xs text-ink-tertiary">Drafts</div>
          </div>
        </div>
      </motion.div>

      {/* Tabs + Search */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.12 }} className="flex items-center justify-between gap-4 flex-wrap">
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
          {(['all', 'cat', 'assignment', 'exam'] as Tab[]).map(t => (
            <button
              key={t}
              onClick={() => setTab(t)}
              className={`px-4 py-1.5 text-sm font-medium rounded-lg transition-all capitalize cursor-pointer ${
                tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}
            >
              {t === 'all' ? 'All' : TYPE_META[t as 'cat' | 'assignment' | 'exam'].label + 's'}
            </button>
          ))}
        </div>
        <div className="flex gap-2 flex-1 justify-end">
          <div className="relative max-w-xs flex-1">
            <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input
              type="text"
              placeholder="Search assessments..."
              value={search}
              onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
            />
          </div>
          <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
            <Filter size={16} />
          </button>
        </div>
      </motion.div>

      {/* Assessment list */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.16 }} className="space-y-3">
        {filtered.map((assessment, i) => {
          const typeMeta = TYPE_META[assessment.type];
          const statusMeta = STATUS_META[assessment.status];
          const isActive = assessment.status === 'active';
          const isGraded = assessment.status === 'graded';

          return (
            <motion.div
              key={assessment.id}
              initial={{ opacity: 0, x: -8 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ duration: 0.25, delay: i * 0.04 }}
            >
              <Card
                hover
                className={`${isActive ? 'border-success/40 bg-success-light/20' : ''}`}
                padding="none"
              >
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    {/* Type indicator */}
                    <div className={`w-12 h-12 rounded-xl ${typeMeta.color} flex flex-col items-center justify-center shrink-0`}>
                      <span className="text-white text-[10px] font-bold font-[family-name:var(--font-display)] leading-none">
                        {typeMeta.label.toUpperCase()}
                      </span>
                    </div>

                    {/* Main content */}
                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <div className="flex items-center gap-2 flex-wrap">
                            <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{assessment.title}</h3>
                            {isActive && (
                              <span className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-success text-white text-[10px] font-bold uppercase tracking-wide animate-pulse">
                                <span className="w-1.5 h-1.5 rounded-full bg-white" /> Live
                              </span>
                            )}
                          </div>
                          <div className="flex items-center gap-2 mt-1">
                            <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${typeMeta.lightColor}`}>{assessment.courseCode}</span>
                            <span className="text-xs text-ink-tertiary">{assessment.course}</span>
                          </div>
                        </div>
                        <div className="flex items-center gap-2 shrink-0">
                          <Badge variant={statusMeta.variant}>
                            {statusMeta.label}
                          </Badge>
                          {assessment.isLocked ? (
                            <Lock size={14} className="text-ink-tertiary" />
                          ) : assessment.status === 'upcoming' && (
                            <Unlock size={14} className="text-success" />
                          )}
                        </div>
                      </div>

                      {/* Details row */}
                      <div className="flex items-center gap-5 mt-3 flex-wrap">
                        <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                          <Clock size={13} />
                          <span>Due: <span className={`font-medium ${isActive ? 'text-success' : 'text-ink-secondary'}`}>{assessment.dueDate}</span></span>
                        </div>
                        <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                          <FileText size={13} />
                          <span>{assessment.totalMarks} marks</span>
                        </div>
                        {assessment.duration && (
                          <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                            <Timer size={13} />
                            <span>{assessment.duration} min</span>
                          </div>
                        )}
                        {isInstructor && assessment.submissionCount !== undefined && (
                          <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                            <Users size={13} />
                            <span>{assessment.submissionCount}/{assessment.totalStudents} submitted</span>
                          </div>
                        )}
                        {isGraded && assessment.score !== undefined && !isInstructor && (
                          <div className="flex items-center gap-1.5">
                            <span className={`text-sm font-bold font-[family-name:var(--font-display)] ${assessment.score >= 70 ? 'text-success' : assessment.score >= 50 ? 'text-warning' : 'text-danger'}`}>
                              {assessment.score}/{assessment.totalMarks}
                            </span>
                            <span className="text-xs text-ink-tertiary">
                              ({Math.round((assessment.score / assessment.totalMarks) * 100)}%)
                            </span>
                          </div>
                        )}
                      </div>

                      {/* Progress bar for active assessments (instructor view) */}
                      {isInstructor && isActive && assessment.submissionCount !== undefined && assessment.totalStudents && (
                        <div className="mt-3">
                          <div className="h-1.5 bg-sand-200 rounded-full overflow-hidden">
                            <div
                              className="h-full bg-success rounded-full transition-all"
                              style={{ width: `${(assessment.submissionCount / assessment.totalStudents) * 100}%` }}
                            />
                          </div>
                          <p className="text-[11px] text-ink-tertiary mt-1">{Math.round((assessment.submissionCount / assessment.totalStudents) * 100)}% submitted</p>
                        </div>
                      )}
                    </div>

                    {/* Actions */}
                    <div className="flex items-center gap-2 shrink-0">
                      {isInstructor ? (
                        <>
                          <Button variant="ghost" size="sm"><BarChart2 size={14} /> Results</Button>
                          <Button variant="ghost" size="sm"><Eye size={14} /> View</Button>
                          <button className="p-1.5 rounded-md hover:bg-sand-100 text-ink-tertiary cursor-pointer">
                            <MoreHorizontal size={16} />
                          </button>
                        </>
                      ) : (
                        <>
                          {isActive && <Button size="sm"><Play size={14} /> Start</Button>}
                          {assessment.status === 'upcoming' && <Button variant="outline" size="sm"><Eye size={14} /> Preview</Button>}
                          {isGraded && <Button variant="ghost" size="sm"><Eye size={14} /> Review</Button>}
                          <ChevronRight size={16} className="text-ink-tertiary" />
                        </>
                      )}
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          );
        })}

        {filtered.length === 0 && (
          <Card className="text-center py-16">
            <ClipboardCheck size={36} className="mx-auto text-ink-placeholder mb-3" />
            <p className="text-ink-tertiary">No assessments found</p>
          </Card>
        )}
      </motion.div>
    </div>
  );
}
