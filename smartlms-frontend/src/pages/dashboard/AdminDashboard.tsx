import { motion } from 'framer-motion';
import {
  Users, BookOpen, GraduationCap, CreditCard, TrendingUp,
  AlertTriangle, CheckCircle2, Clock, ArrowUpRight, Activity,
} from 'lucide-react';
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';
import { StatCard } from '../../components/ui/StatCard';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Avatar } from '../../components/ui/Avatar';
import { ProgressBar } from '../../components/ui/ProgressBar';

const enrollmentData = [
  { month: 'Sep', students: 3200 }, { month: 'Oct', students: 3450 },
  { month: 'Nov', students: 3600 }, { month: 'Dec', students: 3550 },
  { month: 'Jan', students: 3800 }, { month: 'Feb', students: 4100 },
  { month: 'Mar', students: 4350 }, { month: 'Apr', students: 4847 },
];

const courseActivityData = [
  { day: 'Mon', logins: 2100, submissions: 340 },
  { day: 'Tue', logins: 2400, submissions: 420 },
  { day: 'Wed', logins: 2200, submissions: 380 },
  { day: 'Thu', logins: 2600, submissions: 510 },
  { day: 'Fri', logins: 1800, submissions: 290 },
  { day: 'Sat', logins: 800, submissions: 120 },
  { day: 'Sun', logins: 600, submissions: 80 },
];

const recentRegistrations = [
  { name: 'Amina Hassan', programme: 'BSc Computer Science', status: 'pending' as const },
  { name: 'Brian Odhiambo', programme: 'BEd Arts', status: 'approved' as const },
  { name: 'Catherine Njeri', programme: 'BCom Finance', status: 'pending' as const },
  { name: 'David Kimani', programme: 'BSc Civil Engineering', status: 'approved' as const },
  { name: 'Eunice Wambui', programme: 'BA Communication', status: 'pending' as const },
];

const systemAlerts = [
  { type: 'warning' as const, message: '23 exam cards pending finance clearance', time: '2h ago' },
  { type: 'info' as const, message: 'Semester 2 registration closes in 5 days', time: '4h ago' },
  { type: 'success' as const, message: 'M-Pesa reconciliation completed — KES 2.4M matched', time: '6h ago' },
  { type: 'danger' as const, message: '3 failed backup attempts on MongoDB', time: '1d ago' },
];

const fadeIn = {
  initial: { opacity: 0, y: 12 },
  animate: { opacity: 1, y: 0 },
};

export function AdminDashboard() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">
          Institution Dashboard
        </h1>
        <p className="text-sm text-ink-tertiary mt-1">
          University of Nairobi — Academic Year 2025/2026, Semester 2
        </p>
      </motion.div>

      {/* Top-level pattern accent */}
      <div className="border-pattern w-full rounded-sm" />

      {/* Stats row */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="Active Learners" value="4,847" change={12} trend="up" icon={<Users size={18} className="text-brand-500" />} />
        <StatCard label="Active Courses" value="312" change={5} trend="up" icon={<BookOpen size={18} className="text-accent-400" />} accentColor="bg-accent-400" />
        <StatCard label="Graduation Rate" value="87%" change={3} trend="up" icon={<GraduationCap size={18} className="text-gold-500" />} accentColor="bg-gold-400" />
        <StatCard label="Revenue (Sem)" value="KES 24.3M" change={-2} trend="down" icon={<CreditCard size={18} className="text-success" />} accentColor="bg-success" />
      </motion.div>

      {/* Charts row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.15 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Enrollment Growth</h3>
                <p className="text-xs text-ink-tertiary mt-0.5">Active learners over time</p>
              </div>
              <div className="flex items-center gap-1 text-sm font-medium text-success">
                <TrendingUp size={14} />
                +12.4%
              </div>
            </div>
            <ResponsiveContainer width="100%" height={220}>
              <AreaChart data={enrollmentData}>
                <defs>
                  <linearGradient id="enrollGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#0D5E6D" stopOpacity={0.2} />
                    <stop offset="100%" stopColor="#0D5E6D" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fontSize: 12, fill: '#7A7E87' }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fontSize: 12, fill: '#7A7E87' }} />
                <Tooltip
                  contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 13 }}
                  labelStyle={{ fontWeight: 600, fontFamily: 'Outfit' }}
                />
                <Area type="monotone" dataKey="students" stroke="#0D5E6D" strokeWidth={2.5} fill="url(#enrollGrad)" />
              </AreaChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <div className="mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Weekly Activity</h3>
              <p className="text-xs text-ink-tertiary mt-0.5">Logins & submissions</p>
            </div>
            <ResponsiveContainer width="100%" height={220}>
              <BarChart data={courseActivityData}>
                <XAxis dataKey="day" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <Tooltip
                  contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 12 }}
                />
                <Bar dataKey="logins" fill="#0D5E6D" radius={[3, 3, 0, 0]} />
                <Bar dataKey="submissions" fill="#D4A84B" radius={[3, 3, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>
      </div>

      {/* Bottom row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Recent registrations */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.25 }} className="lg:col-span-1">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Recent Registrations</h3>
              <button className="text-xs text-brand-500 hover:text-brand-600 font-medium flex items-center gap-0.5 cursor-pointer">
                View all <ArrowUpRight size={12} />
              </button>
            </div>
            <div className="space-y-3">
              {recentRegistrations.map((reg) => (
                <div key={reg.name} className="flex items-center gap-3">
                  <Avatar name={reg.name} size="sm" />
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium text-ink truncate">{reg.name}</div>
                    <div className="text-xs text-ink-tertiary truncate">{reg.programme}</div>
                  </div>
                  <Badge variant={reg.status === 'approved' ? 'success' : 'warning'}>
                    {reg.status}
                  </Badge>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>

        {/* System alerts */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.3 }} className="lg:col-span-1">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">System Alerts</h3>
              <Activity size={16} className="text-ink-tertiary" />
            </div>
            <div className="space-y-3">
              {systemAlerts.map((alert, i) => (
                <div key={i} className="flex items-start gap-2.5">
                  <div className="mt-0.5">
                    {alert.type === 'warning' && <AlertTriangle size={15} className="text-warning" />}
                    {alert.type === 'success' && <CheckCircle2 size={15} className="text-success" />}
                    {alert.type === 'danger' && <AlertTriangle size={15} className="text-danger" />}
                    {alert.type === 'info' && <Clock size={15} className="text-info" />}
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm text-ink leading-snug">{alert.message}</p>
                    <p className="text-xs text-ink-tertiary mt-0.5">{alert.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>

        {/* Quick metrics */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.35 }} className="lg:col-span-1">
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Module Usage</h3>
            <div className="space-y-4">
              {[
                { label: 'Courses & Content', usage: 94 },
                { label: 'Assessments', usage: 87 },
                { label: 'Communication', usage: 72 },
                { label: 'Fee Management', usage: 68 },
                { label: 'Attendance', usage: 55 },
                { label: 'Library', usage: 41 },
                { label: 'Analytics', usage: 33 },
              ].map(mod => (
                <div key={mod.label}>
                  <div className="flex justify-between mb-1">
                    <span className="text-sm text-ink-secondary">{mod.label}</span>
                    <span className="text-xs text-ink-tertiary font-medium">{mod.usage}%</span>
                  </div>
                  <ProgressBar
                    value={mod.usage}
                    color={mod.usage > 80 ? 'brand' : mod.usage > 50 ? 'accent' : 'warning'}
                    size="sm"
                  />
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
