import { motion } from 'framer-motion';
import {
  BookOpen, Users, ClipboardCheck, MessageSquare, Clock,
  ArrowUpRight, AlertTriangle, Star, FileText,
} from 'lucide-react';
import { StatCard } from '../../components/ui/StatCard';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Avatar } from '../../components/ui/Avatar';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { Button } from '../../components/ui/Button';

const myCourses = [
  { code: 'CS301', title: 'Data Structures & Algorithms', enrolled: 145, submissions: 23, progress: 65 },
  { code: 'CS201', title: 'Object-Oriented Programming', enrolled: 210, submissions: 45, progress: 78 },
  { code: 'CS401', title: 'Machine Learning Fundamentals', enrolled: 68, submissions: 12, progress: 42 },
];

const pendingTasks = [
  { type: 'grading', label: 'Grade CAT 2 — CS301', count: 23, due: 'Due tomorrow' },
  { type: 'grading', label: 'Grade Assignment 3 — CS201', count: 45, due: 'Due in 3 days' },
  { type: 'review', label: 'Review forum posts — CS401', count: 8, due: '4 unanswered' },
  { type: 'content', label: 'Publish Week 12 materials — CS301', count: null, due: 'Scheduled Mon' },
];

const atRiskStudents = [
  { name: 'Brian Otieno', course: 'CS301', risk: 78, factor: 'No login for 12 days' },
  { name: 'Grace Nyambura', course: 'CS201', risk: 65, factor: 'Last 2 CATs below 40%' },
  { name: 'Patrick Wafula', course: 'CS301', risk: 58, factor: 'Attendance dropped to 45%' },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function InstructorDashboard() {
  return (
    <div className="space-y-6">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">
          Good afternoon, Prof. Mwangi
        </h1>
        <p className="text-sm text-ink-tertiary mt-1">
          You have <span className="text-accent-400 font-semibold">68 submissions</span> to grade and <span className="text-brand-500 font-semibold">3 students</span> flagged at-risk.
        </p>
      </motion.div>

      <div className="border-pattern w-full rounded-sm" />

      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard label="My Courses" value="3" icon={<BookOpen size={18} className="text-brand-500" />} />
        <StatCard label="Total Students" value="423" change={8} trend="up" icon={<Users size={18} className="text-accent-400" />} accentColor="bg-accent-400" />
        <StatCard label="Pending Grading" value="68" icon={<ClipboardCheck size={18} className="text-warning" />} accentColor="bg-warning" />
        <StatCard label="Forum Questions" value="8" icon={<MessageSquare size={18} className="text-info" />} accentColor="bg-info" />
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* My courses */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.15 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">My Courses</h3>
              <Button variant="ghost" size="sm">View all <ArrowUpRight size={12} /></Button>
            </div>
            <div className="space-y-4">
              {myCourses.map(course => (
                <div key={course.code} className="flex items-center gap-4 p-3 rounded-lg hover:bg-sand-50 transition-colors">
                  <div className="w-11 h-11 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
                    <span className="text-xs font-bold text-brand-600 font-[family-name:var(--font-display)]">{course.code}</span>
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium text-ink">{course.title}</div>
                    <div className="flex items-center gap-3 mt-1 text-xs text-ink-tertiary">
                      <span className="flex items-center gap-1"><Users size={11} /> {course.enrolled}</span>
                      <span className="flex items-center gap-1"><FileText size={11} /> {course.submissions} to grade</span>
                    </div>
                    <div className="mt-2">
                      <ProgressBar value={course.progress} size="sm" showLabel />
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>

        {/* Pending tasks */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Pending Tasks</h3>
            <div className="space-y-3">
              {pendingTasks.map((task, i) => (
                <div key={i} className="flex items-start gap-2.5 p-2.5 rounded-lg hover:bg-sand-50 transition-colors">
                  <div className="mt-0.5">
                    {task.type === 'grading' && <ClipboardCheck size={15} className="text-warning" />}
                    {task.type === 'review' && <MessageSquare size={15} className="text-info" />}
                    {task.type === 'content' && <BookOpen size={15} className="text-brand-400" />}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm text-ink font-medium">{task.label}</div>
                    <div className="flex items-center gap-2 mt-0.5">
                      <span className="text-xs text-ink-tertiary flex items-center gap-1">
                        <Clock size={10} /> {task.due}
                      </span>
                      {task.count && <Badge variant="warning">{task.count}</Badge>}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>

      {/* At-risk students from Julia */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.25 }}>
        <Card>
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">At-Risk Students</h3>
              <Badge variant="accent">Julia AI</Badge>
            </div>
            <Button variant="ghost" size="sm">View all <ArrowUpRight size={12} /></Button>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {atRiskStudents.map(student => (
              <div key={student.name} className="p-4 rounded-lg border border-sand-200 hover:border-warning/50 transition-colors">
                <div className="flex items-center gap-2.5 mb-3">
                  <Avatar name={student.name} size="sm" />
                  <div>
                    <div className="text-sm font-medium text-ink">{student.name}</div>
                    <div className="text-xs text-ink-tertiary">{student.course}</div>
                  </div>
                </div>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs text-ink-tertiary">Risk score</span>
                  <span className="text-sm font-bold text-danger font-[family-name:var(--font-display)]">{student.risk}%</span>
                </div>
                <ProgressBar value={student.risk} color="danger" size="sm" />
                <div className="flex items-center gap-1.5 mt-2.5">
                  <AlertTriangle size={12} className="text-warning" />
                  <span className="text-xs text-ink-secondary">{student.factor}</span>
                </div>
                <div className="flex gap-2 mt-3">
                  <Button variant="secondary" size="sm" className="flex-1 text-xs">Message</Button>
                  <Button variant="ghost" size="sm" className="text-xs"><Star size={12} /></Button>
                </div>
              </div>
            ))}
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
