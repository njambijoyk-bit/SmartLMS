import { useParams, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
  ArrowLeft, Users, BookOpen, Clock, Play, FileText, CheckCircle2,
  Lock, Video, PenTool, ChevronDown, ChevronRight, MessageSquare, BarChart3,
} from 'lucide-react';
import { useState } from 'react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { Avatar } from '../../components/ui/Avatar';

const COURSE = {
  code: 'CS301',
  title: 'Data Structures & Algorithms',
  description: 'A comprehensive course covering fundamental data structures (arrays, linked lists, trees, graphs, hash tables) and algorithm design techniques (divide and conquer, dynamic programming, greedy algorithms). Students learn to analyze algorithm complexity and select appropriate data structures for real-world problems.',
  instructor: 'Prof. James Mwangi',
  enrolled: 145,
  units: 12,
  progress: 72,
  rating: 4.6,
  semester: 'Semester 2, 2025/2026',
};

const UNITS = [
  {
    title: 'Unit 1: Arrays & Complexity Analysis',
    lessons: [
      { title: 'Introduction to Algorithm Analysis', type: 'video', duration: '24 min', completed: true },
      { title: 'Big-O, Big-Theta, Big-Omega', type: 'reading', duration: '15 min', completed: true },
      { title: 'Array Operations & Complexity', type: 'video', duration: '32 min', completed: true },
      { title: 'Practice: Array Problems', type: 'quiz', duration: '20 min', completed: true },
    ],
    completed: true,
  },
  {
    title: 'Unit 2: Linked Lists',
    lessons: [
      { title: 'Singly Linked Lists', type: 'video', duration: '28 min', completed: true },
      { title: 'Doubly Linked Lists', type: 'video', duration: '22 min', completed: true },
      { title: 'Linked List Problems', type: 'assignment', duration: '60 min', completed: true },
      { title: 'CAT 1: Arrays & Linked Lists', type: 'cat', duration: '45 min', completed: true },
    ],
    completed: true,
  },
  {
    title: 'Unit 3: Stacks & Queues',
    lessons: [
      { title: 'Stack Implementation', type: 'video', duration: '20 min', completed: true },
      { title: 'Queue Variants', type: 'video', duration: '25 min', completed: true },
      { title: 'Applications of Stacks', type: 'reading', duration: '12 min', completed: false },
      { title: 'Priority Queue & Heap', type: 'video', duration: '30 min', completed: false },
    ],
    completed: false,
  },
  {
    title: 'Unit 4: Binary Search Trees',
    lessons: [
      { title: 'BST Fundamentals', type: 'video', duration: '35 min', completed: false, locked: false },
      { title: 'AVL Trees', type: 'video', duration: '40 min', completed: false, locked: true },
      { title: 'Red-Black Trees', type: 'video', duration: '38 min', completed: false, locked: true },
      { title: 'Tree Traversals', type: 'quiz', duration: '25 min', completed: false, locked: true },
    ],
    completed: false,
  },
];

const typeIcon = (type: string) => {
  switch (type) {
    case 'video': return <Video size={14} className="text-brand-400" />;
    case 'reading': return <BookOpen size={14} className="text-gold-500" />;
    case 'quiz': return <PenTool size={14} className="text-accent-400" />;
    case 'assignment': return <FileText size={14} className="text-info" />;
    case 'cat': return <FileText size={14} className="text-danger" />;
    default: return <Play size={14} />;
  }
};

