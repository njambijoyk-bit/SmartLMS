import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  MessageSquare, Plus, Search, Pin, Lock,
  ThumbsUp, MessageCircle, Eye, CheckCircle2,
  ChevronRight, Clock,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';
import { useAuth } from '../../context/AuthContext';

type ThreadType = 'question' | 'discussion' | 'announcement';

interface Thread {
  id: string;
  title: string;
  type: ThreadType;
  author: string;
  authorRole: string;
  courseCode: string;
  courseName: string;
  content: string;
  replies: number;
  likes: number;
  views: number;
  pinned: boolean;
  locked: boolean;
  resolved: boolean;
  lastActivity: string;
  tags: string[];
}

const THREADS: Thread[] = [
  {
    id: '1', title: 'How to implement AVL tree rotations?', type: 'question',
    author: 'Faith Kamau', authorRole: 'Learner',
    courseCode: 'CS301', courseName: 'Data Structures',
    content: 'I understand the concept of balancing but the double rotation case is confusing. Can someone explain with an example?',
    replies: 8, likes: 12, views: 67, pinned: false, locked: false, resolved: true,
    lastActivity: '2h ago', tags: ['binary-trees', 'avl'],
  },
  {
    id: '2', title: 'CAT 2 Study Resources — Compiled', type: 'discussion',
    author: 'Prof. James Mwangi', authorRole: 'Instructor',
    courseCode: 'CS301', courseName: 'Data Structures',
    content: 'Here are the compiled resources for CAT 2 preparation. Good luck everyone!',
    replies: 23, likes: 45, views: 234, pinned: true, locked: false, resolved: false,
    lastActivity: '30m ago', tags: ['cat-prep', 'resources'],
  },
  {
    id: '3', title: 'Assignment 3 Deadline Extended to April 12', type: 'announcement',
    author: 'Dr. Achieng Odhiambo', authorRole: 'Instructor',
    courseCode: 'CS302', courseName: 'Database Systems',
    content: 'Due to the upcoming university event, the deadline for Assignment 3 has been extended.',
    replies: 5, likes: 34, views: 189, pinned: true, locked: true, resolved: false,
    lastActivity: '4h ago', tags: ['deadline', 'assignment-3'],
  },
  {
    id: '4', title: 'Difference between BFS and DFS — When to use which?', type: 'question',
    author: 'Daniel Mutua', authorRole: 'Learner',
    courseCode: 'CS301', courseName: 'Data Structures',
    content: 'Can someone explain the practical differences and when to prefer one over the other?',
    replies: 6, likes: 8, views: 45, pinned: false, locked: false, resolved: false,
    lastActivity: '5h ago', tags: ['graphs', 'algorithms'],
  },
  {
    id: '5', title: 'Group project team formation — looking for members', type: 'discussion',
    author: 'Grace Njeri', authorRole: 'Learner',
    courseCode: 'CS302', courseName: 'Database Systems',
    content: 'We need 2 more members for the group project. We plan to build a library management system.',
    replies: 11, likes: 3, views: 56, pinned: false, locked: false, resolved: false,
    lastActivity: '1d ago', tags: ['group-project', 'team'],
  },
  {
    id: '6', title: 'Hash table collision resolution — chaining vs open addressing', type: 'question',
    author: 'Brian Otieno', authorRole: 'Learner',
    courseCode: 'CS301', courseName: 'Data Structures',
    content: 'When would you choose chaining over open addressing? Is one strictly better?',
    replies: 4, likes: 6, views: 31, pinned: false, locked: false, resolved: true,
    lastActivity: '2d ago', tags: ['hash-tables'],
  },
];

const TYPE_META: Record<ThreadType, { variant: 'brand' | 'accent' | 'warning'; label: string }> = {
  question: { variant: 'brand', label: 'Question' },
  discussion: { variant: 'accent', label: 'Discussion' },
  announcement: { variant: 'warning', label: 'Announcement' },
};

