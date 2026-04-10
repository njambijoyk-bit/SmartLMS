import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  CheckCircle2, XCircle, Clock, Download,
  Search, ChevronRight, AlertTriangle,
  Building2, BookOpen, CreditCard, Home,
  FileText, Shield, Users,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type ClearanceStatus = 'cleared' | 'pending' | 'blocked';

interface Department {
  id: string;
  name: string;
  icon: React.ReactNode;
  officer: string;
  status: ClearanceStatus;
  note?: string;
  clearedDate?: string;
}

const STUDENT_DEPARTMENTS: Department[] = [
  { id: '1', name: 'Finance Office', icon: <CreditCard size={18} />, officer: 'Ms. Wanjiru', status: 'cleared', clearedDate: 'Mar 28, 2026' },
  { id: '2', name: 'Library', icon: <BookOpen size={18} />, officer: 'Mr. Odhiambo', status: 'cleared', clearedDate: 'Mar 25, 2026' },
  { id: '3', name: 'Examination Office', icon: <FileText size={18} />, officer: 'Dr. Kamau', status: 'pending' },
  { id: '4', name: 'Department', icon: <Building2 size={18} />, officer: 'Prof. Mwangi', status: 'blocked', note: 'Project report not submitted' },
  { id: '5', name: 'Hostel', icon: <Home size={18} />, officer: 'Ms. Adhiambo', status: 'cleared', clearedDate: 'Mar 20, 2026' },
  { id: '6', name: 'Student Welfare', icon: <Shield size={18} />, officer: 'Mr. Gitau', status: 'pending' },
];

const ADMIN_STUDENTS = [
  { id: '1', name: 'Faith Kamau', regNo: 'CS/2022/001', programme: 'BSc Computer Science', cleared: 5, total: 6, status: 'in_progress' as const },
  { id: '2', name: 'Daniel Mutua', regNo: 'CS/2022/004', programme: 'BSc Computer Science', cleared: 6, total: 6, status: 'complete' as const },
  { id: '3', name: 'Grace Njeri', regNo: 'CS/2022/007', programme: 'BSc Computer Science', cleared: 6, total: 6, status: 'complete' as const },
  { id: '4', name: 'Brian Otieno', regNo: 'CS/2022/002', programme: 'BSc Computer Science', cleared: 2, total: 6, status: 'in_progress' as const },
  { id: '5', name: 'Rose Adhiambo', regNo: 'CS/2022/005', programme: 'BSc Computer Science', cleared: 4, total: 6, status: 'in_progress' as const },
  { id: '6', name: 'Kevin Kamau', regNo: 'CS/2022/006', programme: 'BSc Computer Science', cleared: 0, total: 6, status: 'blocked' as const },
];

