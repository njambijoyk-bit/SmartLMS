import { motion } from 'framer-motion';
import {
  BookOpen, Clock, CheckCircle2, AlertCircle, Play, FileText,
  ArrowUpRight, Calendar, Award, Flame, Target,
} from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { Button } from '../../components/ui/Button';

const enrolledCourses = [
  { code: 'CS301', title: 'Data Structures & Algorithms', instructor: 'Prof. Mwangi', progress: 72, nextLesson: 'Binary Search Trees', color: 'bg-brand-500' },
  { code: 'CS302', title: 'Database Systems', instructor: 'Dr. Achieng', progress: 58, nextLesson: 'Normalization (3NF)', color: 'bg-accent-400' },
  { code: 'MAT301', title: 'Discrete Mathematics', instructor: 'Prof. Kariuki', progress: 85, nextLesson: 'Graph Coloring', color: 'bg-gold-500' },
  { code: 'CS305', title: 'Computer Networks', instructor: 'Dr. Omondi', progress: 34, nextLesson: 'TCP/IP Protocol', color: 'bg-brand-300' },
];

const upcomingDeadlines = [
  { title: 'CAT 2 — Data Structures', course: 'CS301', due: 'Tomorrow, 2:00 PM', type: 'cat' as const, urgent: true },
  { title: 'Assignment 3 — ER Diagrams', course: 'CS302', due: 'Apr 10, 11:59 PM', type: 'assignment' as const, urgent: false },
  { title: 'Lab Report 5', course: 'CS305', due: 'Apr 12, 5:00 PM', type: 'assignment' as const, urgent: false },
  { title: 'End of Semester Exam', course: 'MAT301', due: 'May 2, 9:00 AM', type: 'exam' as const, urgent: false },
];

const gradeHistory = [
  { assessment: 'CAT 1', cs301: 72, cs302: 65, mat301: 88 },
  { assessment: 'Assign 1', cs301: 78, cs302: 71, mat301: 85 },
  { assessment: 'CAT 2', cs301: 68, cs302: 74, mat301: 90 },
  { assessment: 'Assign 2', cs301: 82, cs302: 69, mat301: 87 },
];