const COURSES = ['All Courses', 'CS301 — Data Structures', 'CS302 — Database Systems', 'MAT301 — Discrete Mathematics'];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function DiscussionForumsPage() {
  const { user } = useAuth();
  const [courseFilter, setCourseFilter] = useState('All Courses');
  const [typeFilter, setTypeFilter] = useState<ThreadType | 'all'>('all');
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const filtered = THREADS.filter(t => {
    if (courseFilter !== 'All Courses' && !courseFilter.startsWith(t.courseCode)) return false;
    if (typeFilter !== 'all' && t.type !== typeFilter) return false;
    if (search && !t.title.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const pinned = filtered.filter(t => t.pinned);
  const regular = filtered.filter(t => !t.pinned);

  const totalThreads = THREADS.length;
  const unanswered = THREADS.filter(t => t.type === 'question' && !t.resolved).length;

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Discussion Forums</h1>
            <p className="text-sm text-ink-tertiary mt-1">Course discussions, Q&A, and announcements</p>
          </div>
          <Button size="sm"><Plus size={14} /> New Thread</Button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><MessageSquare size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Threads</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{totalThreads}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Total Replies</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{THREADS.reduce((s, t) => s + t.replies, 0)}</div>
        </div>
        <div className="bg-warning-light rounded-xl border border-warning/20 p-4">
          <div className="text-xs text-warning/70">Unanswered</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-warning">{unanswered}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Active Today</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">4</div>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
        <div className="flex flex-wrap items-center gap-2">
          <div className="relative flex-1 max-w-sm">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search discussions..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
          </div>
          <div className="flex gap-1.5 overflow-x-auto">
            {COURSES.map(c => (
              <button key={c} onClick={() => setCourseFilter(c)}
                className={`px-3 py-1.5 text-xs font-medium rounded-full whitespace-nowrap transition-all cursor-pointer ${
                  courseFilter === c ? 'bg-brand-500 text-white' : 'bg-surface-raised border border-sand-300 text-ink-secondary hover:border-brand-300'
                }`}>
                {c.split(' — ')[0]}
              </button>
            ))}
          </div>
          <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
            {(['all', 'question', 'discussion', 'announcement'] as const).map(t => (
              <button key={t} onClick={() => setTypeFilter(t)}
                className={`px-2.5 py-1.5 text-[11px] font-medium rounded-lg transition-all capitalize cursor-pointer ${
                  typeFilter === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
                }`}>
                {t}
              </button>
            ))}
          </div>
        </div>
      </motion.div>

      {/* Pinned threads */}
      {pinned.length > 0 && (
        <div className="space-y-2">
          <div className="flex items-center gap-1.5 px-1">
            <Pin size={12} className="text-brand-500" />
            <span className="text-xs font-semibold text-ink-tertiary uppercase tracking-wider">Pinned</span>
          </div>
          {pinned.map((thread, i) => (
            <ThreadCard key={thread.id} thread={thread} index={i} isInstructor={isInstructor} />
          ))}
        </div>
      )}

      {/* Regular threads */}
      <div className="space-y-2">
        {pinned.length > 0 && (
          <div className="flex items-center gap-1.5 px-1">
            <Clock size={12} className="text-ink-tertiary" />
            <span className="text-xs font-semibold text-ink-tertiary uppercase tracking-wider">Recent</span>
          </div>
        )}
        {regular.map((thread, i) => (
          <ThreadCard key={thread.id} thread={thread} index={i + pinned.length} isInstructor={isInstructor} />
        ))}
      </div>
    </div>
  );
}

function ThreadCard({ thread, index, isInstructor }: { thread: Thread; index: number; isInstructor: boolean }) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: 0.15 + index * 0.04 }}
    >
      <Card padding="none">
        <div className="flex gap-4 p-4 cursor-pointer hover:bg-sand-50/50 transition-colors">
          <Avatar name={thread.author} size="md" />
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <Badge variant={TYPE_META[thread.type].variant} size="sm">{TYPE_META[thread.type].label}</Badge>
              <span className="text-[10px] text-ink-placeholder px-1.5 py-0.5 bg-sand-100 rounded">{thread.courseCode}</span>
              {thread.pinned && <Pin size={10} className="text-brand-500" />}
              {thread.locked && <Lock size={10} className="text-ink-placeholder" />}
              {thread.resolved && (
                <span className="flex items-center gap-0.5 text-[10px] text-success font-medium"><CheckCircle2 size={10} /> Resolved</span>
              )}
            </div>
            <h3 className="text-sm font-semibold text-ink mt-1.5 leading-snug">{thread.title}</h3>
            <p className="text-xs text-ink-tertiary mt-1 line-clamp-1">{thread.content}</p>
            <div className="flex items-center gap-3 mt-2.5">
              <span className="text-[10px] text-ink-tertiary">
                <span className="font-medium text-ink-secondary">{thread.author}</span> · {thread.authorRole}
              </span>
              <span className="text-[10px] text-ink-placeholder">·</span>
              <span className="text-[10px] text-ink-tertiary">{thread.lastActivity}</span>
            </div>
            <div className="flex flex-wrap gap-1.5 mt-2">
              {thread.tags.map(tag => (
                <span key={tag} className="px-1.5 py-0.5 rounded text-[9px] font-medium bg-sand-100 text-ink-tertiary">#{tag}</span>
              ))}
            </div>
          </div>
          <div className="flex flex-col items-end gap-2 shrink-0 text-ink-tertiary">
            <div className="flex items-center gap-3">
              <span className="flex items-center gap-1 text-xs"><MessageCircle size={12} />{thread.replies}</span>
              <span className="flex items-center gap-1 text-xs"><ThumbsUp size={12} />{thread.likes}</span>
              <span className="flex items-center gap-1 text-xs"><Eye size={12} />{thread.views}</span>
            </div>
            <ChevronRight size={14} className="text-ink-placeholder mt-auto" />
          </div>
        </div>
      </Card>
    </motion.div>
  );
}
