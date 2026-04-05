import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  FileStack, Search, Filter, Plus, Clock, CheckCircle2,
  Lock, Eye, Download, ChevronRight, Shield, AlertTriangle,
  BookOpen, Calendar, Users, MoreHorizontal, Archive, Send,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type PaperType = 'cat' | 'midsem' | 'endsem' | 'supplementary';
type PaperStatus = 'draft' | 'reviewed' | 'approved' | 'archived' | 'published';
type Tab = 'all' | 'cat' | 'midsem' | 'endsem' | 'supplementary';

interface ExamPaper {
  id: string;
  title: string;
  course: string;
  courseCode: string;
  academicYear: string;
  semester: number;
  paperType: PaperType;
  status: PaperStatus;
  sections: number;
  totalMarks: number;
  duration: number;
  author: string;
  reviewedBy?: string;
  approvedBy?: string;
  classification: 'restricted' | 'confidential' | 'public';
  lastModified: string;
  practiceMode?: boolean;
}

const PAPERS: ExamPaper[] = [
  { id: '1', title: 'End of Semester Examination', course: 'Data Structures & Algorithms', courseCode: 'CS301', academicYear: '2025/2026', semester: 2, paperType: 'endsem', status: 'approved', sections: 5, totalMarks: 70, duration: 180, author: 'Dr. Kamau', reviewedBy: 'Prof. Muthoni', approvedBy: 'Dean - SoC', classification: 'restricted', lastModified: '2 days ago' },
  { id: '2', title: 'CAT 2 — Binary Trees & Hash Tables', course: 'Data Structures & Algorithms', courseCode: 'CS301', academicYear: '2025/2026', semester: 2, paperType: 'cat', status: 'approved', sections: 2, totalMarks: 30, duration: 45, author: 'Dr. Kamau', reviewedBy: 'Dr. Ochieng', approvedBy: 'HoD - CS', classification: 'confidential', lastModified: '5 days ago' },
  { id: '3', title: 'Mid-Semester Examination', course: 'Database Systems', courseCode: 'CS302', academicYear: '2025/2026', semester: 2, paperType: 'midsem', status: 'reviewed', sections: 3, totalMarks: 40, duration: 90, author: 'Dr. Njoroge', reviewedBy: 'Prof. Wafula', classification: 'restricted', lastModified: '1 day ago' },
  { id: '4', title: 'End of Semester Examination', course: 'Computer Networks', courseCode: 'CS305', academicYear: '2025/2026', semester: 2, paperType: 'endsem', status: 'draft', sections: 4, totalMarks: 70, duration: 180, author: 'Dr. Ochieng', classification: 'restricted', lastModified: '3 hours ago' },
  { id: '5', title: 'Supplementary Examination', course: 'Discrete Mathematics', courseCode: 'MAT301', academicYear: '2024/2025', semester: 1, paperType: 'supplementary', status: 'approved', sections: 5, totalMarks: 70, duration: 180, author: 'Dr. Akinyi', reviewedBy: 'Prof. Muthoni', approvedBy: 'Dean - SoS', classification: 'confidential', lastModified: '3 months ago' },
  { id: '6', title: 'End of Semester Examination', course: 'Database Systems', courseCode: 'CS302', academicYear: '2024/2025', semester: 2, paperType: 'endsem', status: 'archived', sections: 5, totalMarks: 70, duration: 180, author: 'Dr. Njoroge', classification: 'public', lastModified: '6 months ago', practiceMode: true },
  { id: '7', title: 'CAT 1 — Network Layers', course: 'Computer Networks', courseCode: 'CS305', academicYear: '2025/2026', semester: 2, paperType: 'cat', status: 'draft', sections: 2, totalMarks: 30, duration: 45, author: 'Dr. Ochieng', classification: 'confidential', lastModified: '1 hour ago' },
  { id: '8', title: 'End of Semester Examination', course: 'Discrete Mathematics', courseCode: 'MAT301', academicYear: '2024/2025', semester: 1, paperType: 'endsem', status: 'archived', sections: 6, totalMarks: 70, duration: 180, author: 'Dr. Akinyi', classification: 'public', lastModified: '1 year ago', practiceMode: true },
];

