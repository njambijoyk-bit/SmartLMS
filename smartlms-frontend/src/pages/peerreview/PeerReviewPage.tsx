import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Users, Star, Clock, Eye,
  FileText,
  ThumbsUp, Scale, ArrowLeftRight,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type ReviewStatus = 'pending' | 'reviewing' | 'submitted' | 'calibrated';
type AssignmentPhase = 'submission' | 'review' | 'calibration' | 'results';
type Tab = 'to_review' | 'my_submissions' | 'tutor_hub';

interface PeerReviewAssignment {
  id: string;
  title: string;
  course: string;
  courseCode: string;
  phase: AssignmentPhase;
  dueDate: string;
  submissionCount: number;
  totalStudents: number;
  peersPerStudent: number;
  rubricCriteria: number;
  weightWork: number;
  weightReview: number;
}

interface ReviewTask {
  id: string;
  assignmentTitle: string;
  courseCode: string;
  peerIndex: number;
  totalPeers: number;
  status: ReviewStatus;
  dueDate: string;
  rubricCriteria: string[];
  myScore?: number;
  calibrationAdjustment?: string;
}

interface TutorProfile {
  id: string;
  name: string;
  programme: string;
  subjects: string[];
  rating: number;
  sessions: number;
  available: boolean;
  hourlyRate?: string;
}

const ASSIGNMENTS: PeerReviewAssignment[] = [
  { id: '1', title: 'Database Schema Design', course: 'Database Systems', courseCode: 'CS302', phase: 'review', dueDate: 'Apr 8, 2026', submissionCount: 185, totalStudents: 198, peersPerStudent: 3, rubricCriteria: 4, weightWork: 70, weightReview: 30 },
  { id: '2', title: 'Algorithm Analysis Report', course: 'Data Structures & Algorithms', courseCode: 'CS301', phase: 'results', dueDate: 'Mar 25, 2026', submissionCount: 142, totalStudents: 145, peersPerStudent: 3, rubricCriteria: 5, weightWork: 70, weightReview: 30 },
  { id: '3', title: 'Network Protocol Analysis', course: 'Computer Networks', courseCode: 'CS305', phase: 'submission', dueDate: 'Apr 15, 2026', submissionCount: 34, totalStudents: 89, peersPerStudent: 2, rubricCriteria: 3, weightWork: 80, weightReview: 20 },
];

const REVIEW_TASKS: ReviewTask[] = [
  { id: '1', assignmentTitle: 'Database Schema Design', courseCode: 'CS302', peerIndex: 1, totalPeers: 3, status: 'submitted', dueDate: 'Apr 8', rubricCriteria: ['Normalization', 'Relationships', 'Documentation', 'Efficiency'], myScore: 78 },
  { id: '2', assignmentTitle: 'Database Schema Design', courseCode: 'CS302', peerIndex: 2, totalPeers: 3, status: 'reviewing', dueDate: 'Apr 8', rubricCriteria: ['Normalization', 'Relationships', 'Documentation', 'Efficiency'] },
  { id: '3', assignmentTitle: 'Database Schema Design', courseCode: 'CS302', peerIndex: 3, totalPeers: 3, status: 'pending', dueDate: 'Apr 8', rubricCriteria: ['Normalization', 'Relationships', 'Documentation', 'Efficiency'] },
  { id: '4', assignmentTitle: 'Algorithm Analysis Report', courseCode: 'CS301', peerIndex: 1, totalPeers: 3, status: 'calibrated', dueDate: 'Mar 25', rubricCriteria: ['Complexity', 'Proofs', 'Examples', 'Clarity', 'Originality'], myScore: 82, calibrationAdjustment: '+3 (slightly harsh)' },
  { id: '5', assignmentTitle: 'Algorithm Analysis Report', courseCode: 'CS301', peerIndex: 2, totalPeers: 3, status: 'calibrated', dueDate: 'Mar 25', rubricCriteria: ['Complexity', 'Proofs', 'Examples', 'Clarity', 'Originality'], myScore: 71, calibrationAdjustment: '-1 (accurate)' },
  { id: '6', assignmentTitle: 'Algorithm Analysis Report', courseCode: 'CS301', peerIndex: 3, totalPeers: 3, status: 'calibrated', dueDate: 'Mar 25', rubricCriteria: ['Complexity', 'Proofs', 'Examples', 'Clarity', 'Originality'], myScore: 65, calibrationAdjustment: '+5 (generous)' },
];

