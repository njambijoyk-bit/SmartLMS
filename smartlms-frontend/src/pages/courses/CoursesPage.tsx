import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
  Search, Plus, Filter, Grid3X3, List, Users, BookOpen,
  MoreHorizontal, Clock, ChevronRight,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { Avatar } from '../../components/ui/Avatar';
import { useAuth } from '../../context/AuthContext';
import type { Course } from '../../types';

const COURSES: Course[] = [
  { id: '1', code: 'CS301', title: 'Data Structures & Algorithms', description: 'Core computer science course covering fundamental data structures and algorithm design.', instructor: 'Prof. James Mwangi', enrolledCount: 145, status: 'published', category: 'Computer Science', units: 12, progress: 72 },
  { id: '2', code: 'CS302', title: 'Database Systems', description: 'Relational databases, SQL, normalization, and distributed database concepts.', instructor: 'Dr. Achieng Odhiambo', enrolledCount: 198, status: 'published', category: 'Computer Science', units: 10, progress: 58 },
  { id: '3', code: 'MAT301', title: 'Discrete Mathematics', description: 'Graph theory, combinatorics, logic, and proofs for computer science.', instructor: 'Prof. Kariuki Maina', enrolledCount: 132, status: 'published', category: 'Mathematics', units: 14, progress: 85 },
  { id: '4', code: 'CS305', title: 'Computer Networks', description: 'TCP/IP, routing protocols, network security, and distributed systems fundamentals.', instructor: 'Dr. Omondi Were', enrolledCount: 89, status: 'published', category: 'Computer Science', units: 11, progress: 34 },
  { id: '5', code: 'CS401', title: 'Machine Learning Fundamentals', description: 'Supervised and unsupervised learning, neural networks, and practical ML applications.', instructor: 'Prof. James Mwangi', enrolledCount: 68, status: 'published', category: 'Computer Science', units: 15, progress: 42 },
  { id: '6', code: 'CS310', title: 'Software Engineering', description: 'SDLC methodologies, design patterns, testing, and project management.', instructor: 'Dr. Wambui Njeru', enrolledCount: 156, status: 'published', category: 'Computer Science', units: 13 },
  { id: '7', code: 'CS320', title: 'Operating Systems', description: 'Process management, memory management, file systems, and system-level programming.', instructor: 'Prof. Ochieng Otieno', enrolledCount: 112, status: 'draft', category: 'Computer Science', units: 12 },
  { id: '8', code: 'BUS201', title: 'Introduction to Accounting', description: 'Financial statements, double-entry bookkeeping, and management accounting basics.', instructor: 'Dr. Fatima Ali', enrolledCount: 234, status: 'published', category: 'Business', units: 10, progress: 91 },
];

const COLORS = ['bg-brand-500', 'bg-accent-400', 'bg-gold-500', 'bg-brand-300', 'bg-accent-300', 'bg-brand-700', 'bg-gold-400', 'bg-accent-500'];