const TYPE_META: Record<PaperType, { label: string; color: string }> = {
  cat: { label: 'CAT', color: 'bg-brand-500' },
  midsem: { label: 'Mid-Sem', color: 'bg-accent-400' },
  endsem: { label: 'End-Sem', color: 'bg-gold-500' },
  supplementary: { label: 'Suppl.', color: 'bg-info' },
};

const STATUS_META: Record<PaperStatus, { label: string; variant: 'default' | 'warning' | 'brand' | 'success' | 'info'; icon: React.ReactNode }> = {
  draft: { label: 'Draft', variant: 'warning', icon: <Clock size={12} /> },
  reviewed: { label: 'Reviewed', variant: 'brand', icon: <Eye size={12} /> },
  approved: { label: 'Approved', variant: 'success', icon: <CheckCircle2 size={12} /> },
  archived: { label: 'Archived', variant: 'default', icon: <Archive size={12} /> },
  published: { label: 'Published', variant: 'info', icon: <Send size={12} /> },
};

const CLASS_META: Record<string, { label: string; color: string; icon: React.ReactNode }> = {
  restricted: { label: 'Restricted', color: 'text-danger', icon: <Lock size={11} /> },
  confidential: { label: 'Confidential', color: 'text-warning', icon: <Shield size={11} /> },
  public: { label: 'Public', color: 'text-success', icon: <Eye size={11} /> },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ExamBankPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('all');
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';
  const isStudent = user?.role === 'learner';

  const visible = isStudent
    ? PAPERS.filter(p => p.practiceMode || p.status === 'published')
    : PAPERS;

  const filtered = visible.filter(p => {
    if (tab !== 'all' && p.paperType !== tab) return false;
    if (search && !p.title.toLowerCase().includes(search.toLowerCase()) && !p.course.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const stats = {
    total: visible.length,
    approved: visible.filter(p => p.status === 'approved').length,
    pendingReview: visible.filter(p => p.status === 'draft' || p.status === 'reviewed').length,
    archived: visible.filter(p => p.status === 'archived').length,
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Exam Bank</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isStudent ? 'Practice with past papers and view published exams' : 'Manage exam papers, approval workflows, and exam archives'}
            </p>
          </div>
          {isInstructor && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><Calendar size={15} /> Exam Timetable</Button>
              <Button size="sm"><Plus size={15} /> Create Paper</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
            <FileStack size={18} className="text-brand-500" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.total}</div>
            <div className="text-xs text-ink-tertiary">Total Papers</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center shrink-0">
            <CheckCircle2 size={18} className="text-success" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.approved}</div>
            <div className="text-xs text-ink-tertiary">Approved</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center shrink-0">
            <AlertTriangle size={18} className="text-warning" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.pendingReview}</div>
            <div className="text-xs text-ink-tertiary">{isStudent ? 'Upcoming' : 'Pending'}</div>
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-surface-sunken flex items-center justify-center shrink-0">
            <Archive size={18} className="text-ink-tertiary" />
          </div>
          <div>
            <div className="text-xl font-bold font-[family-name:var(--font-display)]">{stats.archived}</div>
            <div className="text-xs text-ink-tertiary">Archived</div>
          </div>
        </div>
      </motion.div>

      {/* Tabs + Search */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="flex items-center justify-between gap-4 flex-wrap">
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
          {(['all', 'cat', 'midsem', 'endsem', 'supplementary'] as Tab[]).map(t => (
            <button
              key={t}
              onClick={() => setTab(t)}
              className={`px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${
                tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}
            >
              {t === 'all' ? 'All' : TYPE_META[t as PaperType].label}
            </button>
          ))}
        </div>
        <div className="flex gap-2 flex-1 justify-end">
          <div className="relative max-w-xs flex-1">
            <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search papers..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400" />
          </div>
          <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
            <Filter size={16} />
          </button>
        </div>
      </motion.div>

      {/* Approval workflow banner (instructor) */}
      {isInstructor && (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.14 }}>
          <Card padding="sm" className="bg-brand-50/50 border-brand-100">
            <div className="flex items-center gap-6 text-xs">
              <span className="font-semibold text-brand-600">Approval Workflow:</span>
              <div className="flex items-center gap-2">
                <span className="px-2 py-0.5 rounded bg-warning-light text-warning font-medium">Draft</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-brand-50 text-brand-600 font-medium">Reviewed</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-success-light text-success font-medium">Approved</span>
                <ChevronRight size={12} className="text-ink-placeholder" />
                <span className="px-2 py-0.5 rounded bg-sand-200 text-ink-secondary font-medium">Locked</span>
              </div>
            </div>
          </Card>
        </motion.div>
      )}

      {/* Papers list */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.16 }} className="space-y-3">
        {filtered.map((paper, i) => {
          const typeMeta = TYPE_META[paper.paperType];
          const statusMeta = STATUS_META[paper.status];
          const classMeta = CLASS_META[paper.classification];

          return (
            <motion.div key={paper.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ duration: 0.25, delay: i * 0.04 }}>
              <Card hover padding="none">
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    <div className={`w-12 h-12 rounded-xl ${typeMeta.color} flex flex-col items-center justify-center shrink-0`}>
                      <span className="text-white text-[9px] font-bold font-[family-name:var(--font-display)] leading-none">{typeMeta.label.toUpperCase()}</span>
                    </div>

                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-3">
                        <div>
                          <div className="flex items-center gap-2 flex-wrap">
                            <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{paper.title}</h3>
                            {paper.practiceMode && (
                              <span className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-success-light text-success text-[10px] font-bold">
                                <BookOpen size={10} /> Practice
                              </span>
                            )}
                          </div>
                          <div className="flex items-center gap-2 mt-1">
                            <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{paper.courseCode}</span>
                            <span className="text-xs text-ink-tertiary">{paper.course}</span>
                          </div>
                        </div>
                        <div className="flex items-center gap-2 shrink-0">
                          <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                          {isInstructor && (
                            <span className={`flex items-center gap-0.5 text-[10px] font-medium ${classMeta.color}`}>
                              {classMeta.icon} {classMeta.label}
                            </span>
                          )}
                        </div>
                      </div>

                      <div className="flex items-center gap-5 mt-3 flex-wrap">
                        <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                          <Calendar size={13} />
                          <span>{paper.academicYear} · Sem {paper.semester}</span>
                        </div>
                        <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                          <FileStack size={13} />
                          <span>{paper.sections} sections · {paper.totalMarks} marks</span>
                        </div>
                        <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                          <Clock size={13} />
                          <span>{paper.duration} min</span>
                        </div>
                        {isInstructor && (
                          <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                            <Users size={13} />
                            <span>By {paper.author}</span>
                          </div>
                        )}
                        <span className="text-[11px] text-ink-placeholder">Modified {paper.lastModified}</span>
                      </div>

                      {isInstructor && (paper.reviewedBy || paper.approvedBy) && (
                        <div className="flex items-center gap-4 mt-2 text-[11px] text-ink-tertiary">
                          {paper.reviewedBy && <span>Reviewed by <span className="font-medium text-ink-secondary">{paper.reviewedBy}</span></span>}
                          {paper.approvedBy && <span>Approved by <span className="font-medium text-ink-secondary">{paper.approvedBy}</span></span>}
                        </div>
                      )}
                    </div>

                    <div className="flex items-center gap-2 shrink-0">
                      {isStudent && paper.practiceMode && <Button size="sm"><BookOpen size={14} /> Practice</Button>}
                      {isInstructor && (
                        <>
                          {paper.status === 'draft' && <Button variant="outline" size="sm"><Send size={14} /> Submit</Button>}
                          {paper.status === 'reviewed' && <Button size="sm"><CheckCircle2 size={14} /> Approve</Button>}
                          <Button variant="ghost" size="sm"><Eye size={14} /></Button>
                          <Button variant="ghost" size="sm"><Download size={14} /></Button>
                          <button className="p-1.5 rounded-md hover:bg-sand-100 text-ink-tertiary cursor-pointer">
                            <MoreHorizontal size={16} />
                          </button>
                        </>
                      )}
                      {!isStudent && !isInstructor && <ChevronRight size={16} className="text-ink-tertiary" />}
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          );
        })}

        {filtered.length === 0 && (
          <Card className="text-center py-16">
            <FileStack size={36} className="mx-auto text-ink-placeholder mb-3" />
            <p className="text-ink-tertiary">No exam papers found</p>
          </Card>
        )}
      </motion.div>
    </div>
  );
}
