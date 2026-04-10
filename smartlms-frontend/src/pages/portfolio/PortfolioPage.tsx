import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  FolderOpen, Plus, ExternalLink, Download,
  Star, Eye, Lock, Globe,
  Code, FileText, Award, ChevronRight,
  Link2,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

type PortfolioType = 'academic' | 'project' | 'competency' | 'career';
type Visibility = 'private' | 'advisor' | 'employer' | 'public';

interface PortfolioItem {
  id: string;
  title: string;
  course: string;
  type: 'assignment' | 'project' | 'certificate' | 'external';
  date: string;
  score?: string;
  reflection?: string;
  endorsed: boolean;
  endorsedBy?: string;
  tags: string[];
}

const ITEMS: PortfolioItem[] = [
  { id: '1', title: 'Database Design — Library Management System', course: 'CS302', type: 'project', date: 'Mar 2026', score: 'A', reflection: 'Learned about normalisation trade-offs in real-world systems.', endorsed: true, endorsedBy: 'Dr. Achieng', tags: ['SQL', 'ERD', 'normalisation'] },
  { id: '2', title: 'AVL Tree Implementation in C', course: 'CS301', type: 'assignment', date: 'Feb 2026', score: 'A-', endorsed: false, tags: ['C', 'data-structures', 'algorithms'] },
  { id: '3', title: 'Network Packet Analyser', course: 'CS305', type: 'project', date: 'Mar 2026', score: 'B+', reflection: 'Built a Wireshark-like tool for educational purposes.', endorsed: true, endorsedBy: 'Dr. Omondi', tags: ['Python', 'networking', 'security'] },
  { id: '4', title: 'AWS Cloud Practitioner Certificate', course: 'External', type: 'certificate', date: 'Jan 2026', endorsed: false, tags: ['AWS', 'cloud', 'certification'] },
  { id: '5', title: 'Open Source Contribution — React Component Library', course: 'External', type: 'external', date: 'Dec 2025', reflection: 'Contributed accessible form components to a popular library.', endorsed: false, tags: ['React', 'open-source', 'TypeScript'] },
  { id: '6', title: 'Discrete Mathematics Proof Portfolio', course: 'MAT301', type: 'assignment', date: 'Mar 2026', score: 'B+', endorsed: false, tags: ['proofs', 'graph-theory', 'combinatorics'] },
];

const TYPE_META = {
  assignment: { icon: <FileText size={16} />, color: 'text-brand-500', bg: 'bg-brand-50' },
  project: { icon: <Code size={16} />, color: 'text-accent-400', bg: 'bg-accent-50' },
  certificate: { icon: <Award size={16} />, color: 'text-gold-500', bg: 'bg-gold-50' },
  external: { icon: <ExternalLink size={16} />, color: 'text-info', bg: 'bg-info-light' },
};

const VISIBILITY_META: Record<Visibility, { icon: React.ReactNode; label: string }> = {
  private: { icon: <Lock size={12} />, label: 'Private' },
  advisor: { icon: <Eye size={12} />, label: 'Advisor Only' },
  employer: { icon: <Link2 size={12} />, label: 'Shared Link' },
  public: { icon: <Globe size={12} />, label: 'Public' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function PortfolioPage() {
  const [view, setView] = useState<PortfolioType>('academic');
  const [visibility, setVisibility] = useState<Visibility>('private');

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">My Portfolio</h1>
            <p className="text-sm text-ink-tertiary mt-1">Curate your best work and build your professional profile</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm"><Download size={14} /> Export PDF</Button>
            <Button size="sm"><Plus size={14} /> Add Item</Button>
          </div>
        </div>
      </motion.div>

      {/* Portfolio type tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-1 bg-sand-100 rounded-xl p-1 w-fit">
          {(['academic', 'project', 'competency', 'career'] as PortfolioType[]).map(t => (
            <button key={t} onClick={() => setView(t)}
              className={`px-4 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${
                view === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {t}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.1 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><FolderOpen size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Total Items</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ITEMS.length}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Endorsed</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ITEMS.filter(i => i.endorsed).length}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Skills Tagged</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{new Set(ITEMS.flatMap(i => i.tags)).size}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Visibility</div>
          <div className="flex items-center gap-1.5 mt-1">
            <select
              value={visibility}
              onChange={e => setVisibility(e.target.value as Visibility)}
              className="text-sm font-semibold text-ink bg-transparent focus:outline-none cursor-pointer"
            >
              {Object.entries(VISIBILITY_META).map(([key, meta]) => (
                <option key={key} value={key}>{meta.label}</option>
              ))}
            </select>
          </div>
        </div>
      </motion.div>

      {/* Portfolio items */}
      <div className="space-y-3">
        {ITEMS.map((item, i) => {
          const meta = TYPE_META[item.type];
          return (
            <motion.div key={item.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 + i * 0.05 }}>
              <Card padding="none">
                <div className="flex gap-4 p-4 hover:bg-sand-50/50 transition-colors cursor-pointer">
                  <div className={`w-10 h-10 rounded-xl ${meta.bg} flex items-center justify-center ${meta.color} shrink-0`}>
                    {meta.icon}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 flex-wrap">
                      <h3 className="text-sm font-semibold text-ink">{item.title}</h3>
                      {item.endorsed && (
                        <span className="flex items-center gap-1 text-[10px] text-gold-500 font-semibold">
                          <Star size={10} fill="currentColor" /> Endorsed
                        </span>
                      )}
                    </div>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-xs text-ink-tertiary">{item.course}</span>
                      <span className="text-[10px] text-ink-placeholder">·</span>
                      <span className="text-xs text-ink-tertiary">{item.date}</span>
                      {item.score && (
                        <>
                          <span className="text-[10px] text-ink-placeholder">·</span>
                          <Badge variant="success" size="sm">{item.score}</Badge>
                        </>
                      )}
                    </div>
                    {item.reflection && (
                      <p className="text-xs text-ink-secondary mt-2 italic leading-relaxed">"{item.reflection}"</p>
                    )}
                    {item.endorsedBy && (
                      <div className="text-[10px] text-gold-500 mt-1">Endorsed by {item.endorsedBy}</div>
                    )}
                    <div className="flex flex-wrap gap-1.5 mt-2">
                      {item.tags.map(tag => (
                        <span key={tag} className="px-2 py-0.5 rounded-full text-[10px] font-medium bg-sand-100 text-ink-tertiary">{tag}</span>
                      ))}
                    </div>
                  </div>
                  <ChevronRight size={14} className="text-ink-placeholder shrink-0 mt-2" />
                </div>
              </Card>
            </motion.div>
          );
        })}
      </div>

      {/* Share section */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.4 }}>
        <Card>
          <div className="flex items-center justify-between">
            <div>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Share Your Portfolio</h3>
              <p className="text-xs text-ink-tertiary mt-0.5">Generate a verified, time-limited link for employers</p>
            </div>
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><Globe size={14} /> Public Profile</Button>
              <Button size="sm"><Link2 size={14} /> Generate Link</Button>
            </div>
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
