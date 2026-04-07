import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  UserCheck, Calendar, Clock, Search,
  ChevronRight, TrendingUp,
  TrendingDown, AlertTriangle, FileText,
  MessageSquare, Target, CheckCircle2,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

interface Advisee {
  id: string;
  name: string;
  regNo: string;
  programme: string;
  year: string;
  gpa: number;
  gpaTrend: 'up' | 'down' | 'flat';
  attendance: number;
  creditsCompleted: number;
  creditsRequired: number;
  riskLevel: 'low' | 'medium' | 'high';
  nextAppointment?: string;
  notes: number;
}

const ADVISEES: Advisee[] = [
  { id: '1', name: 'Faith Kamau', regNo: 'CS/2022/001', programme: 'BSc Computer Science', year: 'Year 3', gpa: 3.4, gpaTrend: 'up', attendance: 87, creditsCompleted: 96, creditsRequired: 144, riskLevel: 'low', nextAppointment: 'Apr 8, 2:00 PM', notes: 5 },
  { id: '2', name: 'Brian Otieno', regNo: 'CS/2022/002', programme: 'BSc Computer Science', year: 'Year 3', gpa: 2.8, gpaTrend: 'down', attendance: 68, creditsCompleted: 84, creditsRequired: 144, riskLevel: 'high', notes: 8 },
  { id: '3', name: 'Mary Wanjiku', regNo: 'CS/2022/003', programme: 'BSc Computer Science', year: 'Year 3', gpa: 2.1, gpaTrend: 'down', attendance: 50, creditsCompleted: 72, creditsRequired: 144, riskLevel: 'high', notes: 12 },
  { id: '4', name: 'Daniel Mutua', regNo: 'CS/2022/004', programme: 'BSc Computer Science', year: 'Year 3', gpa: 3.7, gpaTrend: 'up', attendance: 94, creditsCompleted: 108, creditsRequired: 144, riskLevel: 'low', nextAppointment: 'Apr 10, 10:00 AM', notes: 3 },
  { id: '5', name: 'Rose Adhiambo', regNo: 'CS/2022/005', programme: 'BSc Computer Science', year: 'Year 3', gpa: 3.0, gpaTrend: 'flat', attendance: 75, creditsCompleted: 90, creditsRequired: 144, riskLevel: 'medium', notes: 6 },
  { id: '6', name: 'Kevin Kamau', regNo: 'CS/2022/006', programme: 'BSc Computer Science', year: 'Year 3', gpa: 1.9, gpaTrend: 'down', attendance: 38, creditsCompleted: 60, creditsRequired: 144, riskLevel: 'high', notes: 15 },
];

const UPCOMING = [
  { student: 'Faith Kamau', time: 'Apr 8, 2:00 PM', topic: 'Semester course selection' },
  { student: 'Daniel Mutua', time: 'Apr 10, 10:00 AM', topic: 'Graduation audit review' },
  { student: 'Rose Adhiambo', time: 'Apr 12, 3:00 PM', topic: 'Elective recommendations' },
];

