import { useState } from 'react';
import { motion } from 'framer-motion';
import { Download, Filter, Search, ChevronDown } from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';
import { useAuth } from '../../context/AuthContext';
import type { GradeEntry } from '../../types';

const GRADE_DATA: GradeEntry[] = [
  { studentId: '1', studentName: 'Faith Wanjiku', cat1: 72, cat2: 68, assignment: 82, exam: 71, total: 73, grade: 'B' },
  { studentId: '2', studentName: 'Brian Otieno', cat1: 45, cat2: 38, assignment: 55, exam: null, total: null, grade: '-' },
  { studentId: '3', studentName: 'Grace Nyambura', cat1: 88, cat2: 92, assignment: 95, exam: 87, total: 90, grade: 'A' },
  { studentId: '4', studentName: 'Patrick Wafula', cat1: 52, cat2: 48, assignment: 60, exam: 55, total: 54, grade: 'D+' },
  { studentId: '5', studentName: 'Amina Hassan', cat1: 76, cat2: 81, assignment: 79, exam: 83, total: 80, grade: 'A-' },
  { studentId: '6', studentName: 'David Kimani', cat1: 63, cat2: 71, assignment: 68, exam: 72, total: 69, grade: 'B-' },
  { studentId: '7', studentName: 'Eunice Wambui', cat1: 91, cat2: 87, assignment: 93, exam: 89, total: 90, grade: 'A' },
  { studentId: '8', studentName: 'Kevin Njoroge', cat1: 58, cat2: 62, assignment: 65, exam: 60, total: 62, grade: 'C+' },
  { studentId: '9', studentName: 'Sarah Otieno', cat1: 84, cat2: 79, assignment: 88, exam: 82, total: 83, grade: 'A-' },
  { studentId: '10', studentName: 'Moses Kamau', cat1: 35, cat2: null, assignment: 42, exam: null, total: null, grade: '-' },
];

function gradeColor(score: number | null) {
  if (score === null) return 'text-ink-placeholder';
  if (score >= 80) return 'text-success font-semibold';
  if (score >= 60) return 'text-ink';
  if (score >= 50) return 'text-warning font-semibold';
  return 'text-danger font-semibold';
}

export function GradebookPage() {
  const { user } = useAuth();
  const [selectedCourse, setSelectedCourse] = useState('CS301');

  const isInstructor = user?.role === 'instructor' || user?.role === 'admin';

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Gradebook</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isInstructor ? 'Manage grades and track student performance' : 'View your grades and academic progress'}
            </p>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="secondary" size="sm"><Download size={14} /> Export</Button>
            {isInstructor && <Button size="sm"><Filter size={14} /> Filters</Button>}
          </div>
        </div>
      </motion.div>

      {/* Course selector */}
      <div className="flex items-center gap-3">
        <div className="relative">
          <select
            value={selectedCourse}
            onChange={e => setSelectedCourse(e.target.value)}
            className="appearance-none bg-surface-raised border border-sand-300 rounded-lg pl-3 pr-8 py-2 text-sm font-medium text-ink focus:outline-none focus:ring-2 focus:ring-brand-300 cursor-pointer"
          >
            <option value="CS301">CS301 — Data Structures & Algorithms</option>
            <option value="CS302">CS302 — Database Systems</option>
            <option value="CS401">CS401 — Machine Learning</option>
          </select>
          <ChevronDown size={14} className="absolute right-2.5 top-1/2 -translate-y-1/2 text-ink-tertiary pointer-events-none" />
        </div>
        <div className="flex items-center gap-2 text-xs text-ink-tertiary">
          <span>Weights:</span>
          <Badge variant="brand">CAT 1: 15%</Badge>
          <Badge variant="brand">CAT 2: 15%</Badge>
          <Badge variant="accent">Assignment: 20%</Badge>
          <Badge variant="default">Exam: 50%</Badge>
        </div>
      </div>

      {/* Grade stats */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <Card padding="sm">
          <div className="text-xs text-ink-tertiary">Class Average</div>
          <div className="text-xl font-bold font-[family-name:var(--font-display)] text-ink">72.6%</div>
        </Card>
        <Card padding="sm">
          <div className="text-xs text-ink-tertiary">Highest</div>
          <div className="text-xl font-bold font-[family-name:var(--font-display)] text-success">90%</div>
        </Card>
        <Card padding="sm">
          <div className="text-xs text-ink-tertiary">At Risk (&lt;50%)</div>
          <div className="text-xl font-bold font-[family-name:var(--font-display)] text-danger">2</div>
        </Card>
        <Card padding="sm">
          <div className="text-xs text-ink-tertiary">Pending Grades</div>
          <div className="text-xl font-bold font-[family-name:var(--font-display)] text-warning">3</div>
        </Card>
      </div>

      {/* Grade table */}
      <Card padding="none">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-sand-200 bg-sand-50">
                <th className="text-left px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Student</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">CAT 1 (15%)</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">CAT 2 (15%)</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Assignment (20%)</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Exam (50%)</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Total</th>
                <th className="text-center px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Grade</th>
              </tr>
            </thead>
            <tbody>
              {GRADE_DATA.map((entry, i) => (
                <tr key={entry.studentId} className={`border-b border-sand-100 hover:bg-sand-50 transition-colors ${i % 2 === 0 ? '' : 'bg-sand-50/30'}`}>
                  <td className="px-4 py-3">
                    <div className="flex items-center gap-2.5">
                      <Avatar name={entry.studentName} size="sm" />
                      <span className="text-sm font-medium text-ink">{entry.studentName}</span>
                    </div>
                  </td>
                  <td className={`text-center px-4 py-3 text-sm tabular-nums ${gradeColor(entry.cat1)}`}>
                    {entry.cat1 ?? '—'}
                  </td>
                  <td className={`text-center px-4 py-3 text-sm tabular-nums ${gradeColor(entry.cat2)}`}>
                    {entry.cat2 ?? '—'}
                  </td>
                  <td className={`text-center px-4 py-3 text-sm tabular-nums ${gradeColor(entry.assignment)}`}>
                    {entry.assignment ?? '—'}
                  </td>
                  <td className={`text-center px-4 py-3 text-sm tabular-nums ${gradeColor(entry.exam)}`}>
                    {entry.exam ?? '—'}
                  </td>
                  <td className={`text-center px-4 py-3 text-sm font-bold tabular-nums ${gradeColor(entry.total)}`}>
                    {entry.total ?? '—'}
                  </td>
                  <td className="text-center px-4 py-3">
                    <Badge
                      variant={
                        entry.grade === 'A' || entry.grade === 'A-' ? 'success' :
                        entry.grade === 'B' || entry.grade === 'B-' ? 'brand' :
                        entry.grade === '-' ? 'default' :
                        entry.total && entry.total < 50 ? 'danger' : 'warning'
                      }
                      size="md"
                    >
                      {entry.grade}
                    </Badge>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
