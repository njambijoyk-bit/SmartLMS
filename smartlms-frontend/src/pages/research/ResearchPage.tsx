import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  BookMarked, Search, Plus, Clock, CheckCircle2, AlertTriangle,
  Upload, MessageSquare, Calendar, Eye,
  FileText, Flag, MoreHorizontal, TrendingUp,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type ResearchStatus = 'proposal' | 'active' | 'corrections' | 'completed' | 'overdue';
type MilestoneStatus = 'completed' | 'current' | 'upcoming' | 'overdue';
type Tab = 'overview' | 'milestones' | 'submissions' | 'meetings';

interface ResearchProject {
  id: string;
  title: string;
  student: string;
  regNumber: string;
  programme: string;
  supervisor: string;
  coSupervisors: string[];
  status: ResearchStatus;
  startDate: string;
  expectedEnd: string;
  progress: number;
  milestones: MilestoneItem[];
  chaptersSubmitted: number;
  totalChapters: number;
  lastInteraction: string;
  ethicsClearance: boolean;
}

interface MilestoneItem {
  id: string;
  name: string;
  dueDate: string;
  status: MilestoneStatus;
  signedOff?: boolean;
}

const PROJECTS: ResearchProject[] = [
  {
    id: '1', title: 'Machine Learning Approaches to Detecting Academic Plagiarism in Multilingual Contexts',
    student: 'James Mwangi', regNumber: 'MSC-CS-2024-012', programme: 'MSc Computer Science',
    supervisor: 'Prof. Muthoni', coSupervisors: ['Dr. Kamau'],
    status: 'active', startDate: 'Sep 2024', expectedEnd: 'Aug 2026', progress: 42,
    milestones: [
      { id: 'm1', name: 'Proposal Defence', dueDate: 'Dec 2024', status: 'completed', signedOff: true },
      { id: 'm2', name: 'Ethics Clearance', dueDate: 'Jan 2025', status: 'completed', signedOff: true },
      { id: 'm3', name: 'Literature Review', dueDate: 'Mar 2025', status: 'completed', signedOff: true },
      { id: 'm4', name: 'Data Collection', dueDate: 'Jun 2025', status: 'current' },
      { id: 'm5', name: 'Analysis & Results', dueDate: 'Dec 2025', status: 'upcoming' },
      { id: 'm6', name: 'Thesis Submission', dueDate: 'May 2026', status: 'upcoming' },
      { id: 'm7', name: 'Viva/Defence', dueDate: 'Jul 2026', status: 'upcoming' },
    ],
    chaptersSubmitted: 3, totalChapters: 6, lastInteraction: '2 days ago', ethicsClearance: true,
  },
  {
    id: '2', title: 'Blockchain-Based Credential Verification for East African Higher Education Institutions',
    student: 'Amina Hassan', regNumber: 'MSC-CS-2023-045', programme: 'MSc Computer Science',
    supervisor: 'Dr. Ochieng', coSupervisors: ['Prof. Wafula', 'Dr. Njoroge'],
    status: 'corrections', startDate: 'Sep 2023', expectedEnd: 'Aug 2025', progress: 88,
    milestones: [
      { id: 'm1', name: 'Proposal Defence', dueDate: 'Dec 2023', status: 'completed', signedOff: true },
      { id: 'm2', name: 'Ethics Clearance', dueDate: 'Feb 2024', status: 'completed', signedOff: true },
      { id: 'm3', name: 'Literature Review', dueDate: 'Apr 2024', status: 'completed', signedOff: true },
      { id: 'm4', name: 'Data Collection', dueDate: 'Aug 2024', status: 'completed', signedOff: true },
      { id: 'm5', name: 'Analysis & Results', dueDate: 'Dec 2024', status: 'completed', signedOff: true },
      { id: 'm6', name: 'Thesis Submission', dueDate: 'Mar 2025', status: 'completed', signedOff: true },
      { id: 'm7', name: 'Viva/Defence', dueDate: 'Apr 2025', status: 'current' },
    ],
    chaptersSubmitted: 6, totalChapters: 6, lastInteraction: '1 day ago', ethicsClearance: true,
  },
  {
    id: '3', title: 'IoT-Based Water Quality Monitoring in Rural Kenya',
    student: 'Peter Odhiambo', regNumber: 'PHD-CS-2022-008', programme: 'PhD Computer Science',
    supervisor: 'Prof. Muthoni', coSupervisors: ['Dr. Akinyi'],
    status: 'overdue', startDate: 'Sep 2022', expectedEnd: 'Aug 2025', progress: 35,
    milestones: [
      { id: 'm1', name: 'Proposal Defence', dueDate: 'Mar 2023', status: 'completed', signedOff: true },
      { id: 'm2', name: 'Ethics Clearance', dueDate: 'Jun 2023', status: 'completed', signedOff: true },
      { id: 'm3', name: 'Literature Review', dueDate: 'Sep 2023', status: 'completed', signedOff: true },
      { id: 'm4', name: 'Prototype Development', dueDate: 'Mar 2024', status: 'overdue' },
      { id: 'm5', name: 'Field Deployment', dueDate: 'Sep 2024', status: 'upcoming' },
      { id: 'm6', name: 'Data Analysis', dueDate: 'Mar 2025', status: 'upcoming' },
      { id: 'm7', name: 'Thesis Writing', dueDate: 'Jun 2025', status: 'upcoming' },
    ],
    chaptersSubmitted: 2, totalChapters: 7, lastInteraction: '3 weeks ago', ethicsClearance: true,
  },
  {
    id: '4', title: 'Natural Language Processing for Swahili Sentiment Analysis in Social Media',
    student: 'Grace Achieng', regNumber: 'MSC-CS-2024-067', programme: 'MSc Computer Science',
    supervisor: 'Dr. Kamau', coSupervisors: [],
    status: 'proposal', startDate: 'Sep 2024', expectedEnd: 'Aug 2026', progress: 8,
    milestones: [
      { id: 'm1', name: 'Proposal Defence', dueDate: 'Feb 2025', status: 'current' },
      { id: 'm2', name: 'Ethics Clearance', dueDate: 'Apr 2025', status: 'upcoming' },
      { id: 'm3', name: 'Literature Review', dueDate: 'Jul 2025', status: 'upcoming' },
    ],
    chaptersSubmitted: 0, totalChapters: 6, lastInteraction: '5 days ago', ethicsClearance: false,
  },
];

