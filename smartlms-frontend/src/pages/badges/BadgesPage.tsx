import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Award, Star, Trophy, Search, Filter, ExternalLink,
  Clock, CheckCircle2, Lock, ChevronRight, Share2,
  Download, Layers, Target, Zap, BookOpen,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type CredentialType = 'badge' | 'micro_credential' | 'stackable';
type CredentialStatus = 'earned' | 'in_progress' | 'locked' | 'expired';
type Tab = 'all' | 'badge' | 'micro_credential' | 'stackable';

interface Credential {
  id: string;
  name: string;
  description: string;
  type: CredentialType;
  status: CredentialStatus;
  issuer: string;
  course?: string;
  courseCode?: string;
  earnedDate?: string;
  expiryDate?: string;
  competencies: string[];
  progress?: number;
  evidenceCount?: number;
  stacksInto?: string;
  badgesRequired?: number;
  badgesEarned?: number;
  verificationUrl?: string;
}

const CREDENTIALS: Credential[] = [
  { id: '1', name: 'Data Structures Mastery', description: 'Demonstrated expert-level knowledge of fundamental data structures', type: 'badge', status: 'earned', issuer: 'SmartLMS University', course: 'Data Structures & Algorithms', courseCode: 'CS301', earnedDate: 'Mar 15, 2026', competencies: ['Linked Lists', 'Binary Trees', 'Hash Tables'], evidenceCount: 4, verificationUrl: 'https://verify.smartlms.io/b/abc123' },
  { id: '2', name: 'SQL Expert', description: 'Mastered complex query writing and database optimization', type: 'badge', status: 'earned', issuer: 'SmartLMS University', course: 'Database Systems', courseCode: 'CS302', earnedDate: 'Feb 28, 2026', competencies: ['Complex Queries', 'Query Optimization'], evidenceCount: 3, verificationUrl: 'https://verify.smartlms.io/b/def456' },
  { id: '3', name: 'Algorithm Design', description: 'Applied divide-and-conquer and dynamic programming techniques', type: 'badge', status: 'in_progress', issuer: 'SmartLMS University', course: 'Data Structures & Algorithms', courseCode: 'CS301', competencies: ['Divide & Conquer', 'Dynamic Programming'], progress: 65 },
  { id: '4', name: 'Full-Stack Foundations', description: 'Complete 3-course pathway covering frontend, backend, and database skills', type: 'micro_credential', status: 'in_progress', issuer: 'SmartLMS University', competencies: ['HTML/CSS/JS', 'REST APIs', 'SQL'], progress: 72, badgesRequired: 5, badgesEarned: 3 },
  { id: '5', name: 'Network Security Basics', description: 'Understanding of network protocols and security fundamentals', type: 'badge', status: 'in_progress', issuer: 'SmartLMS University', course: 'Computer Networks', courseCode: 'CS305', competencies: ['TCP/IP', 'Encryption'], progress: 40 },
  { id: '6', name: 'Professional Software Engineer', description: 'Comprehensive credential covering the full software engineering lifecycle', type: 'stackable', status: 'in_progress', issuer: 'SmartLMS University', competencies: ['Data Structures', 'Algorithms', 'Databases', 'Networks', 'Software Design'], badgesRequired: 8, badgesEarned: 2, progress: 25 },
  { id: '7', name: 'Research Methodology', description: 'Demonstrated understanding of research design and academic writing', type: 'badge', status: 'locked', issuer: 'SmartLMS University', competencies: ['Research Design', 'Academic Writing', 'Ethics'] },
  { id: '8', name: 'Graph Algorithms', description: 'Applied BFS, DFS, shortest path, and spanning tree algorithms', type: 'badge', status: 'locked', issuer: 'SmartLMS University', course: 'Data Structures & Algorithms', courseCode: 'CS301', competencies: ['BFS/DFS', 'Dijkstra', 'MST'] },
];

const TYPE_META: Record<CredentialType, { label: string; icon: React.ReactNode; color: string; bg: string }> = {
  badge: { label: 'Badge', icon: <Award size={16} />, color: 'text-brand-500', bg: 'bg-brand-50' },
  micro_credential: { label: 'Micro-Credential', icon: <Layers size={16} />, color: 'text-accent-500', bg: 'bg-accent-50' },
  stackable: { label: 'Stackable', icon: <Trophy size={16} />, color: 'text-gold-500', bg: 'bg-gold-50' },
};

