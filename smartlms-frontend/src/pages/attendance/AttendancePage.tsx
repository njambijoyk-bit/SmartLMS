import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Check, X, Download,
  AlertTriangle, Search,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

const COURSES = ['CS301 — Data Structures', 'CS302 — Database Systems', 'MAT301 — Discrete Mathematics', 'CS305 — Computer Networks'];

const STUDENTS = [
  { id: '1', name: 'Faith Kamau', regNo: 'CS/2022/001', attendance: [1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1], total: 87 },
  { id: '2', name: 'Brian Otieno', regNo: 'CS/2022/002', attendance: [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1], total: 81 },
  { id: '3', name: 'Mary Wanjiku', regNo: 'CS/2022/003', attendance: [1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0], total: 50 },
  { id: '4', name: 'Daniel Mutua', regNo: 'CS/2022/004', attendance: [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1], total: 94 },
  { id: '5', name: 'Rose Adhiambo', regNo: 'CS/2022/005', attendance: [0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0], total: 75 },
  { id: '6', name: 'Kevin Kamau', regNo: 'CS/2022/006', attendance: [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1], total: 38 },
  { id: '7', name: 'Grace Njeri', regNo: 'CS/2022/007', attendance: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], total: 100 },
  { id: '8', name: 'Samuel Ochieng', regNo: 'CS/2022/008', attendance: [1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1], total: 75 },
];

const WEEKS = ['Week 1', 'Week 2', 'Week 3', 'Week 4', 'Week 5', 'Week 6', 'Week 7', 'Week 8', 'Week 9', 'Week 10', 'Week 11', 'Week 12', 'Week 13', 'Week 14', 'Week 15', 'Week 16'];

const TODAY_STUDENTS = [
  { id: '1', name: 'Faith Kamau', regNo: 'CS/2022/001', status: 'present' as const },
  { id: '2', name: 'Brian Otieno', regNo: 'CS/2022/002', status: 'absent' as const },
  { id: '3', name: 'Mary Wanjiku', regNo: 'CS/2022/003', status: 'late' as const },
  { id: '4', name: 'Daniel Mutua', regNo: 'CS/2022/004', status: 'present' as const },
  { id: '5', name: 'Rose Adhiambo', regNo: 'CS/2022/005', status: 'present' as const },
  { id: '6', name: 'Kevin Kamau', regNo: 'CS/2022/006', status: 'absent' as const },
  { id: '7', name: 'Grace Njeri', regNo: 'CS/2022/007', status: 'present' as const },
  { id: '8', name: 'Samuel Ochieng', regNo: 'CS/2022/008', status: 'present' as const },
];

type AttendanceStatus = 'present' | 'absent' | 'late';