export function CourseDetailPage() {
  useParams();
  const navigate = useNavigate();
  const [expandedUnit, setExpandedUnit] = useState(2);
  const [activeTab, setActiveTab] = useState('content');

  return (
    <div className="space-y-6">
      {/* Back nav */}
      <button onClick={() => navigate('/courses')} className="flex items-center gap-1.5 text-sm text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
        <ArrowLeft size={16} /> Back to Courses
      </button>

      {/* Course header */}
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <Card padding="none">
          <div className="h-32 bg-brand-500 rounded-t-xl relative overflow-hidden">
            <div className="absolute inset-0 opacity-10">
              <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
                <defs>
                  <pattern id="courseGeo" width="60" height="60" patternUnits="userSpaceOnUse">
                    <path d="M30 0L60 30L30 60L0 30z" fill="none" stroke="white" strokeWidth="1" />
                    <circle cx="30" cy="30" r="8" fill="none" stroke="white" strokeWidth="0.5" />
                  </pattern>
                </defs>
                <rect width="100%" height="100%" fill="url(#courseGeo)" />
              </svg>
            </div>
            <div className="absolute bottom-4 left-6">
              <Badge variant="success" size="md">Published</Badge>
            </div>
          </div>
          <div className="p-6">
            <div className="flex items-start justify-between">
              <div>
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-sm font-semibold text-brand-500 font-[family-name:var(--font-display)]">{COURSE.code}</span>
                  <span className="text-xs text-ink-tertiary">{COURSE.semester}</span>
                </div>
                <h1 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink">{COURSE.title}</h1>
                <p className="text-sm text-ink-tertiary mt-2 max-w-2xl">{COURSE.description}</p>
              </div>
              <Button variant="accent">Continue Learning</Button>
            </div>
            <div className="flex items-center gap-6 mt-4 text-sm text-ink-secondary">
              <div className="flex items-center gap-2">
                <Avatar name={COURSE.instructor} size="sm" />
                <span>{COURSE.instructor}</span>
              </div>
              <span className="flex items-center gap-1"><Users size={14} /> {COURSE.enrolled} learners</span>
              <span className="flex items-center gap-1"><BookOpen size={14} /> {COURSE.units} units</span>
              <span className="flex items-center gap-1"><Clock size={14} /> ~48 hours</span>
            </div>
            <div className="mt-4 max-w-md">
              <ProgressBar value={COURSE.progress} size="md" showLabel color="brand" />
            </div>
          </div>
        </Card>
      </motion.div>

      {/* Tabs */}
      <div className="flex gap-1 border-b border-sand-200">
        {['content', 'assessments', 'forum', 'analytics'].map(tab => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`px-4 py-2.5 text-sm font-medium border-b-2 transition-colors capitalize cursor-pointer ${
              activeTab === tab
                ? 'border-brand-500 text-brand-600'
                : 'border-transparent text-ink-tertiary hover:text-ink'
            }`}
          >
            {tab === 'forum' && <MessageSquare size={14} className="inline mr-1.5" />}
            {tab === 'analytics' && <BarChart3 size={14} className="inline mr-1.5" />}
            {tab}
          </button>
        ))}
      </div>

      {/* Course content */}
      {activeTab === 'content' && (
        <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="space-y-3">
          {UNITS.map((unit, uIdx) => {
            const completedLessons = unit.lessons.filter(l => l.completed).length;
            const isExpanded = expandedUnit === uIdx;

            return (
              <Card key={uIdx} padding="none">
                <button
                  onClick={() => setExpandedUnit(isExpanded ? -1 : uIdx)}
                  className="w-full flex items-center justify-between p-4 cursor-pointer hover:bg-sand-50 transition-colors"
                >
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${unit.completed ? 'bg-success-light' : 'bg-sand-100'}`}>
                      {unit.completed ? <CheckCircle2 size={16} className="text-success" /> : <span className="text-xs font-bold text-ink-tertiary">{uIdx + 1}</span>}
                    </div>
                    <div className="text-left">
                      <div className="text-sm font-semibold text-ink">{unit.title}</div>
                      <div className="text-xs text-ink-tertiary">{completedLessons}/{unit.lessons.length} lessons completed</div>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="w-20">
                      <ProgressBar value={completedLessons} max={unit.lessons.length} size="sm" color={unit.completed ? 'success' : 'brand'} />
                    </div>
                    {isExpanded ? <ChevronDown size={16} className="text-ink-tertiary" /> : <ChevronRight size={16} className="text-ink-tertiary" />}
                  </div>
                </button>
                {isExpanded && (
                  <div className="border-t border-sand-200">
                    {unit.lessons.map((lesson, lIdx) => (
                      <div
                        key={lIdx}
                        className={`flex items-center gap-3 px-4 py-3 hover:bg-sand-50 transition-colors ${lIdx < unit.lessons.length - 1 ? 'border-b border-sand-100' : ''} ${(lesson as any).locked ? 'opacity-50' : 'cursor-pointer'}`}
                      >
                        <div className="w-6 flex justify-center">
                          {lesson.completed ? (
                            <CheckCircle2 size={16} className="text-success" />
                          ) : (lesson as any).locked ? (
                            <Lock size={14} className="text-ink-placeholder" />
                          ) : (
                            typeIcon(lesson.type)
                          )}
                        </div>
                        <div className="flex-1 min-w-0">
                          <span className="text-sm text-ink">{lesson.title}</span>
                        </div>
                        <Badge variant={lesson.type === 'cat' ? 'danger' : lesson.type === 'assignment' ? 'info' : 'default'} size="sm">
                          {lesson.type}
                        </Badge>
                        <span className="text-xs text-ink-tertiary flex items-center gap-1">
                          <Clock size={11} /> {lesson.duration}
                        </span>
                      </div>
                    ))}
                  </div>
                )}
              </Card>
            );
          })}
        </motion.div>
      )}
    </div>
  );
}