const knowledgeState = [
  { concept: 'Binary Trees', mastery: 82 },
  { concept: 'Hash Tables', mastery: 71 },
  { concept: 'Graph Algorithms', mastery: 45 },
  { concept: 'Sorting Algorithms', mastery: 93 },
  { concept: 'Dynamic Programming', mastery: 28 },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function LearnerDashboard() {
  return (
    <div className="space-y-6">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">
          Welcome back, Faith
        </h1>
        <p className="text-sm text-ink-tertiary mt-1">
          You have <span className="text-danger font-semibold">1 CAT tomorrow</span> and <span className="text-brand-500 font-semibold">4 courses</span> in progress.
        </p>
      </motion.div>

      <div className="border-pattern w-full rounded-sm" />

      {/* Quick stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 text-center">
          <Flame size={20} className="text-accent-400 mx-auto mb-1" />
          <div className="text-2xl font-bold font-[family-name:var(--font-display)]">14</div>
          <div className="text-xs text-ink-tertiary">Day streak</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 text-center">
          <Target size={20} className="text-brand-500 mx-auto mb-1" />
          <div className="text-2xl font-bold font-[family-name:var(--font-display)]">3.42</div>
          <div className="text-xs text-ink-tertiary">Current GPA</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 text-center">
          <Award size={20} className="text-gold-500 mx-auto mb-1" />
          <div className="text-2xl font-bold font-[family-name:var(--font-display)]">7</div>
          <div className="text-xs text-ink-tertiary">Badges earned</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 text-center">
          <CheckCircle2 size={20} className="text-success mx-auto mb-1" />
          <div className="text-2xl font-bold font-[family-name:var(--font-display)]">62%</div>
          <div className="text-xs text-ink-tertiary">Sem progress</div>
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* My courses */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.15 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">My Courses</h3>
              <Button variant="ghost" size="sm">View all <ArrowUpRight size={12} /></Button>
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {enrolledCourses.map(course => (
                <div key={course.code} className="p-4 rounded-lg border border-sand-200 hover:border-brand-200 transition-all group cursor-pointer">
                  <div className="flex items-center gap-2.5 mb-3">
                    <div className={`w-10 h-10 rounded-lg ${course.color} flex items-center justify-center`}>
                      <span className="text-[10px] font-bold text-white font-[family-name:var(--font-display)]">{course.code}</span>
                    </div>
                    <div className="min-w-0">
                      <div className="text-sm font-medium text-ink truncate">{course.title}</div>
                      <div className="text-xs text-ink-tertiary">{course.instructor}</div>
                    </div>
                  </div>
                  <ProgressBar value={course.progress} size="md" showLabel />
                  <div className="flex items-center gap-1.5 mt-3 text-xs text-brand-500 font-medium">
                    <Play size={11} />
                    <span className="truncate">Next: {course.nextLesson}</span>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>

        {/* Deadlines */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Upcoming Deadlines</h3>
              <Calendar size={16} className="text-ink-tertiary" />
            </div>
            <div className="space-y-3">
              {upcomingDeadlines.map((item, i) => (
                <div key={i} className={`flex items-start gap-2.5 p-2.5 rounded-lg ${item.urgent ? 'bg-danger-light border border-danger/20' : 'hover:bg-sand-50'} transition-colors`}>
                  <div className="mt-0.5">
                    {item.type === 'cat' && <FileText size={15} className={item.urgent ? 'text-danger' : 'text-warning'} />}
                    {item.type === 'assignment' && <BookOpen size={15} className="text-brand-400" />}
                    {item.type === 'exam' && <AlertCircle size={15} className="text-accent-400" />}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm text-ink font-medium">{item.title}</div>
                    <div className="flex items-center gap-2 mt-0.5">
                      <span className="text-xs text-ink-tertiary">{item.course}</span>
                      <span className={`text-xs font-medium flex items-center gap-1 ${item.urgent ? 'text-danger' : 'text-ink-tertiary'}`}>
                        <Clock size={10} /> {item.due}
                      </span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Grade trend */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.25 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Grade Trend</h3>
            <ResponsiveContainer width="100%" height={200}>
              <LineChart data={gradeHistory}>
                <XAxis dataKey="assessment" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <YAxis domain={[40, 100]} axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <Tooltip contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 12 }} />
                <Line type="monotone" dataKey="cs301" stroke="#0D5E6D" strokeWidth={2} dot={{ r: 3 }} name="CS301" />
                <Line type="monotone" dataKey="cs302" stroke="#C75C2B" strokeWidth={2} dot={{ r: 3 }} name="CS302" />
                <Line type="monotone" dataKey="mat301" stroke="#D4A84B" strokeWidth={2} dot={{ r: 3 }} name="MAT301" />
              </LineChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Knowledge state (Julia DKT) */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.3 }}>
          <Card>
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Knowledge Map</h3>
                <Badge variant="accent">Julia AI</Badge>
              </div>
            </div>
            <div className="text-xs text-ink-tertiary mb-3">CS301 — Data Structures concept mastery</div>
            <div className="space-y-3">
              {knowledgeState.map(item => (
                <div key={item.concept}>
                  <div className="flex justify-between mb-1">
                    <span className="text-sm text-ink-secondary">{item.concept}</span>
                    <span className="text-xs font-semibold text-ink tabular-nums">{item.mastery}%</span>
                  </div>
                  <ProgressBar
                    value={item.mastery}
                    color={item.mastery >= 80 ? 'success' : item.mastery >= 50 ? 'brand' : 'danger'}
                    size="md"
                  />
                </div>
              ))}
            </div>
            <p className="text-xs text-ink-tertiary mt-4 p-2.5 bg-sand-100 rounded-lg">
              <span className="font-semibold text-ink-secondary">Julia recommends:</span> Focus on Dynamic Programming — 2 practice questions before your CAT tomorrow.
            </p>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
