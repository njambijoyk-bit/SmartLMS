import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Users, BookOpen, TrendingUp, TrendingDown,
  CreditCard, Calendar, Shield, ChevronRight,
  Clock, AlertTriangle, CheckCircle2, Eye,
  Bell, GraduationCap,
} from 'lucide-react';
import { AreaChart, Area, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';

const CHILDREN = [
  { id: '1', name: 'Faith Wanjiku', regNo: 'CS/2022/001', programme: 'BSc Computer Science', year: 'Year 3', gpa: 3.4, status: 'active' },
  { id: '2', name: 'David Wanjiku', regNo: 'EE/2024/042', programme: 'BSc Electrical Engineering', year: 'Year 1', gpa: 3.1, status: 'active' },
];

const GRADE_TREND = [
  { sem: 'Y1S1', gpa: 3.2 }, { sem: 'Y1S2', gpa: 3.3 }, { sem: 'Y2S1', gpa: 3.1 },
  { sem: 'Y2S2', gpa: 3.5 }, { sem: 'Y3S1', gpa: 3.4 },
];

const ENROLLED_COURSES = [
  { code: 'CS301', name: 'Data Structures & Algorithms', instructor: 'Prof. Mwangi', grade: 'B+', progress: 72, attendance: 87 },
  { code: 'CS302', name: 'Database Systems', instructor: 'Dr. Achieng', grade: 'A-', progress: 68, attendance: 92 },
  { code: 'MAT301', name: 'Discrete Mathematics', instructor: 'Prof. Kariuki', grade: 'B', progress: 65, attendance: 78 },
  { code: 'CS305', name: 'Computer Networks', instructor: 'Dr. Omondi', grade: 'B+', progress: 58, attendance: 85 },
];

const NOTIFICATIONS = [
  { id: '1', text: 'Faith\'s exam card has been issued for Semester 1 exams', type: 'success' as const, time: '2h ago' },
  { id: '2', text: 'MAT301 attendance at 78% — below 80% threshold', type: 'warning' as const, time: '1d ago' },
  { id: '3', text: 'Fee payment of KSh 30,000 received successfully', type: 'success' as const, time: '3d ago' },
  { id: '4', text: 'CS301 CAT 2 scheduled for April 8, 2:00 PM', type: 'info' as const, time: '4d ago' },
];

const NOTIF_ICON = {
  success: <CheckCircle2 size={14} className="text-success" />,
  warning: <AlertTriangle size={14} className="text-warning" />,
  info: <Bell size={14} className="text-info" />,
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ParentsPortalPage() {
  const [selectedChild, setSelectedChild] = useState(CHILDREN[0]);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Parent Portal</h1>
            <p className="text-sm text-ink-tertiary mt-1">Monitor your child's academic progress and manage payments</p>
          </div>
          <Button variant="outline" size="sm"><CreditCard size={14} /> Pay Fees</Button>
        </div>
      </motion.div>

      {/* Child selector */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-3">
          {CHILDREN.map(child => (
            <button
              key={child.id}
              onClick={() => setSelectedChild(child)}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl border-2 transition-all cursor-pointer ${
                selectedChild.id === child.id
                  ? 'border-brand-300 bg-brand-50 shadow-sm'
                  : 'border-sand-200 bg-surface-raised hover:border-brand-200'
              }`}
            >
              <div className="w-10 h-10 rounded-full bg-brand-500 flex items-center justify-center text-white text-sm font-bold font-[family-name:var(--font-display)] shrink-0">
                {child.name.charAt(0)}
              </div>
              <div className="text-left">
                <div className="text-sm font-semibold text-ink">{child.name}</div>
                <div className="text-xs text-ink-tertiary">{child.programme} · {child.year}</div>
              </div>
            </button>
          ))}
        </div>
      </motion.div>

      {/* KPIs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><GraduationCap size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Current GPA</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{selectedChild.gpa}</div>
          <div className="flex items-center gap-1 text-xs text-success mt-1"><TrendingUp size={11} /> +0.1 from last sem</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Courses</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ENROLLED_COURSES.length}</div>
          <div className="text-xs text-ink-tertiary mt-1">enrolled this semester</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Avg Attendance</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">
            {Math.round(ENROLLED_COURSES.reduce((s, c) => s + c.attendance, 0) / ENROLLED_COURSES.length)}%
          </div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Fee Balance</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-warning">KSh 25,000</div>
          <div className="text-xs text-ink-tertiary mt-1">Due Apr 15, 2026</div>
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Grade trend chart */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.16 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">GPA Trend</h3>
              <Badge variant="default">5 semesters</Badge>
            </div>
            <ResponsiveContainer width="100%" height={180}>
              <AreaChart data={GRADE_TREND}>
                <defs>
                  <linearGradient id="gpaGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#0D5E6D" stopOpacity={0.15} />
                    <stop offset="100%" stopColor="#0D5E6D" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <XAxis dataKey="sem" axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <YAxis domain={[0, 4]} axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <Tooltip contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 11 }} />
                <Area type="monotone" dataKey="gpa" stroke="#0D5E6D" strokeWidth={2} fill="url(#gpaGrad)" name="GPA" />
              </AreaChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Notifications */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.2 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Recent Updates</h3>
            <div className="space-y-3">
              {NOTIFICATIONS.map(notif => (
                <div key={notif.id} className="flex items-start gap-2.5">
                  <div className="mt-0.5 shrink-0">{NOTIF_ICON[notif.type]}</div>
                  <div className="flex-1 min-w-0">
                    <p className="text-xs text-ink leading-relaxed">{notif.text}</p>
                    <span className="text-[10px] text-ink-placeholder">{notif.time}</span>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>

      {/* Enrolled courses */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.24 }}>
        <Card>
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Enrolled Courses</h3>
            <Badge variant="default">{ENROLLED_COURSES.length} courses</Badge>
          </div>
          <div className="space-y-3">
            {ENROLLED_COURSES.map((course, i) => (
              <motion.div key={course.code} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: 0.3 + i * 0.05 }}
                className="flex items-center gap-4 p-3.5 rounded-xl border border-sand-200 hover:border-brand-200 transition-colors cursor-pointer">
                <div className="w-10 h-10 rounded-xl bg-brand-50 flex items-center justify-center text-brand-600 text-xs font-bold font-[family-name:var(--font-display)] shrink-0">
                  {course.code.slice(-3)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-semibold text-ink">{course.code}</span>
                    <span className="text-xs text-ink-tertiary truncate">{course.name}</span>
                  </div>
                  <div className="text-xs text-ink-tertiary mt-0.5">{course.instructor}</div>
                  <div className="flex items-center gap-4 mt-2">
                    <div className="flex-1 max-w-24">
                      <div className="flex justify-between text-[10px] text-ink-tertiary mb-0.5">
                        <span>Progress</span><span>{course.progress}%</span>
                      </div>
                      <ProgressBar value={course.progress} size="sm" />
                    </div>
                    <span className={`text-xs font-medium ${course.attendance >= 80 ? 'text-success' : 'text-warning'}`}>
                      {course.attendance}% attendance
                    </span>
                  </div>
                </div>
                <div className="text-right shrink-0">
                  <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">{course.grade}</div>
                  <div className="text-[10px] text-ink-tertiary">Current</div>
                </div>
                <ChevronRight size={14} className="text-ink-placeholder shrink-0" />
              </motion.div>
            ))}
          </div>
        </Card>
      </motion.div>

      {/* Fee summary */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.28 }}>
        <Card>
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Fee Summary</h3>
            <Button variant="outline" size="sm"><Eye size={14} /> Full Statement</Button>
          </div>
          <div className="flex items-center gap-6 p-4 rounded-xl bg-sand-50 border border-sand-200">
            <div>
              <div className="text-xs text-ink-tertiary">Total Fee</div>
              <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">KSh 85,000</div>
            </div>
            <div>
              <div className="text-xs text-ink-tertiary">Paid</div>
              <div className="text-lg font-bold font-[family-name:var(--font-display)] text-success">KSh 60,000</div>
            </div>
            <div>
              <div className="text-xs text-ink-tertiary">Balance</div>
              <div className="text-lg font-bold font-[family-name:var(--font-display)] text-danger">KSh 25,000</div>
            </div>
            <div className="flex-1 max-w-32">
              <div className="flex justify-between text-[10px] text-ink-tertiary mb-1"><span>71% paid</span></div>
              <div className="h-2.5 bg-sand-200 rounded-full overflow-hidden">
                <div className="h-full bg-brand-500 rounded-full" style={{ width: '71%' }} />
              </div>
            </div>
            <Button size="sm" className="ml-auto"><CreditCard size={14} /> Pay Now</Button>
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