const STATUS_META: Record<ClearanceStatus, { bg: string; icon: React.ReactNode; label: string; variant: 'success' | 'warning' | 'danger' }> = {
  cleared: { bg: 'bg-success-light border-success/20', icon: <CheckCircle2 size={20} className="text-success" />, label: 'Cleared', variant: 'success' },
  pending: { bg: 'bg-warning-light border-warning/20', icon: <Clock size={20} className="text-warning" />, label: 'Pending', variant: 'warning' },
  blocked: { bg: 'bg-danger-light border-danger/20', icon: <XCircle size={20} className="text-danger" />, label: 'Blocked', variant: 'danger' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ClearancePage() {
  const { user } = useAuth();
  const [search, setSearch] = useState('');
  const [filter, setFilter] = useState<'all' | 'complete' | 'in_progress' | 'blocked'>('all');

  const isAdmin = user?.role === 'admin';

  const clearedCount = STUDENT_DEPARTMENTS.filter(d => d.status === 'cleared').length;
  const allCleared = clearedCount === STUDENT_DEPARTMENTS.length;

  const filteredStudents = ADMIN_STUDENTS.filter(s => {
    if (filter !== 'all' && s.status !== filter) return false;
    if (search && !s.name.toLowerCase().includes(search.toLowerCase()) && !s.regNo.includes(search)) return false;
    return true;
  });

  // Student view
  if (!isAdmin) {
    return (
      <div className="space-y-5 max-w-2xl">
        <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
          <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Clearance</h1>
          <p className="text-sm text-ink-tertiary mt-1">Complete all department clearances to receive your certificate</p>
        </motion.div>

        {/* Progress summary */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
          <div className={`rounded-xl border-2 p-5 ${allCleared ? 'border-success/30 bg-success-light' : 'border-brand-200 bg-brand-50'}`}>
            <div className="flex items-center justify-between">
              <div>
                <div className={`text-sm font-medium ${allCleared ? 'text-success' : 'text-brand-600'}`}>Clearance Progress</div>
                <div className={`text-4xl font-bold font-[family-name:var(--font-display)] mt-1 ${allCleared ? 'text-success' : 'text-brand-700'}`}>
                  {clearedCount}/{STUDENT_DEPARTMENTS.length}
                </div>
                <div className={`text-xs mt-1 ${allCleared ? 'text-success/70' : 'text-brand-500'}`}>
                  {allCleared ? 'All departments cleared! Download your certificate below.' : 'departments cleared'}
                </div>
              </div>
              <div className={`w-20 h-20 rounded-full border-4 flex items-center justify-center ${
                allCleared ? 'border-success bg-success-light' : 'border-brand-300 bg-brand-50'
              }`}>
                <span className={`text-2xl font-bold font-[family-name:var(--font-display)] ${allCleared ? 'text-success' : 'text-brand-600'}`}>
                  {Math.round((clearedCount / STUDENT_DEPARTMENTS.length) * 100)}%
                </span>
              </div>
            </div>
            {allCleared && (
              <Button className="mt-4 w-full" size="sm"><Download size={14} /> Download Clearance Certificate</Button>
            )}
          </div>
        </motion.div>

        {/* Department checklist */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <div className="space-y-3">
            {STUDENT_DEPARTMENTS.map((dept, i) => {
              const meta = STATUS_META[dept.status];
              return (
                <motion.div
                  key={dept.id}
                  initial={{ opacity: 0, x: -8 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.25 + i * 0.06 }}
                >
                  <Card padding="none">
                    <div className={`flex items-center gap-4 p-4 border-l-4 ${
                      dept.status === 'cleared' ? 'border-l-success' : dept.status === 'blocked' ? 'border-l-danger' : 'border-l-warning'
                    }`}>
                      <div className={`w-10 h-10 rounded-xl ${meta.bg} border flex items-center justify-center shrink-0`}>
                        {meta.icon}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <span className="text-sm font-semibold text-ink">{dept.name}</span>
                          <Badge variant={meta.variant} size="sm">{meta.label}</Badge>
                        </div>
                        <div className="text-xs text-ink-tertiary mt-0.5">Officer: {dept.officer}</div>
                        {dept.note && (
                          <div className="flex items-center gap-1.5 mt-1.5 text-xs text-danger">
                            <AlertTriangle size={11} /> {dept.note}
                          </div>
                        )}
                        {dept.clearedDate && (
                          <div className="text-[10px] text-success mt-1">Cleared on {dept.clearedDate}</div>
                        )}
                      </div>
                      <span className="text-ink-tertiary shrink-0">{dept.icon}</span>
                    </div>
                  </Card>
                </motion.div>
              );
            })}
          </div>
        </motion.div>
      </div>
    );
  }

  // Admin view
  return (
    <div className="space-y-5">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Clearance Management</h1>
            <p className="text-sm text-ink-tertiary mt-1">Review and manage student clearance across all departments</p>
          </div>
          <Button variant="outline" size="sm"><Download size={14} /> Export Report</Button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Users size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Total Students</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ADMIN_STUDENTS.length}</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Fully Cleared</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{ADMIN_STUDENTS.filter(s => s.status === 'complete').length}</div>
        </div>
        <div className="bg-warning-light rounded-xl border border-warning/20 p-4">
          <div className="text-xs text-warning/70">In Progress</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-warning">{ADMIN_STUDENTS.filter(s => s.status === 'in_progress').length}</div>
        </div>
        <div className="bg-danger-light rounded-xl border border-danger/20 p-4">
          <div className="text-xs text-danger/70">Blocked</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-danger">{ADMIN_STUDENTS.filter(s => s.status === 'blocked').length}</div>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
        <div className="flex items-center gap-2">
          <div className="relative flex-1 max-w-sm">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search student..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
          </div>
          <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
            {(['all', 'complete', 'in_progress', 'blocked'] as const).map(f => (
              <button key={f} onClick={() => setFilter(f)}
                className={`px-2.5 py-1.5 text-[11px] font-medium rounded-lg transition-all capitalize cursor-pointer ${filter === f ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                {f.replace('_', ' ')}
              </button>
            ))}
          </div>
        </div>
      </motion.div>

      {/* Student list */}
      <Card padding="none">
        <div className="divide-y divide-sand-100">
          {filteredStudents.map((student, i) => (
            <motion.div key={student.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
              className="flex items-center gap-4 px-5 py-4 hover:bg-sand-50 transition-colors cursor-pointer">
              <div className="w-9 h-9 rounded-full bg-brand-500 flex items-center justify-center text-white text-sm font-bold font-[family-name:var(--font-display)] shrink-0">
                {student.name.charAt(0)}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span className="text-sm font-semibold text-ink">{student.name}</span>
                  <span className="text-xs text-ink-tertiary">{student.regNo}</span>
                </div>
                <div className="text-xs text-ink-tertiary">{student.programme}</div>
              </div>
              <div className="flex items-center gap-3 shrink-0">
                <div className="text-right">
                  <div className="text-sm font-bold font-[family-name:var(--font-display)] text-ink">{student.cleared}/{student.total}</div>
                  <div className="text-[10px] text-ink-tertiary">departments</div>
                </div>
                <div className="w-16 h-2 bg-sand-200 rounded-full overflow-hidden">
                  <div className={`h-full rounded-full ${student.status === 'complete' ? 'bg-success' : student.status === 'blocked' ? 'bg-danger' : 'bg-warning'}`}
                    style={{ width: `${(student.cleared / student.total) * 100}%` }} />
                </div>
                <Badge variant={student.status === 'complete' ? 'success' : student.status === 'blocked' ? 'danger' : 'warning'} size="sm">
                  {student.status === 'complete' ? 'Cleared' : student.status === 'blocked' ? 'Blocked' : 'In Progress'}
                </Badge>
              </div>
              <ChevronRight size={14} className="text-ink-placeholder shrink-0" />
            </motion.div>
          ))}
        </div>
      </Card>
    </div>
  );
}
