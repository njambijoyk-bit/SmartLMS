import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  TrendingUp, TrendingDown, Users, BookOpen,
  Award, AlertCircle, Download, RefreshCw, ChevronDown,
} from 'lucide-react';
import {
  AreaChart, Area, BarChart, Bar, LineChart, Line,
  XAxis, YAxis, Tooltip, ResponsiveContainer, PieChart, Pie, Cell,
} from 'recharts';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

const enrollmentTrend = [
  { month: 'Sep', active: 1240, new: 320 },
  { month: 'Oct', active: 1380, new: 180 },
  { month: 'Nov', active: 1420, new: 95 },
  { month: 'Dec', active: 1100, new: 30 },
  { month: 'Jan', active: 1510, new: 410 },
  { month: 'Feb', active: 1680, new: 220 },
  { month: 'Mar', active: 1740, new: 140 },
  { month: 'Apr', active: 1820, new: 180 },
];

const gradeDistribution = [
  { grade: 'A (70–100)', count: 412, color: '#1B8F5A' },
  { grade: 'B (60–69)', count: 534, color: '#0D5E6D' },
  { grade: 'C (50–59)', count: 380, color: '#D4A84B' },
  { grade: 'D (40–49)', count: 198, color: '#C75C2B' },
  { grade: 'F (<40)', count: 94, color: '#C43D3D' },
];

const courseEngagement = [
  { course: 'CS301', completion: 72, avgScore: 74, dropout: 8 },
  { course: 'CS302', completion: 58, avgScore: 68, dropout: 14 },
  { course: 'MAT301', completion: 85, avgScore: 81, dropout: 5 },
  { course: 'CS305', completion: 34, avgScore: 61, dropout: 22 },
  { course: 'CS401', completion: 42, avgScore: 73, dropout: 18 },
  { course: 'BUS201', completion: 91, avgScore: 77, dropout: 3 },
];

const weeklyActivity = [
  { day: 'Mon', logins: 820, submissions: 145 },
  { day: 'Tue', logins: 940, submissions: 210 },
  { day: 'Wed', logins: 1120, submissions: 380 },
  { day: 'Thu', logins: 980, submissions: 290 },
  { day: 'Fri', logins: 760, submissions: 180 },
  { day: 'Sat', logins: 340, submissions: 95 },
  { day: 'Sun', logins: 280, submissions: 55 },
];

