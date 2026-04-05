import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  FileCheck, Search, Plus, Clock, CheckCircle2, AlertTriangle,
  Upload, Eye, ChevronRight, FileText, Users, X,
  MoreHorizontal, Scale, Award, Star,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type RPLStatus = 'draft' | 'submitted' | 'under_review' | 'challenge_scheduled' | 'granted' | 'partial' | 'denied';
type Tab = 'my_applications' | 'review_queue';

interface RPLApplication {
  id: string;
  student: string;
  regNumber: string;
  course: string;
  courseCode: string;
  competencies: string[];
  status: RPLStatus;
  submittedDate?: string;
  evidenceItems: number;
  reflectiveStatement: boolean;
  assessor?: string;
  decision?: string;
  creditAwarded?: string;
  challengeDate?: string;
}

const APPLICATIONS: RPLApplication[] = [
  { id: '1', student: 'James Mwangi', regNumber: 'CS-2022-0042', course: 'Database Systems', courseCode: 'CS302', competencies: ['SQL Queries', 'Schema Design', 'Query Optimization'], status: 'granted', submittedDate: 'Jan 15, 2026', evidenceItems: 6, reflectiveStatement: true, assessor: 'Dr. Njoroge', decision: 'Full credit granted — 2 years industry experience at Safaricom', creditAwarded: 'Full Credit (3 units)' },
  { id: '2', student: 'James Mwangi', regNumber: 'CS-2022-0042', course: 'Computer Networks', courseCode: 'CS305', competencies: ['TCP/IP', 'Network Security', 'Routing'], status: 'under_review', submittedDate: 'Mar 1, 2026', evidenceItems: 4, reflectiveStatement: true, assessor: 'Dr. Ochieng' },
  { id: '3', student: 'James Mwangi', regNumber: 'CS-2022-0042', course: 'Software Engineering', courseCode: 'CS310', competencies: ['Agile', 'CI/CD', 'Testing'], status: 'challenge_scheduled', submittedDate: 'Feb 10, 2026', evidenceItems: 5, reflectiveStatement: true, assessor: 'Dr. Kamau', challengeDate: 'Apr 12, 2026' },
  { id: '4', student: 'Amina Hassan', regNumber: 'CS-2023-0078', course: 'Web Development', courseCode: 'CS303', competencies: ['HTML/CSS', 'JavaScript', 'React'], status: 'submitted', submittedDate: 'Mar 28, 2026', evidenceItems: 8, reflectiveStatement: true },
  { id: '5', student: 'Peter Odhiambo', regNumber: 'CS-2021-0156', course: 'Project Management', courseCode: 'BUS301', competencies: ['Scrum', 'Risk Management', 'Stakeholder Communication'], status: 'denied', submittedDate: 'Dec 5, 2025', evidenceItems: 2, reflectiveStatement: false, assessor: 'Dr. Akinyi', decision: 'Insufficient evidence — missing reflective statement and practical demonstration' },
  { id: '6', student: 'Grace Achieng', regNumber: 'IT-2024-0112', course: 'Cybersecurity Fundamentals', courseCode: 'CS320', competencies: ['Encryption', 'Network Security', 'Ethical Hacking'], status: 'partial', submittedDate: 'Feb 20, 2026', evidenceItems: 4, reflectiveStatement: true, assessor: 'Dr. Ochieng', decision: 'Partial credit — strong on encryption, insufficient evidence for ethical hacking', creditAwarded: 'Partial Credit (2 of 3 units)' },
];

