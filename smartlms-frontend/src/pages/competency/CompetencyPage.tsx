import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Target, ChevronRight, Lock, CheckCircle2,
  Search,
  Zap, Star, Users,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type MasteryLevel = 'novice' | 'practitioner' | 'expert';
type CompetencyStatus = 'mastered' | 'in_progress' | 'locked' | 'not_started';

interface Competency {
  id: string;
  name: string;
  domain: string;
  level: MasteryLevel;
  mastery: number;
  status: CompetencyStatus;
  assessments: number;
  prerequisites: string[];
}

const DOMAINS = [
  { name: 'Data Structures', color: 'bg-brand-500', competencies: 6, mastered: 4 },
  { name: 'Algorithms', color: 'bg-accent-400', competencies: 5, mastered: 2 },
  { name: 'Database Systems', color: 'bg-gold-500', competencies: 4, mastered: 3 },
  { name: 'Computer Networks', color: 'bg-info', competencies: 4, mastered: 1 },
];

const COMPETENCIES: Competency[] = [
  { id: '1', name: 'Implement linked lists', domain: 'Data Structures', level: 'expert', mastery: 100, status: 'mastered', assessments: 3, prerequisites: [] },
  { id: '2', name: 'Implement binary search trees', domain: 'Data Structures', level: 'expert', mastery: 100, status: 'mastered', assessments: 4, prerequisites: ['1'] },
  { id: '3', name: 'Implement AVL tree rotations', domain: 'Data Structures', level: 'practitioner', mastery: 72, status: 'in_progress', assessments: 2, prerequisites: ['2'] },
  { id: '4', name: 'Implement hash tables with collision resolution', domain: 'Data Structures', level: 'expert', mastery: 100, status: 'mastered', assessments: 3, prerequisites: [] },
  { id: '5', name: 'Implement graph traversal (BFS/DFS)', domain: 'Data Structures', level: 'practitioner', mastery: 85, status: 'in_progress', assessments: 2, prerequisites: ['1'] },
  { id: '6', name: 'Implement heap and priority queue', domain: 'Data Structures', level: 'novice', mastery: 30, status: 'in_progress', assessments: 1, prerequisites: ['2'] },
  { id: '7', name: 'Analyse time complexity', domain: 'Algorithms', level: 'expert', mastery: 100, status: 'mastered', assessments: 5, prerequisites: [] },
  { id: '8', name: 'Apply divide and conquer', domain: 'Algorithms', level: 'practitioner', mastery: 65, status: 'in_progress', assessments: 2, prerequisites: ['7'] },
  { id: '9', name: 'Apply dynamic programming', domain: 'Algorithms', level: 'novice', mastery: 20, status: 'in_progress', assessments: 1, prerequisites: ['7', '8'] },
  { id: '10', name: 'Implement greedy algorithms', domain: 'Algorithms', level: 'novice', mastery: 0, status: 'not_started', assessments: 0, prerequisites: ['7'] },
  { id: '11', name: 'Apply graph algorithms', domain: 'Algorithms', level: 'novice', mastery: 0, status: 'locked', assessments: 0, prerequisites: ['5', '8'] },
  { id: '12', name: 'Design normalised schemas', domain: 'Database Systems', level: 'expert', mastery: 100, status: 'mastered', assessments: 3, prerequisites: [] },
  { id: '13', name: 'Write complex SQL queries', domain: 'Database Systems', level: 'expert', mastery: 100, status: 'mastered', assessments: 4, prerequisites: ['12'] },
  { id: '14', name: 'Optimise query performance', domain: 'Database Systems', level: 'practitioner', mastery: 78, status: 'in_progress', assessments: 2, prerequisites: ['13'] },
  { id: '15', name: 'Design distributed databases', domain: 'Database Systems', level: 'novice', mastery: 0, status: 'locked', assessments: 0, prerequisites: ['14'] },
];

const STATUS_META: Record<CompetencyStatus, { color: string; bg: string; icon: React.ReactNode }> = {
  mastered: { color: 'text-success', bg: 'bg-success', icon: <CheckCircle2 size={14} /> },
  in_progress: { color: 'text-brand-500', bg: 'bg-brand-500', icon: <Zap size={14} /> },
  not_started: { color: 'text-ink-tertiary', bg: 'bg-sand-300', icon: <Target size={14} /> },
  locked: { color: 'text-ink-placeholder', bg: 'bg-sand-200', icon: <Lock size={14} /> },
};