const TUTORS: TutorProfile[] = [
  { id: '1', name: 'Sarah Wanjiku', programme: 'BSc Computer Science (Y4)', subjects: ['Data Structures', 'Algorithms', 'Databases'], rating: 4.8, sessions: 24, available: true },
  { id: '2', name: 'Daniel Kiprop', programme: 'BSc Computer Science (Y4)', subjects: ['Computer Networks', 'Security'], rating: 4.6, sessions: 18, available: true },
  { id: '3', name: 'Faith Njeri', programme: 'BSc Software Engineering (Y3)', subjects: ['Web Development', 'Databases'], rating: 4.9, sessions: 31, available: false },
  { id: '4', name: 'Kevin Otieno', programme: 'BSc Computer Science (Y3)', subjects: ['Mathematics', 'Algorithms'], rating: 4.3, sessions: 12, available: true },
];

const PHASE_META: Record<AssignmentPhase, { label: string; variant: 'default' | 'brand' | 'warning' | 'success' }> = {
  submission: { label: 'Submission', variant: 'default' },
  review: { label: 'Review Phase', variant: 'brand' },
  calibration: { label: 'Calibrating', variant: 'warning' },
  results: { label: 'Results', variant: 'success' },
};

const STATUS_META: Record<ReviewStatus, { label: string; variant: 'default' | 'brand' | 'success' | 'warning' }> = {
  pending: { label: 'Pending', variant: 'default' },
  reviewing: { label: 'In Progress', variant: 'brand' },
  submitted: { label: 'Submitted', variant: 'success' },
  calibrated: { label: 'Calibrated', variant: 'warning' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function PeerReviewPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('to_review');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const pendingReviews = REVIEW_TASKS.filter(r => r.status === 'pending' || r.status === 'reviewing').length;
  const completedReviews = REVIEW_TASKS.filter(r => r.status === 'submitted' || r.status === 'calibrated').length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Peer Review</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isInstructor ? 'Manage peer review assignments and calibration' : 'Review peers\' work, get feedback, and find study partners'}
            </p>
          </div>
          {isInstructor && <Button size="sm"><Users size={15} /> Create Peer Assignment</Button>}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><FileText size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">To Review</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{pendingReviews}</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Completed</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{completedReviews}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Scale size={14} className="text-warning" /><span className="text-xs text-ink-tertiary">Calibration</span></div>
          <div className="text-sm font-semibold text-ink mt-1">Slightly harsh</div>
          <div className="text-[10px] text-ink-placeholder">Avg +2.7 adjustment</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Star size={14} className="text-gold-500" /><span className="text-xs text-ink-tertiary">Review Quality</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-gold-500">4.2</div>
          <div className="text-[10px] text-ink-placeholder">/ 5.0 rating</div>
        </div>
      </motion.div>

      {/* Active assignments */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
        <h2 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Active Assignments</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
          {ASSIGNMENTS.map((a, i) => {
            const phaseMeta = PHASE_META[a.phase];
            return (
              <motion.div key={a.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.12 + i * 0.04 }}>
                <Card hover padding="sm">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{a.courseCode}</span>
                    <Badge variant={phaseMeta.variant}>{phaseMeta.label}</Badge>
                  </div>
                  <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{a.title}</h3>
                  <div className="flex items-center gap-3 mt-2 text-xs text-ink-tertiary">
                    <span className="flex items-center gap-1"><Clock size={11} /> {a.dueDate}</span>
                    <span className="flex items-center gap-1"><Users size={11} /> {a.peersPerStudent} peers</span>
                  </div>
                  <div className="mt-3 flex items-center gap-2 text-xs text-ink-tertiary">
                    <span>Grade: {a.weightWork}% work + {a.weightReview}% reviews</span>
                  </div>
                  {a.phase !== 'submission' && (
                    <div className="mt-2">
                      <ProgressBar value={(a.submissionCount / a.totalStudents) * 100} size="sm" color="brand" />
                      <span className="text-[10px] text-ink-placeholder mt-0.5 inline-block">{a.submissionCount}/{a.totalStudents} submitted</span>
                    </div>
                  )}
                </Card>
              </motion.div>
            );
          })}
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.14 }}>
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5 max-w-md">
          {([
            { key: 'to_review' as Tab, label: 'To Review' },
            { key: 'my_submissions' as Tab, label: 'My Submissions' },
            { key: 'tutor_hub' as Tab, label: 'Peer Tutoring' },
          ]).map(t => (
            <button key={t.key} onClick={() => setTab(t.key)}
              className={`flex-1 px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${
                tab === t.key ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {t.label}
              {t.key === 'to_review' && pendingReviews > 0 && (
                <span className="ml-1.5 px-1.5 py-0.5 rounded-full text-[10px] font-semibold bg-danger text-white">{pendingReviews}</span>
              )}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Review tasks */}
      {tab === 'to_review' && (
        <div className="space-y-3">
          {REVIEW_TASKS.filter(r => r.status === 'pending' || r.status === 'reviewing').map((task, i) => (
            <motion.div key={task.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.04 }}>
              <Card hover padding="none">
                <div className="p-4 flex items-center gap-4">
                  <div className="w-12 h-12 rounded-xl bg-brand-50 flex flex-col items-center justify-center shrink-0">
                    <span className="text-[10px] text-brand-400 font-medium">Peer</span>
                    <span className="text-lg font-bold text-brand-600 font-[family-name:var(--font-display)] leading-none">{task.peerIndex}</span>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{task.assignmentTitle}</h3>
                      <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{task.courseCode}</span>
                    </div>
                    <div className="flex items-center gap-3 mt-1 text-xs text-ink-tertiary">
                      <span>Review {task.peerIndex} of {task.totalPeers}</span>
                      <span>·</span>
                      <span>{task.rubricCriteria.length} rubric criteria</span>
                      <span>·</span>
                      <span className="flex items-center gap-1"><Clock size={11} /> Due {task.dueDate}</span>
                    </div>
                  </div>
                  <Badge variant={STATUS_META[task.status].variant}>{STATUS_META[task.status].label}</Badge>
                  <Button size="sm">{task.status === 'reviewing' ? 'Continue' : 'Start Review'}</Button>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>
      )}

      {/* My submissions */}
      {tab === 'my_submissions' && (
        <div className="space-y-3">
          {REVIEW_TASKS.filter(r => r.status === 'submitted' || r.status === 'calibrated').map((task, i) => (
            <motion.div key={task.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.04 }}>
              <Card padding="none">
                <div className="p-4 flex items-center gap-4">
                  <div className={`w-12 h-12 rounded-xl flex flex-col items-center justify-center shrink-0 ${task.status === 'calibrated' ? 'bg-warning-light' : 'bg-success-light'}`}>
                    <span className="text-[10px] text-ink-tertiary font-medium">Peer</span>
                    <span className="text-lg font-bold font-[family-name:var(--font-display)] leading-none text-ink-secondary">{task.peerIndex}</span>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{task.assignmentTitle}</h3>
                      <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{task.courseCode}</span>
                    </div>
                    <div className="flex items-center gap-3 mt-1 text-xs text-ink-tertiary">
                      {task.myScore !== undefined && (
                        <span>Your score: <span className="font-bold text-ink-secondary">{task.myScore}%</span></span>
                      )}
                      {task.calibrationAdjustment && (
                        <span className="flex items-center gap-1 text-warning">
                          <ArrowLeftRight size={11} /> {task.calibrationAdjustment}
                        </span>
                      )}
                    </div>
                  </div>
                  <Badge variant={STATUS_META[task.status].variant}>{STATUS_META[task.status].label}</Badge>
                  <Button variant="ghost" size="sm"><Eye size={13} /> View</Button>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>
      )}

      {/* Peer tutoring hub */}
      {tab === 'tutor_hub' && (
        <div className="space-y-4">
          <Card padding="sm" className="bg-brand-50/50 border-brand-100">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm text-brand-700">
                <ThumbsUp size={14} />
                <span className="font-medium">Peer tutors are top-performing students who volunteered to help</span>
              </div>
              <Button variant="outline" size="sm">Become a Tutor</Button>
            </div>
          </Card>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            {TUTORS.map((tutor, i) => (
              <motion.div key={tutor.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.04 }}>
                <Card hover>
                  <div className="flex items-start gap-4">
                    <div className="w-12 h-12 rounded-full bg-brand-50 flex items-center justify-center shrink-0">
                      <Users size={18} className="text-brand-500" />
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center gap-2">
                        <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{tutor.name}</h3>
                        {tutor.available ? (
                          <span className="flex items-center gap-1 text-[10px] font-medium text-success"><span className="w-1.5 h-1.5 rounded-full bg-success" /> Available</span>
                        ) : (
                          <span className="text-[10px] font-medium text-ink-placeholder">Unavailable</span>
                        )}
                      </div>
                      <div className="text-xs text-ink-tertiary mt-0.5">{tutor.programme}</div>
                      <div className="flex items-center gap-1.5 mt-2 flex-wrap">
                        {tutor.subjects.map(s => (
                          <span key={s} className="text-[10px] px-1.5 py-0.5 rounded bg-sand-100 text-ink-secondary font-medium">{s}</span>
                        ))}
                      </div>
                      <div className="flex items-center gap-4 mt-2 text-xs text-ink-tertiary">
                        <span className="flex items-center gap-1"><Star size={11} className="text-gold-500" fill="currentColor" /> {tutor.rating}</span>
                        <span>{tutor.sessions} sessions</span>
                      </div>
                    </div>
                    {tutor.available && <Button variant="outline" size="sm">Book Session</Button>}
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