const atRiskStudents = [
  { name: 'Kevin Otieno', course: 'CS305', score: 31, attendance: 42, risk: 'high' },
  { name: 'Mary Wanjiku', course: 'CS302', score: 38, attendance: 55, risk: 'high' },
  { name: 'Brian Kamau', course: 'CS401', score: 44, attendance: 60, risk: 'medium' },
  { name: 'Rose Adhiambo', course: 'MAT301', score: 46, attendance: 68, risk: 'medium' },
  { name: 'Daniel Mutua', course: 'CS301', score: 49, attendance: 72, risk: 'medium' },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

const CUSTOM_TOOLTIP_STYLE = { borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 12, boxShadow: '0 4px 12px rgba(0,0,0,0.06)' };

export function AnalyticsPage() {
  const [period, setPeriod] = useState('semester');

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between flex-wrap gap-3">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Analytics</h1>
            <p className="text-sm text-ink-tertiary mt-1">Institution-wide learning insights · Updated 5 min ago</p>
          </div>
          <div className="flex gap-2">
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {['week', 'month', 'semester'].map(p => (
                <button
                  key={p}
                  onClick={() => setPeriod(p)}
                  className={`px-3 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${period === p ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}
                >
                  {p}
                </button>
              ))}
            </div>
            <Button variant="outline" size="sm"><Download size={14} /> Export</Button>
            <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
              <RefreshCw size={15} />
            </button>
          </div>
        </div>
      </motion.div>

      {/* KPI cards */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        {[
          { label: 'Active Learners', value: '1,820', change: +4.8, icon: <Users size={18} />, color: 'text-brand-500', bg: 'bg-brand-50' },
          { label: 'Avg. Completion', value: '63.7%', change: +2.1, icon: <BookOpen size={18} />, color: 'text-success', bg: 'bg-success-light' },
          { label: 'Avg. Assessment Score', value: '72.4', change: -1.3, icon: <Award size={18} />, color: 'text-gold-500', bg: 'bg-gold-50' },
          { label: 'At-Risk Students', value: '34', change: -5, icon: <AlertCircle size={18} />, color: 'text-danger', bg: 'bg-danger-light' },
        ].map((kpi, i) => (
          <div key={i} className="bg-surface-raised rounded-xl border border-sand-200 p-4">
            <div className={`w-9 h-9 rounded-lg ${kpi.bg} flex items-center justify-center mb-3`}>
              <span className={kpi.color}>{kpi.icon}</span>
            </div>
            <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{kpi.value}</div>
            <div className="flex items-center justify-between mt-1">
              <span className="text-xs text-ink-tertiary">{kpi.label}</span>
              <span className={`text-xs font-semibold flex items-center gap-0.5 ${kpi.change > 0 ? 'text-success' : 'text-danger'}`}>
                {kpi.change > 0 ? <TrendingUp size={11} /> : <TrendingDown size={11} />}
                {Math.abs(kpi.change)}%
              </span>
            </div>
          </div>
        ))}
      </motion.div>

      {/* Charts row 1 */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Enrollment trend */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.12 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-5">
              <div>
                <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Learner Activity Trend</h3>
                <p className="text-xs text-ink-tertiary mt-0.5">Active and new enrolments by month</p>
              </div>
              <button className="flex items-center gap-1 text-xs text-ink-tertiary border border-sand-300 rounded-lg px-2.5 py-1.5 hover:text-ink cursor-pointer">
                Monthly <ChevronDown size={13} />
              </button>
            </div>
            <ResponsiveContainer width="100%" height={220}>
              <AreaChart data={enrollmentTrend}>
                <defs>
                  <linearGradient id="activeGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#0D5E6D" stopOpacity={0.15} />
                    <stop offset="95%" stopColor="#0D5E6D" stopOpacity={0} />
                  </linearGradient>
                  <linearGradient id="newGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#C75C2B" stopOpacity={0.12} />
                    <stop offset="95%" stopColor="#C75C2B" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <Tooltip contentStyle={CUSTOM_TOOLTIP_STYLE} />
                <Area type="monotone" dataKey="active" stroke="#0D5E6D" strokeWidth={2} fill="url(#activeGrad)" name="Active" />
                <Area type="monotone" dataKey="new" stroke="#C75C2B" strokeWidth={2} fill="url(#newGrad)" name="New" />
              </AreaChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Grade distribution */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.16 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-1">Grade Distribution</h3>
            <p className="text-xs text-ink-tertiary mb-4">All students, current semester</p>
            <ResponsiveContainer width="100%" height={160}>
              <PieChart>
                <Pie data={gradeDistribution} dataKey="count" nameKey="grade" cx="50%" cy="50%" outerRadius={65} innerRadius={35}>
                  {gradeDistribution.map((entry, i) => (
                    <Cell key={i} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip contentStyle={CUSTOM_TOOLTIP_STYLE} />
              </PieChart>
            </ResponsiveContainer>
            <div className="space-y-2 mt-2">
              {gradeDistribution.map(g => (
                <div key={g.grade} className="flex items-center justify-between text-xs">
                  <div className="flex items-center gap-2">
                    <div className="w-2.5 h-2.5 rounded-full" style={{ background: g.color }} />
                    <span className="text-ink-secondary">{g.grade}</span>
                  </div>
                  <span className="font-semibold text-ink tabular-nums">{g.count}</span>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>

      {/* Charts row 2 */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Course engagement */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-1">Course Completion Rates</h3>
            <p className="text-xs text-ink-tertiary mb-5">Completion % vs avg score per course</p>
            <ResponsiveContainer width="100%" height={200}>
              <BarChart data={courseEngagement} layout="vertical" barSize={10}>
                <XAxis type="number" domain={[0, 100]} axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} unit="%" />
                <YAxis type="category" dataKey="course" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} width={45} />
                <Tooltip contentStyle={CUSTOM_TOOLTIP_STYLE} />
                <Bar dataKey="completion" fill="#0D5E6D" radius={[0, 4, 4, 0]} name="Completion %" />
                <Bar dataKey="avgScore" fill="#D4A84B" radius={[0, 4, 4, 0]} name="Avg Score" />
              </BarChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Weekly activity */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.24 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-1">Weekly Activity Pattern</h3>
            <p className="text-xs text-ink-tertiary mb-5">Logins and submissions by day</p>
            <ResponsiveContainer width="100%" height={200}>
              <LineChart data={weeklyActivity}>
                <XAxis dataKey="day" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <Tooltip contentStyle={CUSTOM_TOOLTIP_STYLE} />
                <Line type="monotone" dataKey="logins" stroke="#0D5E6D" strokeWidth={2.5} dot={false} name="Logins" />
                <Line type="monotone" dataKey="submissions" stroke="#C75C2B" strokeWidth={2.5} dot={false} name="Submissions" />
              </LineChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>
      </div>

      {/* At-risk students */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.28 }}>
        <Card padding="none">
          <div className="p-5 border-b border-sand-200 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <AlertCircle size={18} className="text-danger" />
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">At-Risk Students</h3>
              <Badge variant="danger">Needs attention</Badge>
            </div>
            <Button variant="ghost" size="sm">View all</Button>
          </div>
          <div className="divide-y divide-sand-100">
            {atRiskStudents.map((s, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.3 + i * 0.06 }}
                className="flex items-center gap-4 px-5 py-3.5 hover:bg-sand-50 transition-colors"
              >
                <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold text-white shrink-0 ${s.risk === 'high' ? 'bg-danger' : 'bg-warning'}`}>
                  {s.name.charAt(0)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-medium text-ink">{s.name}</span>
                    <span className="text-xs text-ink-tertiary">{s.course}</span>
                  </div>
                  <div className="flex items-center gap-4 mt-1.5">
                    <div className="flex items-center gap-1.5 flex-1">
                      <span className="text-[10px] text-ink-tertiary w-10">Score</span>
                      <div className="flex-1 h-1.5 bg-sand-200 rounded-full overflow-hidden max-w-24">
                        <div className={`h-full rounded-full ${s.score < 40 ? 'bg-danger' : 'bg-warning'}`} style={{ width: `${s.score}%` }} />
                      </div>
                      <span className={`text-xs font-semibold ${s.score < 40 ? 'text-danger' : 'text-warning'}`}>{s.score}%</span>
                    </div>
                    <div className="flex items-center gap-1.5 flex-1">
                      <span className="text-[10px] text-ink-tertiary w-16">Attendance</span>
                      <div className="flex-1 h-1.5 bg-sand-200 rounded-full overflow-hidden max-w-24">
                        <div className={`h-full rounded-full ${s.attendance < 50 ? 'bg-danger' : 'bg-warning'}`} style={{ width: `${s.attendance}%` }} />
                      </div>
                      <span className={`text-xs font-semibold ${s.attendance < 50 ? 'text-danger' : 'text-warning'}`}>{s.attendance}%</span>
                    </div>
                  </div>
                </div>
                <Badge variant={s.risk === 'high' ? 'danger' : 'warning'} className="shrink-0">
                  {s.risk === 'high' ? 'High risk' : 'Watch'}
                </Badge>
              </motion.div>
            ))}
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