const STATUS_META: Record<RPLStatus, { label: string; variant: 'default' | 'brand' | 'warning' | 'success' | 'danger' | 'info' | 'accent' }> = {
  draft: { label: 'Draft', variant: 'default' },
  submitted: { label: 'Submitted', variant: 'brand' },
  under_review: { label: 'Under Review', variant: 'warning' },
  challenge_scheduled: { label: 'Challenge Scheduled', variant: 'accent' },
  granted: { label: 'Granted', variant: 'success' },
  partial: { label: 'Partial Credit', variant: 'info' },
  denied: { label: 'Denied', variant: 'danger' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function RPLPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('my_applications');
  const [search, setSearch] = useState('');

  const isAdmin = user?.role === 'admin' || user?.role === 'instructor';
  const isStudent = user?.role === 'learner';

  const myApps = isStudent ? APPLICATIONS.filter(a => a.student === (user?.name || 'James Mwangi')) : APPLICATIONS;
  const reviewQueue = APPLICATIONS.filter(a => a.status === 'submitted' || a.status === 'under_review');

  const filtered = (tab === 'review_queue' ? reviewQueue : myApps).filter(a => {
    if (search && !a.course.toLowerCase().includes(search.toLowerCase()) && !a.student.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const stats = {
    total: myApps.length,
    granted: myApps.filter(a => a.status === 'granted' || a.status === 'partial').length,
    pending: myApps.filter(a => a.status === 'submitted' || a.status === 'under_review' || a.status === 'challenge_scheduled').length,
    denied: myApps.filter(a => a.status === 'denied').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Recognition of Prior Learning</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isAdmin ? 'Assess RPL applications and award credit for prior experience' : 'Apply for course credit based on your work experience and prior learning'}
            </p>
          </div>
          {isStudent && <Button size="sm"><Plus size={15} /> New Application</Button>}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
            <FileCheck size={18} className="text-brand-500" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.total}</div>
            <div className="text-xs text-ink-tertiary">Applications</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center shrink-0">
            <CheckCircle2 size={18} className="text-success" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.granted}</div>
            <div className="text-xs text-ink-tertiary">Credit Awarded</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center shrink-0">
            <Clock size={18} className="text-warning" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.pending}</div>
            <div className="text-xs text-ink-tertiary">Pending Review</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-danger-light flex items-center justify-center shrink-0">
            <X size={18} className="text-danger" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.denied}</div>
            <div className="text-xs text-ink-tertiary">Denied</div>
          </div>
        </div>
      </motion.div>

      {/* How RPL works */}
      {isStudent && (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
          <Card padding="sm" className="bg-brand-50/50 border-brand-100">
            <div className="flex items-center gap-6 text-xs">
              <span className="font-semibold text-brand-600">RPL Process:</span>
              <div className="flex items-center gap-2">
                <span className="px-2 py-0.5 rounded bg-surface-raised text-ink-secondary font-medium">Select Courses</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-surface-raised text-ink-secondary font-medium">Upload Evidence</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-surface-raised text-ink-secondary font-medium">Reflective Statement</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-surface-raised text-ink-secondary font-medium">Assessor Review</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-success-light text-success font-medium">Credit Awarded</span>
              </div>
            </div>
          </Card>
        </motion.div>
      )}

      {/* Tabs (admin) */}
      {isAdmin && (
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
          <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5 max-w-sm">
            <button onClick={() => setTab('my_applications')}
              className={`flex-1 px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${tab === 'my_applications' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
              All Applications
            </button>
            <button onClick={() => setTab('review_queue')}
              className={`flex-1 px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${tab === 'review_queue' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
              Review Queue
              {reviewQueue.length > 0 && <span className="ml-1.5 px-1.5 py-0.5 rounded-full text-[10px] font-semibold bg-danger text-white">{reviewQueue.length}</span>}
            </button>
          </div>
        </motion.div>
      )}

      {/* Search */}
      <div className="relative max-w-sm">
        <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
        <input type="text" placeholder="Search applications..." value={search} onChange={e => setSearch(e.target.value)}
          className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
      </div>

      {/* Applications */}
      <div className="space-y-3">
        {filtered.map((app, i) => {
          const statusMeta = STATUS_META[app.status];
          const isGranted = app.status === 'granted' || app.status === 'partial';
          const isDenied = app.status === 'denied';

          return (
            <motion.div key={app.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.04 }}>
              <Card hover={!isDenied} padding="none" className={`${isGranted ? 'border-success/30' : isDenied ? 'border-danger/20 opacity-75' : ''}`}>
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    <div className={`w-12 h-12 rounded-xl flex items-center justify-center shrink-0 ${
                      isGranted ? 'bg-success-light' : isDenied ? 'bg-danger-light' : 'bg-brand-50'
                    }`}>
                      {isGranted ? <Award size={18} className="text-success" /> :
                       isDenied ? <X size={18} className="text-danger" /> :
                       <FileCheck size={18} className="text-brand-500" />}
                    </div>

                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <div className="flex items-center gap-2 flex-wrap">
                            <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{app.course}</h3>
                            <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{app.courseCode}</span>
                          </div>
                          {isAdmin && <div className="text-xs text-ink-tertiary mt-0.5">{app.student} · {app.regNumber}</div>}
                        </div>
                        <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                      </div>

                      {/* Competencies claimed */}
                      <div className="flex items-center gap-1.5 mt-2 flex-wrap">
                        {app.competencies.map(c => (
                          <span key={c} className="text-[10px] px-1.5 py-0.5 rounded bg-sand-100 text-ink-secondary font-medium">{c}</span>
                        ))}
                      </div>

                      {/* Evidence details */}
                      <div className="flex items-center gap-4 mt-3 text-xs text-ink-tertiary flex-wrap">
                        <span className="flex items-center gap-1"><FileText size={11} /> {app.evidenceItems} evidence items</span>
                        <span className="flex items-center gap-1">
                          {app.reflectiveStatement ? <CheckCircle2 size={11} className="text-success" /> : <AlertTriangle size={11} className="text-warning" />}
                          Reflective statement {app.reflectiveStatement ? 'included' : 'missing'}
                        </span>
                        {app.submittedDate && <span className="flex items-center gap-1"><Clock size={11} /> Submitted {app.submittedDate}</span>}
                        {app.assessor && <span className="flex items-center gap-1"><Users size={11} /> Assessor: {app.assessor}</span>}
                      </div>

                      {/* Challenge assessment date */}
                      {app.challengeDate && (
                        <div className="mt-2 flex items-center gap-2 text-xs font-medium text-accent-500">
                          <Scale size={12} />
                          Challenge assessment scheduled: {app.challengeDate}
                        </div>
                      )}

                      {/* Decision */}
                      {app.decision && (
                        <div className={`mt-3 p-2.5 rounded-lg text-xs ${isGranted ? 'bg-success-light text-success' : 'bg-danger-light text-danger'}`}>
                          <span className="font-medium">Decision: </span>{app.decision}
                        </div>
                      )}

                      {/* Credit awarded */}
                      {app.creditAwarded && (
                        <div className="mt-2 flex items-center gap-2">
                          <Star size={13} className="text-gold-500" fill="currentColor" />
                          <span className="text-xs font-semibold text-ink">{app.creditAwarded}</span>
                        </div>
                      )}
                    </div>

                    <div className="flex items-center gap-2 shrink-0">
                      {isAdmin && (app.status === 'submitted' || app.status === 'under_review') && (
                        <>
                          <Button size="sm"><Eye size={14} /> Review</Button>
                          <button className="p-1.5 rounded-md hover:bg-sand-100 text-ink-tertiary cursor-pointer"><MoreHorizontal size={16} /></button>
                        </>
                      )}
                      {isStudent && app.status === 'draft' && <Button size="sm"><Upload size={14} /> Continue</Button>}
                      {!isAdmin && <ChevronRight size={14} className="text-ink-placeholder" />}
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          );
        })}
      </div>

      {filtered.length === 0 && (
        <Card className="text-center py-16">
          <FileCheck size={36} className="mx-auto text-ink-placeholder mb-3" />
          <p className="text-ink-tertiary">No RPL applications found</p>
        </Card>
      )}
    </div>
  );
}