export function CoursesPage() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [view, setView] = useState<'grid' | 'list'>('grid');
  const [filter, setFilter] = useState('all');

  const filtered = filter === 'all' ? COURSES : COURSES.filter(c => c.status === filter);

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Courses</h1>
            <p className="text-sm text-ink-tertiary mt-1">{COURSES.length} courses available this semester</p>
          </div>
          {user?.role === 'admin' && (
            <Button><Plus size={16} /> Create Course</Button>
          )}
        </div>
      </motion.div>

      {/* Filters bar */}
      <div className="flex items-center justify-between gap-4">
        <div className="relative flex-1 max-w-sm">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
          <input
            type="text"
            placeholder="Search courses..."
            className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
          />
        </div>
        <div className="flex items-center gap-2">
          <div className="flex bg-sand-100 rounded-lg p-0.5">
            {['all', 'published', 'draft'].map(f => (
              <button
                key={f}
                onClick={() => setFilter(f)}
                className={`px-3 py-1.5 text-xs font-medium rounded-md transition-colors capitalize cursor-pointer ${
                  filter === f ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                {f}
              </button>
            ))}
          </div>
          <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
            <Filter size={16} />
          </button>
          <div className="flex bg-sand-100 rounded-lg p-0.5">
            <button onClick={() => setView('grid')} className={`p-1.5 rounded-md cursor-pointer ${view === 'grid' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}>
              <Grid3X3 size={16} />
            </button>
            <button onClick={() => setView('list')} className={`p-1.5 rounded-md cursor-pointer ${view === 'list' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}>
              <List size={16} />
            </button>
          </div>
        </div>
      </div>

      {/* Course grid */}
      {view === 'grid' ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.3 }}
          className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
        >
          {filtered.map((course, i) => (
            <motion.div
              key={course.id}
              initial={{ opacity: 0, y: 15 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: i * 0.05 }}
            >
              <Card hover padding="none" onClick={() => navigate(`/courses/${course.id}`)}>
                <div className={`h-24 ${COLORS[i % COLORS.length]} rounded-t-xl relative overflow-hidden`}>
                  <div className="absolute inset-0 opacity-10">
                    <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
                      <defs>
                        <pattern id={`p${i}`} width="40" height="40" patternUnits="userSpaceOnUse">
                          <circle cx="20" cy="20" r="12" fill="none" stroke="white" strokeWidth="1" />
                          <path d="M0 20h40M20 0v40" stroke="white" strokeWidth="0.5" />
                        </pattern>
                      </defs>
                      <rect width="100%" height="100%" fill={`url(#p${i})`} />
                    </svg>
                  </div>
                  <div className="absolute bottom-3 left-4">
                    <span className="text-white text-lg font-bold font-[family-name:var(--font-display)]">{course.code}</span>
                  </div>
                  <div className="absolute top-3 right-3">
                    <Badge variant={course.status === 'published' ? 'success' : 'warning'}>{course.status}</Badge>
                  </div>
                </div>
                <div className="p-4">
                  <h4 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] line-clamp-1">{course.title}</h4>
                  <p className="text-xs text-ink-tertiary mt-1 line-clamp-2">{course.description}</p>
                  <div className="flex items-center gap-2 mt-3 text-xs text-ink-tertiary">
                    <Users size={12} />
                    <span>{course.enrolledCount} enrolled</span>
                    <span className="text-sand-300">|</span>
                    <BookOpen size={12} />
                    <span>{course.units} units</span>
                  </div>
                  {course.progress !== undefined && (
                    <div className="mt-3">
                      <ProgressBar value={course.progress} size="sm" showLabel />
                    </div>
                  )}
                </div>
              </Card>
            </motion.div>
          ))}
        </motion.div>
      ) : (
        <Card padding="none">
          <div className="divide-y divide-sand-200">
            {filtered.map(course => (
              <div key={course.id} className="flex items-center gap-4 p-4 hover:bg-sand-50 transition-colors cursor-pointer" onClick={() => navigate(`/courses/${course.id}`)}>
                <div className={`w-12 h-12 rounded-lg ${COLORS[COURSES.indexOf(course) % COLORS.length]} flex items-center justify-center shrink-0`}>
                  <span className="text-xs font-bold text-white font-[family-name:var(--font-display)]">{course.code}</span>
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-semibold text-ink">{course.title}</span>
                    <Badge variant={course.status === 'published' ? 'success' : 'warning'}>{course.status}</Badge>
                  </div>
                  <div className="text-xs text-ink-tertiary mt-0.5">{course.instructor}</div>
                </div>
                <div className="flex items-center gap-6 text-sm text-ink-tertiary">
                  <span className="flex items-center gap-1"><Users size={13} /> {course.enrolledCount}</span>
                  <span className="flex items-center gap-1"><BookOpen size={13} /> {course.units} units</span>
                  {course.progress !== undefined && (
                    <div className="w-24">
                      <ProgressBar value={course.progress} size="sm" showLabel />
                    </div>
                  )}
                </div>
                <ChevronRight size={16} className="text-ink-tertiary" />
              </div>
            ))}
          </div>
        </Card>
      )}
    </div>
  );
}