const RISK_META = {
  low: { variant: 'success' as const, label: 'Low Risk' },
  medium: { variant: 'warning' as const, label: 'Medium' },
  high: { variant: 'danger' as const, label: 'High Risk' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function AdvisingPage() {
  const { user } = useAuth();
  const [search, setSearch] = useState('');
  const [riskFilter, setRiskFilter] = useState<'all' | 'low' | 'medium' | 'high'>('all');

  const isAdvisor = user?.role === 'admin' || user?.role === 'advisor' || user?.role === 'instructor';

  const filtered = ADVISEES.filter(a => {
    if (riskFilter !== 'all' && a.riskLevel !== riskFilter) return false;
    if (search && !a.name.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const highRiskCount = ADVISEES.filter(a => a.riskLevel === 'high').length;

  // Student view
  if (!isAdvisor) {
    return (
      <div className="space-y-5 max-w-2xl">
        <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
          <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Academic Advising</h1>
          <p className="text-sm text-ink-tertiary mt-1">Your advisor and academic plan</p>
        </motion.div>

        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
          <Card>
            <div className="flex items-center gap-4">
              <Avatar name="Dr. Sarah Otieno" size="lg" />
              <div>
                <div className="text-sm font-semibold text-ink">Dr. Sarah Otieno</div>
                <div className="text-xs text-ink-tertiary">Your Academic Advisor · School of Computing</div>
                <div className="flex gap-2 mt-2">
                  <Button size="sm"><Calendar size={14} /> Book Appointment</Button>
                  <Button variant="outline" size="sm"><MessageSquare size={14} /> Message</Button>
                </div>
              </div>
            </div>
          </Card>
        </motion.div>

        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Graduation Progress</h3>
            <div className="space-y-4">
              <div>
                <div className="flex justify-between text-sm mb-1.5">
                  <span className="text-ink-secondary font-medium">Credits Completed</span>
                  <span className="font-bold font-[family-name:var(--font-display)] text-ink">96 / 144</span>
                </div>
                <ProgressBar value={67} />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="p-3 rounded-lg bg-sand-50 border border-sand-200">
                  <div className="text-xs text-ink-tertiary">Core Units</div>
                  <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">72/96</div>
                </div>
                <div className="p-3 rounded-lg bg-sand-50 border border-sand-200">
                  <div className="text-xs text-ink-tertiary">Electives</div>
                  <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">24/48</div>
                </div>
              </div>
              <div className="p-3 rounded-lg bg-brand-50 border border-brand-200">
                <div className="text-xs font-medium text-brand-700">Graduation Audit</div>
                <p className="text-xs text-brand-600 mt-1">You need 12 more elective credits and CS401 (Capstone Project) to graduate. Consider taking CS404 and CS406 next semester.</p>
              </div>
            </div>
          </Card>
        </motion.div>
      </div>
    );
  }

  // Advisor view
  return (
    <div className="space-y-5">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Academic Advising</h1>
            <p className="text-sm text-ink-tertiary mt-1">Manage your advisee caseload and track student progress</p>
          </div>
          <Button size="sm"><Calendar size={14} /> Schedule Session</Button>
        </div>
      </motion.div>

      {/* KPIs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><UserCheck size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Advisees</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ADVISEES.length}</div>
        </div>
        <div className="bg-danger-light rounded-xl border border-danger/20 p-4">
          <div className="flex items-center gap-1.5 mb-1"><AlertTriangle size={14} className="text-danger" /><span className="text-xs text-danger/70">High Risk</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-danger">{highRiskCount}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Avg GPA</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">
            {(ADVISEES.reduce((s, a) => s + a.gpa, 0) / ADVISEES.length).toFixed(2)}
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Upcoming Sessions</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{UPCOMING.length}</div>
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Advisee list */}
        <div className="lg:col-span-2 space-y-3">
          <div className="flex items-center gap-2">
            <div className="relative flex-1 max-w-sm">
              <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
              <input type="text" placeholder="Search advisees..." value={search} onChange={e => setSearch(e.target.value)}
                className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
            </div>
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {(['all', 'high', 'medium', 'low'] as const).map(r => (
                <button key={r} onClick={() => setRiskFilter(r)}
                  className={`px-2.5 py-1.5 text-[11px] font-medium rounded-lg transition-all capitalize cursor-pointer ${riskFilter === r ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                  {r}
                </button>
              ))}
            </div>
          </div>

          {filtered.map((advisee, i) => (
            <motion.div key={advisee.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 + i * 0.05 }}>
              <Card padding="none">
                <div className="flex items-center gap-4 p-4 hover:bg-sand-50/50 transition-colors cursor-pointer">
                  <Avatar name={advisee.name} size="md" />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-semibold text-ink">{advisee.name}</span>
                      <span className="text-xs text-ink-tertiary">{advisee.regNo}</span>
                      <Badge variant={RISK_META[advisee.riskLevel].variant} size="sm">{RISK_META[advisee.riskLevel].label}</Badge>
                    </div>
                    <div className="text-xs text-ink-tertiary mt-0.5">{advisee.programme} · {advisee.year}</div>
                    <div className="flex items-center gap-4 mt-2">
                      <span className="text-xs text-ink-secondary">
                        GPA <span className="font-bold font-[family-name:var(--font-display)]">{advisee.gpa}</span>
                        {advisee.gpaTrend === 'up' && <TrendingUp size={11} className="inline ml-1 text-success" />}
                        {advisee.gpaTrend === 'down' && <TrendingDown size={11} className="inline ml-1 text-danger" />}
                      </span>
                      <span className={`text-xs ${advisee.attendance >= 75 ? 'text-success' : 'text-danger'}`}>
                        {advisee.attendance}% attendance
                      </span>
                      <span className="text-xs text-ink-tertiary">{advisee.creditsCompleted}/{advisee.creditsRequired} credits</span>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <div className="flex items-center gap-1 text-xs text-ink-tertiary"><FileText size={10} /> {advisee.notes} notes</div>
                    {advisee.nextAppointment && (
                      <div className="text-[10px] text-brand-500 mt-1">Next: {advisee.nextAppointment}</div>
                    )}
                  </div>
                  <ChevronRight size={14} className="text-ink-placeholder shrink-0" />
                </div>
              </Card>
            </motion.div>
          ))}
        </div>

        {/* Sidebar */}
        <div className="space-y-4">
          <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.2 }}>
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Upcoming Sessions</h3>
              <div className="space-y-3">
                {UPCOMING.map((apt, i) => (
                  <div key={i} className="flex items-start gap-2.5 p-3 rounded-lg bg-sand-50 border border-sand-200">
                    <Calendar size={14} className="text-brand-500 mt-0.5 shrink-0" />
                    <div>
                      <div className="text-xs font-semibold text-ink">{apt.student}</div>
                      <div className="flex items-center gap-1 text-[10px] text-ink-tertiary mt-0.5"><Clock size={9} /> {apt.time}</div>
                      <div className="text-[10px] text-ink-secondary mt-0.5">{apt.topic}</div>
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          </motion.div>

          <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.25 }}>
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Quick Actions</h3>
              <div className="space-y-2">
                {[
                  { icon: <Target size={14} />, label: 'Create Academic Plan', color: 'text-brand-500' },
                  { icon: <CheckCircle2 size={14} />, label: 'Run Graduation Audit', color: 'text-success' },
                  { icon: <MessageSquare size={14} />, label: 'Refer to Counselling', color: 'text-accent-400' },
                  { icon: <FileText size={14} />, label: 'Add Advising Note', color: 'text-info' },
                ].map(action => (
                  <button key={action.label} className="w-full flex items-center gap-2.5 p-3 rounded-lg border border-sand-200 bg-surface-raised hover:border-brand-200 transition-colors cursor-pointer text-left">
                    <span className={action.color}>{action.icon}</span>
                    <span className="text-xs font-medium text-ink">{action.label}</span>
                  </button>
                ))}
              </div>
            </Card>
          </motion.div>
        </div>
      </div>
    </div>
  );
}