const LEVEL_META: Record<MasteryLevel, { label: string; variant: 'default' | 'brand' | 'success' }> = {
  novice: { label: 'Novice', variant: 'default' },
  practitioner: { label: 'Practitioner', variant: 'brand' },
  expert: { label: 'Expert', variant: 'success' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function CompetencyPage() {
  const { user } = useAuth();
  const [domainFilter, setDomainFilter] = useState<string>('all');
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const filtered = COMPETENCIES.filter(c => {
    if (domainFilter !== 'all' && c.domain !== domainFilter) return false;
    if (search && !c.name.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const totalMastered = COMPETENCIES.filter(c => c.status === 'mastered').length;
  const totalCompetencies = COMPETENCIES.length;
  const avgMastery = Math.round(COMPETENCIES.reduce((s, c) => s + c.mastery, 0) / totalCompetencies);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Competency Map</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isInstructor ? 'Track cohort competency progress and identify gaps' : 'Track your mastery across all competencies'}
            </p>
          </div>
          {isInstructor && <Button variant="outline" size="sm"><Users size={14} /> Cohort Heatmap</Button>}
        </div>
      </motion.div>

      {/* Overall progress */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Target size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Competencies</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{totalCompetencies}</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Mastered</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{totalMastered}</div>
          <div className="text-xs text-success/60 mt-0.5">{Math.round((totalMastered / totalCompetencies) * 100)}% complete</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">In Progress</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-brand-500">{COMPETENCIES.filter(c => c.status === 'in_progress').length}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Avg Mastery</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{avgMastery}%</div>
        </div>
      </motion.div>

      {/* Domain overview */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
          {DOMAINS.map(domain => (
            <button
              key={domain.name}
              onClick={() => setDomainFilter(domainFilter === domain.name ? 'all' : domain.name)}
              className={`p-4 rounded-xl border-2 text-left transition-all cursor-pointer ${
                domainFilter === domain.name
                  ? 'border-brand-300 bg-brand-50 shadow-sm'
                  : 'border-sand-200 bg-surface-raised hover:border-brand-200'
              }`}
            >
              <div className="flex items-center gap-2 mb-2">
                <div className={`w-3 h-3 rounded-full ${domain.color} shrink-0`} />
                <span className="text-xs font-semibold text-ink truncate">{domain.name}</span>
              </div>
              <div className="flex items-baseline gap-1">
                <span className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">{domain.mastered}</span>
                <span className="text-xs text-ink-tertiary">/ {domain.competencies}</span>
              </div>
              <div className="mt-2">
                <div className="h-1.5 bg-sand-200 rounded-full overflow-hidden">
                  <div className={`h-full rounded-full ${domain.color}`} style={{ width: `${(domain.mastered / domain.competencies) * 100}%` }} />
                </div>
              </div>
            </button>
          ))}
        </div>
      </motion.div>

      {/* Search */}
      <div className="relative max-w-sm">
        <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
        <input type="text" placeholder="Search competencies..." value={search} onChange={e => setSearch(e.target.value)}
          className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
      </div>

      {/* Competency nodes */}
      <div className="space-y-2">
        {filtered.map((comp, i) => {
          const statusMeta = STATUS_META[comp.status];
          const levelMeta = LEVEL_META[comp.level];
          const isLocked = comp.status === 'locked';

          return (
            <motion.div key={comp.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 + i * 0.03 }}>
              <Card padding="none">
                <div className={`flex items-center gap-4 p-4 ${isLocked ? 'opacity-50' : 'hover:bg-sand-50/50 cursor-pointer'} transition-colors`}>
                  {/* Status node */}
                  <div className={`w-10 h-10 rounded-full flex items-center justify-center shrink-0 ${
                    comp.status === 'mastered' ? 'bg-success-light text-success' :
                    comp.status === 'in_progress' ? 'bg-brand-50 text-brand-500' :
                    'bg-sand-100 text-ink-placeholder'
                  }`}>
                    {statusMeta.icon}
                  </div>

                  {/* Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className={`text-sm font-semibold ${isLocked ? 'text-ink-tertiary' : 'text-ink'}`}>{comp.name}</span>
                      <Badge variant={levelMeta.variant} size="sm">{levelMeta.label}</Badge>
                    </div>
                    <div className="flex items-center gap-2 mt-0.5">
                      <span className="text-xs text-ink-tertiary">{comp.domain}</span>
                      {comp.assessments > 0 && (
                        <span className="text-[10px] text-ink-placeholder">· {comp.assessments} assessments</span>
                      )}
                    </div>
                    {comp.status !== 'locked' && comp.status !== 'not_started' && (
                      <div className="flex items-center gap-3 mt-2">
                        <div className="flex-1 max-w-48">
                          <ProgressBar value={comp.mastery} size="sm" color={comp.status === 'mastered' ? 'success' : 'brand'} />
                        </div>
                        <span className={`text-xs font-bold font-[family-name:var(--font-display)] ${
                          comp.mastery === 100 ? 'text-success' : 'text-brand-500'
                        }`}>
                          {comp.mastery}%
                        </span>
                      </div>
                    )}
                    {isLocked && comp.prerequisites.length > 0 && (
                      <div className="flex items-center gap-1 mt-1.5 text-xs text-ink-placeholder">
                        <Lock size={10} /> Requires prerequisite competencies
                      </div>
                    )}
                  </div>

                  {comp.status === 'mastered' && (
                    <div className="shrink-0">
                      <div className="flex items-center gap-1 px-2 py-1 rounded-full bg-success-light text-success text-[10px] font-semibold">
                        <Star size={10} fill="currentColor" /> Mastered
                      </div>
                    </div>
                  )}

                  {!isLocked && <ChevronRight size={14} className="text-ink-placeholder shrink-0" />}
                </div>
              </Card>
            </motion.div>
          );
        })}
      </div>
    </div>
  );
}