const STATUS_META: Record<CredentialStatus, { label: string; variant: 'success' | 'brand' | 'default' | 'danger' }> = {
  earned: { label: 'Earned', variant: 'success' },
  in_progress: { label: 'In Progress', variant: 'brand' },
  locked: { label: 'Locked', variant: 'default' },
  expired: { label: 'Expired', variant: 'danger' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function BadgesPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('all');
  const [search, setSearch] = useState('');

  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const filtered = CREDENTIALS.filter(c => {
    if (tab !== 'all' && c.type !== tab) return false;
    if (search && !c.name.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const earned = CREDENTIALS.filter(c => c.status === 'earned').length;
  const inProgress = CREDENTIALS.filter(c => c.status === 'in_progress').length;
  const totalCompetencies = [...new Set(CREDENTIALS.flatMap(c => c.competencies))].length;

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Badges & Credentials</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isInstructor ? 'Manage digital badges and micro-credential pathways' : 'Your earned and in-progress digital credentials'}
            </p>
          </div>
          {isInstructor && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><Layers size={15} /> Pathways</Button>
              <Button size="sm"><Award size={15} /> Create Badge</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Award size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Earned</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{earned}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Zap size={14} className="text-accent-400" /><span className="text-xs text-ink-tertiary">In Progress</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-accent-500">{inProgress}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Target size={14} className="text-success" /><span className="text-xs text-ink-tertiary">Skills Covered</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{totalCompetencies}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Star size={14} className="text-gold-500" /><span className="text-xs text-ink-tertiary">Open Badges 3.0</span></div>
          <div className="text-sm font-semibold text-ink mt-1">Compliant</div>
        </div>
      </motion.div>

      {/* Tabs + Search */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="flex items-center justify-between gap-4 flex-wrap">
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
          {(['all', 'badge', 'micro_credential', 'stackable'] as Tab[]).map(t => (
            <button key={t} onClick={() => setTab(t)}
              className={`px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${
                tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {t === 'all' ? 'All' : TYPE_META[t as CredentialType].label + 's'}
            </button>
          ))}
        </div>
        <div className="flex gap-2 flex-1 justify-end">
          <div className="relative max-w-xs flex-1">
            <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search credentials..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400" />
          </div>
          <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
            <Filter size={16} />
          </button>
        </div>
      </motion.div>

      {/* Credentials grid */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.14 }} className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {filtered.map((cred, i) => {
          const typeMeta = TYPE_META[cred.type];
          const statusMeta = STATUS_META[cred.status];
          const isEarned = cred.status === 'earned';
          const isLocked = cred.status === 'locked';

          return (
            <motion.div key={cred.id} initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 + i * 0.04 }}>
              <Card hover padding="none" className={`${isLocked ? 'opacity-60' : ''} ${isEarned ? 'border-success/30' : ''}`}>
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    {/* Badge visual */}
                    <div className={`w-14 h-14 rounded-2xl ${typeMeta.bg} flex items-center justify-center shrink-0 relative`}>
                      <span className={typeMeta.color}>{typeMeta.icon}</span>
                      {isEarned && (
                        <div className="absolute -top-1 -right-1 w-5 h-5 rounded-full bg-success flex items-center justify-center">
                          <CheckCircle2 size={12} className="text-white" />
                        </div>
                      )}
                      {isLocked && (
                        <div className="absolute -top-1 -right-1 w-5 h-5 rounded-full bg-sand-300 flex items-center justify-center">
                          <Lock size={10} className="text-ink-tertiary" />
                        </div>
                      )}
                    </div>

                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between gap-2">
                        <div>
                          <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{cred.name}</h3>
                          <p className="text-xs text-ink-tertiary mt-0.5 line-clamp-2">{cred.description}</p>
                        </div>
                        <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                      </div>

                      {cred.courseCode && (
                        <div className="mt-2">
                          <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{cred.courseCode}</span>
                        </div>
                      )}

                      {/* Competencies */}
                      <div className="flex items-center gap-1.5 mt-3 flex-wrap">
                        {cred.competencies.map(c => (
                          <span key={c} className="text-[10px] px-1.5 py-0.5 rounded bg-sand-100 text-ink-secondary font-medium">{c}</span>
                        ))}
                      </div>

                      {/* Progress bar */}
                      {cred.progress !== undefined && cred.status === 'in_progress' && (
                        <div className="mt-3 flex items-center gap-3">
                          <div className="flex-1">
                            <ProgressBar value={cred.progress} size="sm" color="brand" />
                          </div>
                          <span className="text-xs font-bold font-[family-name:var(--font-display)] text-brand-500">{cred.progress}%</span>
                        </div>
                      )}

                      {/* Stackable progress */}
                      {(cred.type === 'stackable' || cred.type === 'micro_credential') && cred.badgesRequired && (
                        <div className="mt-2 text-xs text-ink-tertiary">
                          <span className="font-medium text-ink-secondary">{cred.badgesEarned}</span> / {cred.badgesRequired} badges earned
                        </div>
                      )}

                      {/* Earned details */}
                      {isEarned && (
                        <div className="flex items-center gap-4 mt-3">
                          <div className="flex items-center gap-1 text-xs text-ink-tertiary">
                            <Clock size={11} />
                            <span>Earned {cred.earnedDate}</span>
                          </div>
                          {cred.evidenceCount && (
                            <div className="flex items-center gap-1 text-xs text-ink-tertiary">
                              <BookOpen size={11} />
                              <span>{cred.evidenceCount} evidence items</span>
                            </div>
                          )}
                        </div>
                      )}

                      {/* Actions for earned */}
                      {isEarned && (
                        <div className="flex items-center gap-2 mt-3">
                          <Button variant="ghost" size="sm"><Share2 size={12} /> Share</Button>
                          <Button variant="ghost" size="sm"><ExternalLink size={12} /> LinkedIn</Button>
                          <Button variant="ghost" size="sm"><Download size={12} /> PDF</Button>
                        </div>
                      )}
                    </div>

                    {!isLocked && !isEarned && <ChevronRight size={14} className="text-ink-placeholder shrink-0 mt-1" />}
                  </div>
                </div>
              </Card>
            </motion.div>
          );
        })}
      </motion.div>

      {filtered.length === 0 && (
        <Card className="text-center py-16">
          <Award size={36} className="mx-auto text-ink-placeholder mb-3" />
          <p className="text-ink-tertiary">No credentials found</p>
        </Card>
      )}
    </div>
  );
}