const STATUS_META: Record<ResearchStatus, { label: string; variant: 'success' | 'brand' | 'warning' | 'danger' | 'default' }> = {
  proposal: { label: 'Proposal', variant: 'default' },
  active: { label: 'Active', variant: 'brand' },
  corrections: { label: 'Corrections', variant: 'warning' },
  completed: { label: 'Completed', variant: 'success' },
  overdue: { label: 'Overdue', variant: 'danger' },
};

const MILESTONE_COLORS: Record<MilestoneStatus, string> = {
  completed: 'bg-success',
  current: 'bg-brand-500',
  upcoming: 'bg-sand-300',
  overdue: 'bg-danger',
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ResearchPage() {
  const { user } = useAuth();
  const [activeTab, setActiveTab] = useState<Tab>('overview');
  const [selectedProject, setSelectedProject] = useState<string>(PROJECTS[0].id);
  const [search, setSearch] = useState('');

  const isStudent = user?.role === 'learner';
  const isSupervisor = user?.role === 'instructor' || user?.role === 'admin';

  const project = PROJECTS.find(p => p.id === selectedProject) || PROJECTS[0];

  const stats = {
    total: PROJECTS.length,
    active: PROJECTS.filter(p => p.status === 'active').length,
    overdue: PROJECTS.filter(p => p.status === 'overdue').length,
    corrections: PROJECTS.filter(p => p.status === 'corrections').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Research & Supervision</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isSupervisor ? 'Manage postgraduate research supervision and milestone tracking' : 'Track your research progress and supervisor interactions'}
            </p>
          </div>
          {isSupervisor && <Button size="sm"><Plus size={15} /> New Project</Button>}
        </div>
      </motion.div>

      {/* Stats (Supervisor) */}
      {isSupervisor && (
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
              <BookMarked size={18} className="text-brand-500" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.total}</div>
              <div className="text-xs text-ink-tertiary">Supervisees</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center shrink-0">
              <TrendingUp size={18} className="text-success" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.active}</div>
              <div className="text-xs text-ink-tertiary">Active</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-danger-light flex items-center justify-center shrink-0">
              <AlertTriangle size={18} className="text-danger" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.overdue}</div>
              <div className="text-xs text-ink-tertiary">Overdue</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center shrink-0">
              <Flag size={18} className="text-warning" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.corrections}</div>
              <div className="text-xs text-ink-tertiary">Corrections</div>
            </div>
          </div>
        </motion.div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Projects list (sidebar) */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="lg:col-span-1 space-y-3">
          <div className="relative mb-3">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search projects..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
          </div>

          {PROJECTS.filter(p => !search || p.title.toLowerCase().includes(search.toLowerCase()) || p.student.toLowerCase().includes(search.toLowerCase())).map((p, i) => {
            const statusMeta = STATUS_META[p.status];
            const isSelected = p.id === selectedProject;
            return (
              <motion.div key={p.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: 0.12 + i * 0.04 }}>
                <button
                  onClick={() => setSelectedProject(p.id)}
                  className={`w-full text-left p-4 rounded-xl border-2 transition-all cursor-pointer ${
                    isSelected ? 'border-brand-300 bg-brand-50/50 shadow-sm' : 'border-sand-200 bg-surface-raised hover:border-brand-200'
                  }`}
                >
                  <div className="flex items-start justify-between gap-2 mb-2">
                    <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] line-clamp-2 leading-snug">{p.title}</h3>
                    <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                  </div>
                  <div className="text-xs text-ink-tertiary">{p.student} · {p.programme}</div>
                  <div className="mt-2 flex items-center gap-2">
                    <div className="flex-1"><ProgressBar value={p.progress} size="sm" color={p.status === 'overdue' ? 'danger' : 'brand'} /></div>
                    <span className="text-xs font-bold text-ink-secondary">{p.progress}%</span>
                  </div>
                </button>
              </motion.div>
            );
          })}
        </motion.div>

        {/* Project detail */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.14 }} className="lg:col-span-2 space-y-4">
          {/* Project header card */}
          <Card padding="none">
            <div className="border-pattern" />
            <div className="p-6">
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h2 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink leading-snug">{project.title}</h2>
                  <div className="flex items-center gap-3 mt-2">
                    <Badge variant={STATUS_META[project.status].variant} size="md">{STATUS_META[project.status].label}</Badge>
                    {project.ethicsClearance && <span className="flex items-center gap-1 text-xs text-success font-medium"><CheckCircle2 size={12} /> Ethics Cleared</span>}
                  </div>
                </div>
                <Button variant="ghost" size="sm"><MoreHorizontal size={16} /></Button>
              </div>

              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 text-xs">
                <div>
                  <span className="text-ink-tertiary">Student</span>
                  <div className="font-semibold text-ink mt-0.5">{project.student}</div>
                  <div className="text-ink-placeholder">{project.regNumber}</div>
                </div>
                <div>
                  <span className="text-ink-tertiary">Supervisor</span>
                  <div className="font-semibold text-ink mt-0.5">{project.supervisor}</div>
                  {project.coSupervisors.length > 0 && (
                    <div className="text-ink-placeholder">+ {project.coSupervisors.join(', ')}</div>
                  )}
                </div>
                <div>
                  <span className="text-ink-tertiary">Timeline</span>
                  <div className="font-semibold text-ink mt-0.5">{project.startDate} — {project.expectedEnd}</div>
                </div>
                <div>
                  <span className="text-ink-tertiary">Chapters</span>
                  <div className="font-semibold text-ink mt-0.5">{project.chaptersSubmitted} / {project.totalChapters} submitted</div>
                </div>
              </div>

              <div className="mt-4">
                <div className="flex items-center justify-between mb-1.5">
                  <span className="text-xs text-ink-tertiary">Overall progress</span>
                  <span className="text-xs font-bold text-brand-500">{project.progress}%</span>
                </div>
                <ProgressBar value={project.progress} size="md" color={project.status === 'overdue' ? 'danger' : 'brand'} />
              </div>
            </div>
          </Card>

          {/* Tabs */}
          <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
            {(['overview', 'milestones', 'submissions', 'meetings'] as Tab[]).map(t => (
              <button key={t} onClick={() => setActiveTab(t)}
                className={`flex-1 px-4 py-1.5 text-sm font-medium rounded-lg transition-all capitalize cursor-pointer ${
                  activeTab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
                }`}>
                {t}
              </button>
            ))}
          </div>

          {/* Milestones timeline */}
          {(activeTab === 'overview' || activeTab === 'milestones') && (
            <Card>
              <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Milestone Timeline</h3>
              <div className="space-y-0">
                {project.milestones.map((ms, i) => (
                  <div key={ms.id} className="flex items-start gap-4 relative">
                    {/* Timeline line */}
                    {i < project.milestones.length - 1 && (
                      <div className="absolute left-[11px] top-6 w-0.5 h-full bg-sand-200" />
                    )}
                    {/* Node */}
                    <div className={`w-6 h-6 rounded-full flex items-center justify-center shrink-0 relative z-10 ${MILESTONE_COLORS[ms.status]}`}>
                      {ms.status === 'completed' ? <CheckCircle2 size={12} className="text-white" /> :
                       ms.status === 'current' ? <div className="w-2 h-2 rounded-full bg-white" /> :
                       ms.status === 'overdue' ? <AlertTriangle size={10} className="text-white" /> :
                       <div className="w-2 h-2 rounded-full bg-white/50" />}
                    </div>
                    {/* Content */}
                    <div className="flex-1 pb-6">
                      <div className="flex items-center justify-between">
                        <div>
                          <span className={`text-sm font-medium ${ms.status === 'overdue' ? 'text-danger' : ms.status === 'current' ? 'text-brand-600' : 'text-ink'}`}>{ms.name}</span>
                          {ms.signedOff && <CheckCircle2 size={12} className="inline ml-1.5 text-success" />}
                        </div>
                        <span className={`text-xs ${ms.status === 'overdue' ? 'text-danger font-medium' : 'text-ink-tertiary'}`}>{ms.dueDate}</span>
                      </div>
                      {ms.status === 'current' && (
                        <div className="mt-1.5 flex gap-2">
                          <Button variant="outline" size="sm"><Upload size={12} /> Upload Draft</Button>
                          <Button variant="ghost" size="sm"><MessageSquare size={12} /> Comment</Button>
                        </div>
                      )}
                      {ms.status === 'overdue' && (
                        <span className="text-[11px] text-danger mt-1 inline-block">Overdue — escalation alert sent</span>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* Submissions tab */}
          {activeTab === 'submissions' && (
            <Card>
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink">Chapter Submissions</h3>
                <Button size="sm"><Upload size={14} /> Upload Chapter</Button>
              </div>
              <div className="space-y-3">
                {['Chapter 1: Introduction', 'Chapter 2: Literature Review', 'Chapter 3: Methodology'].map((ch, i) => (
                  <div key={i} className="flex items-center gap-4 p-3 rounded-lg bg-surface-sunken">
                    <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
                      <FileText size={16} className="text-brand-500" />
                    </div>
                    <div className="flex-1">
                      <div className="text-sm font-medium text-ink">{ch}</div>
                      <div className="text-xs text-ink-tertiary">Version 3 · Submitted {3 - i} weeks ago</div>
                    </div>
                    <Badge variant="success">Approved</Badge>
                    <Button variant="ghost" size="sm"><Eye size={13} /></Button>
                  </div>
                ))}
                {project.chaptersSubmitted < project.totalChapters && (
                  <div className="flex items-center gap-4 p-3 rounded-lg border border-dashed border-sand-300">
                    <div className="w-10 h-10 rounded-lg bg-sand-100 flex items-center justify-center shrink-0">
                      <FileText size={16} className="text-ink-placeholder" />
                    </div>
                    <div className="flex-1">
                      <div className="text-sm text-ink-tertiary">Chapter {project.chaptersSubmitted + 1} — Not yet submitted</div>
                    </div>
                  </div>
                )}
              </div>
            </Card>
          )}

          {/* Meetings tab */}
          {activeTab === 'meetings' && (
            <Card>
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink">Supervision Meetings</h3>
                <Button size="sm"><Calendar size={14} /> Schedule Meeting</Button>
              </div>
              <div className="space-y-3">
                {[
                  { date: 'Apr 2, 2026', topic: 'Data collection progress review', status: 'upcoming' },
                  { date: 'Mar 18, 2026', topic: 'Methodology chapter feedback', status: 'completed' },
                  { date: 'Mar 4, 2026', topic: 'Literature review sign-off', status: 'completed' },
                  { date: 'Feb 15, 2026', topic: 'Research instrument validation', status: 'completed' },
                ].map((meeting, i) => (
                  <div key={i} className="flex items-center gap-4 p-3 rounded-lg bg-surface-sunken">
                    <div className={`w-10 h-10 rounded-lg flex items-center justify-center shrink-0 ${meeting.status === 'upcoming' ? 'bg-brand-50' : 'bg-sand-100'}`}>
                      <Calendar size={16} className={meeting.status === 'upcoming' ? 'text-brand-500' : 'text-ink-tertiary'} />
                    </div>
                    <div className="flex-1">
                      <div className="text-sm font-medium text-ink">{meeting.topic}</div>
                      <div className="text-xs text-ink-tertiary">{meeting.date}</div>
                    </div>
                    <Badge variant={meeting.status === 'upcoming' ? 'brand' : 'default'}>{meeting.status === 'upcoming' ? 'Upcoming' : 'Done'}</Badge>
                    {meeting.status === 'completed' && <Button variant="ghost" size="sm"><FileText size={13} /> Notes</Button>}
                  </div>
                ))}
              </div>
            </Card>
          )}

          {/* Quick actions */}
          <Card padding="sm" className="bg-surface-sunken border-sand-200">
            <div className="flex items-center gap-2 text-xs text-ink-tertiary">
              <Clock size={12} />
              <span>Last interaction: {project.lastInteraction}</span>
              {isSupervisor && (
                <>
                  <span className="mx-2">·</span>
                  <button className="text-brand-500 font-medium hover:underline cursor-pointer">Send Progress Reminder</button>
                  <span className="mx-2">·</span>
                  <button className="text-brand-500 font-medium hover:underline cursor-pointer">Generate Quarterly Report</button>
                </>
              )}
            </div>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