const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function AttendancePage() {
  const { user } = useAuth();
  const [selectedCourse, setSelectedCourse] = useState(COURSES[0]);
  const [tab, setTab] = useState<'today' | 'history'>('today');
  const [attendance, setAttendance] = useState<Record<string, AttendanceStatus>>(
    Object.fromEntries(TODAY_STUDENTS.map(s => [s.id, s.status]))
  );
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const presentCount = Object.values(attendance).filter(v => v === 'present').length;
  const absentCount = Object.values(attendance).filter(v => v === 'absent').length;
  const lateCount = Object.values(attendance).filter(v => v === 'late').length;

  const filteredHistory = STUDENTS.filter(s =>
    !search || s.name.toLowerCase().includes(search.toLowerCase()) || s.regNo.includes(search)
  );

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Attendance</h1>
            <p className="text-sm text-ink-tertiary mt-1">Track and manage class attendance</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm"><Download size={14} /> Export CSV</Button>
            {isInstructor && tab === 'today' && (
              <Button size="sm"><Check size={14} /> Save Attendance</Button>
            )}
          </div>
        </div>
      </motion.div>

      {/* Course selector */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-2 overflow-x-auto pb-1">
          {COURSES.map(c => (
            <button key={c} onClick={() => setSelectedCourse(c)}
              className={`px-3 py-1.5 text-xs font-medium rounded-full whitespace-nowrap transition-all cursor-pointer ${selectedCourse === c ? 'bg-brand-500 text-white' : 'bg-surface-raised border border-sand-300 text-ink-secondary hover:border-brand-300'}`}>
              {c.split(' — ')[0]}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Stats */}
      {tab === 'today' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="grid grid-cols-3 gap-3">
          <div className="bg-success-light rounded-xl border border-success/20 p-4 text-center">
            <div className="text-3xl font-bold font-[family-name:var(--font-display)] text-success">{presentCount}</div>
            <div className="text-xs text-success/70 font-medium mt-0.5">Present</div>
          </div>
          <div className="bg-danger-light rounded-xl border border-danger/20 p-4 text-center">
            <div className="text-3xl font-bold font-[family-name:var(--font-display)] text-danger">{absentCount}</div>
            <div className="text-xs text-danger/70 font-medium mt-0.5">Absent</div>
          </div>
          <div className="bg-warning-light rounded-xl border border-warning/20 p-4 text-center">
            <div className="text-3xl font-bold font-[family-name:var(--font-display)] text-warning">{lateCount}</div>
            <div className="text-xs text-warning/70 font-medium mt-0.5">Late</div>
          </div>
        </motion.div>
      )}

      {/* Tabs */}
      <div className="flex bg-sand-100 rounded-xl p-1 w-fit gap-0.5">
        {(['today', 'history'] as const).map(t => (
          <button key={t} onClick={() => setTab(t)}
            className={`px-4 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
            {t === 'today' ? "Today's Class" : 'Attendance History'}
          </button>
        ))}
      </div>

      {/* Today's attendance */}
      {tab === 'today' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }}>
          <Card padding="none">
            <div className="p-4 border-b border-sand-200 flex items-center justify-between">
              <div>
                <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">{selectedCourse.split(' — ')[0]} — Today</h3>
                <p className="text-xs text-ink-tertiary">Apr 5, 2026 · {isInstructor ? 'Click to toggle status' : 'Readonly view'}</p>
              </div>
              <div className="text-xs text-ink-tertiary font-medium">
                {TODAY_STUDENTS.length} students
              </div>
            </div>
            <div className="divide-y divide-sand-100">
              {TODAY_STUDENTS.map((student, i) => {
                const status = attendance[student.id];
                return (
                  <motion.div key={student.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
                    className="flex items-center gap-4 px-5 py-3.5">
                    <div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold text-white shrink-0 ${status === 'present' ? 'bg-brand-500' : status === 'absent' ? 'bg-sand-300' : 'bg-warning'}`}>
                      {student.name.charAt(0)}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="text-sm font-medium text-ink">{student.name}</div>
                      <div className="text-xs text-ink-tertiary">{student.regNo}</div>
                    </div>
                    {isInstructor ? (
                      <div className="flex gap-1.5">
                        {(['present', 'late', 'absent'] as AttendanceStatus[]).map(s => (
                          <button
                            key={s}
                            onClick={() => setAttendance(prev => ({ ...prev, [student.id]: s }))}
                            className={`px-2.5 py-1 rounded-lg text-xs font-medium transition-all cursor-pointer ${
                              status === s
                                ? s === 'present' ? 'bg-success text-white' : s === 'absent' ? 'bg-danger text-white' : 'bg-warning text-white'
                                : 'bg-sand-100 text-ink-tertiary hover:bg-sand-200'
                            }`}
                          >
                            {s.charAt(0).toUpperCase() + s.slice(1)}
                          </button>
                        ))}
                      </div>
                    ) : (
                      <Badge variant={status === 'present' ? 'success' : status === 'absent' ? 'danger' : 'warning'}>
                        {status}
                      </Badge>
                    )}
                  </motion.div>
                );
              })}
            </div>
          </Card>
        </motion.div>
      )}

      {/* Attendance history grid */}
      {tab === 'history' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }}>
          <div className="flex gap-2 mb-4">
            <div className="relative flex-1 max-w-sm">
              <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
              <input type="text" placeholder="Search student..." value={search} onChange={e => setSearch(e.target.value)}
                className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
            </div>
          </div>
          <Card padding="none">
            <div className="overflow-x-auto">
              <table className="w-full text-xs">
                <thead>
                  <tr className="border-b border-sand-200">
                    <th className="text-left px-5 py-3 font-semibold text-ink-tertiary w-40">Student</th>
                    {WEEKS.map(w => (
                      <th key={w} className="text-center px-1 py-3 font-medium text-ink-tertiary whitespace-nowrap min-w-[36px]">{w.replace('Week ', 'W')}</th>
                    ))}
                    <th className="text-center px-3 py-3 font-semibold text-ink-tertiary">Total</th>
                    <th className="text-center px-3 py-3 font-semibold text-ink-tertiary">Status</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-sand-100">
                  {filteredHistory.map((student, i) => (
                    <motion.tr key={student.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
                      className="hover:bg-sand-50 transition-colors">
                      <td className="px-5 py-3">
                        <div className="font-medium text-ink">{student.name}</div>
                        <div className="text-ink-tertiary">{student.regNo}</div>
                      </td>
                      {student.attendance.map((a, wi) => (
                        <td key={wi} className="text-center px-1 py-3">
                          <div className={`w-6 h-6 rounded-full mx-auto flex items-center justify-center ${a === 1 ? 'bg-success-light' : 'bg-danger-light'}`}>
                            {a === 1 ? <Check size={10} className="text-success" /> : <X size={10} className="text-danger" />}
                          </div>
                        </td>
                      ))}
                      <td className="text-center px-3 py-3">
                        <span className={`font-bold font-[family-name:var(--font-display)] ${student.total >= 75 ? 'text-success' : student.total >= 60 ? 'text-warning' : 'text-danger'}`}>
                          {student.total}%
                        </span>
                      </td>
                      <td className="text-center px-3 py-3">
                        {student.total < 60 ? (
                          <span className="flex items-center gap-1 text-danger text-[10px] font-semibold justify-center">
                            <AlertTriangle size={10} /> At Risk
                          </span>
                        ) : student.total >= 75 ? (
                          <span className="text-success text-[10px] font-semibold">Good</span>
                        ) : (
                          <span className="text-warning text-[10px] font-semibold">Watch</span>
                        )}
                      </td>
                    </motion.tr>
                  ))}
                </tbody>
              </table>
            </div>
          </Card>
        </motion.div>
      )}
    </div>
  );
}
