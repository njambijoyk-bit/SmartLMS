import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Search, Filter, Plus, BookOpen, FileText, Link2,
  Video, Database, Download, ExternalLink, Star,
  Grid3X3, List, Tag, ChevronRight, Book,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type ResourceType = 'PDF' | 'Video' | 'Link' | 'EPUB' | 'Dataset';

interface Resource {
  id: string;
  title: string;
  description: string;
  type: ResourceType;
  collection: string;
  author: string;
  year: number;
  tags: string[];
  accessLevel: 'public' | 'enrolled' | 'role-gated';
  downloads: number;
  rating: number;
  linkedCourses: string[];
  size?: string;
}

const COLLECTIONS = ['All', 'Engineering Resources', 'Past Papers', 'Research Articles', 'Textbooks', 'Lab Manuals'];

const RESOURCES: Resource[] = [
  { id: '1', title: 'Introduction to Algorithms (4th Ed.)', description: 'The definitive textbook on algorithm design and analysis. Covers sorting, dynamic programming, graph algorithms, and NP-completeness.', type: 'PDF', collection: 'Textbooks', author: 'Cormen, Leiserson, Rivest, Stein', year: 2022, tags: ['algorithms', 'computer-science', 'textbook'], accessLevel: 'enrolled', downloads: 892, rating: 4.9, linkedCourses: ['CS301'], size: '48 MB' },
  { id: '2', title: 'Database System Concepts (7th Ed.)', description: 'Comprehensive coverage of database design, SQL, transaction processing, and distributed databases.', type: 'PDF', collection: 'Textbooks', author: 'Silberschatz, Korth, Sudarshan', year: 2020, tags: ['databases', 'sql', 'textbook'], accessLevel: 'enrolled', downloads: 754, rating: 4.7, linkedCourses: ['CS302'], size: '52 MB' },
  { id: '3', title: 'CS301 CAT 1 — 2024', description: 'Past paper for CS301 Continuous Assessment Test 1, Semester 1, 2024.', type: 'PDF', collection: 'Past Papers', author: 'Department of CS', year: 2024, tags: ['past-paper', 'cs301', 'cat'], accessLevel: 'enrolled', downloads: 1230, rating: 4.5, linkedCourses: ['CS301'], size: '2 MB' },
  { id: '4', title: 'Graph Theory — Visual Explanation', description: 'Video lecture series covering graph coloring, shortest paths, spanning trees, and network flows with visual animations.', type: 'Video', collection: 'Engineering Resources', author: 'Prof. Kariuki', year: 2024, tags: ['graphs', 'discrete-math', 'video'], accessLevel: 'enrolled', downloads: 445, rating: 4.8, linkedCourses: ['MAT301'] },
  { id: '5', title: 'TCP/IP Protocol Stack — RFC 793', description: 'Official RFC document for the Transmission Control Protocol. Essential reading for CS305.', type: 'Link', collection: 'Engineering Resources', author: 'IETF', year: 1981, tags: ['networking', 'tcp', 'protocol'], accessLevel: 'public', downloads: 312, rating: 4.2, linkedCourses: ['CS305'] },
  { id: '6', title: 'Nairobi Traffic Dataset 2024', description: 'Anonymised urban traffic flow dataset from Nairobi County. Suitable for ML coursework and research.', type: 'Dataset', collection: 'Research Articles', author: 'Kenya Urban Labs', year: 2024, tags: ['dataset', 'machine-learning', 'urban'], accessLevel: 'enrolled', downloads: 188, rating: 4.6, linkedCourses: ['CS401'], size: '840 MB' },
  { id: '7', title: 'Network Security Fundamentals', description: 'Lecture notes and worked examples on cryptography, firewalls, VPN, and intrusion detection systems.', type: 'PDF', collection: 'Engineering Resources', author: 'Dr. Omondi', year: 2024, tags: ['security', 'networking'], accessLevel: 'enrolled', downloads: 567, rating: 4.4, linkedCourses: ['CS305'], size: '12 MB' },
  { id: '8', title: 'Database Normalisation — Interactive Tutorial', description: 'Step-by-step interactive notebook walking through 1NF to BCNF with practical exercises.', type: 'EPUB', collection: 'Textbooks', author: 'TA Kamau', year: 2025, tags: ['databases', 'normalisation', 'tutorial'], accessLevel: 'enrolled', downloads: 334, rating: 4.6, linkedCourses: ['CS302'], size: '8 MB' },
];

const TYPE_META: Record<ResourceType, { icon: React.ReactNode; color: string; bg: string }> = {
  PDF: { icon: <FileText size={18} />, color: 'text-danger', bg: 'bg-danger-light' },
  Video: { icon: <Video size={18} />, color: 'text-brand-500', bg: 'bg-brand-50' },
  Link: { icon: <Link2 size={18} />, color: 'text-info', bg: 'bg-info-light' },
  EPUB: { icon: <BookOpen size={18} />, color: 'text-accent-400', bg: 'bg-accent-50' },
  Dataset: { icon: <Database size={18} />, color: 'text-gold-500', bg: 'bg-gold-50' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function LibraryPage() {
  const { user } = useAuth();
  const [collection, setCollection] = useState('All');
  const [search, setSearch] = useState('');
  const [view, setView] = useState<'grid' | 'list'>('grid');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);

  const filtered = RESOURCES.filter(r => {
    if (collection !== 'All' && r.collection !== collection) return false;
    if (search && !r.title.toLowerCase().includes(search.toLowerCase()) && !r.author.toLowerCase().includes(search.toLowerCase())) return false;
    if (selectedTag && !r.tags.includes(selectedTag)) return false;
    return true;
  });

  const allTags = Array.from(new Set(RESOURCES.flatMap(r => r.tags))).slice(0, 10);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between gap-3">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Library</h1>
            <p className="text-sm text-ink-tertiary mt-1">{RESOURCES.length} resources · {COLLECTIONS.length - 1} collections</p>
          </div>
          {(user?.role === 'admin' || user?.role === 'instructor') && (
            <Button size="sm"><Plus size={15} /> Upload Resource</Button>
          )}
        </div>
      </motion.div>

      {/* Search and controls */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.08 }} className="flex flex-col gap-3">
        <div className="flex gap-2">
          <div className="relative flex-1">
            <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input
              type="text"
              placeholder="Search by title, author, keyword..."
              value={search}
              onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
            />
          </div>
          <button className="p-2.5 rounded-xl border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
            <Filter size={16} />
          </button>
          <div className="flex bg-sand-100 rounded-xl p-0.5">
            <button onClick={() => setView('grid')} className={`p-2 rounded-lg cursor-pointer ${view === 'grid' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}><Grid3X3 size={15} /></button>
            <button onClick={() => setView('list')} className={`p-2 rounded-lg cursor-pointer ${view === 'list' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}><List size={15} /></button>
          </div>
        </div>

        {/* Collection tabs */}
        <div className="flex gap-2 overflow-x-auto pb-1">
          {COLLECTIONS.map(c => (
            <button
              key={c}
              onClick={() => setCollection(c)}
              className={`px-3 py-1.5 text-xs font-medium rounded-full whitespace-nowrap transition-all cursor-pointer ${
                collection === c ? 'bg-brand-500 text-white' : 'bg-surface-raised border border-sand-300 text-ink-secondary hover:border-brand-300'
              }`}
            >
              {c}
            </button>
          ))}
        </div>

        {/* Tags */}
        <div className="flex gap-1.5 flex-wrap">
          {allTags.map(tag => (
            <button
              key={tag}
              onClick={() => setSelectedTag(selectedTag === tag ? null : tag)}
              className={`flex items-center gap-1 px-2.5 py-1 text-[11px] rounded-full transition-all cursor-pointer ${
                selectedTag === tag ? 'bg-brand-100 text-brand-700 border border-brand-200' : 'bg-sand-100 text-ink-tertiary hover:bg-sand-200'
              }`}
            >
              <Tag size={9} /> {tag}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Resources */}
      {view === 'grid' ? (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {filtered.map((resource, i) => {
            const typeMeta = TYPE_META[resource.type];
            return (
              <motion.div key={resource.id} initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
                <Card hover padding="none">
                  <div className="p-5">
                    <div className="flex items-start justify-between mb-3">
                      <div className={`w-10 h-10 rounded-xl ${typeMeta.bg} flex items-center justify-center`}>
                        <span className={typeMeta.color}>{typeMeta.icon}</span>
                      </div>
                      <div className="flex items-center gap-1 text-xs text-gold-500">
                        <Star size={11} fill="currentColor" />
                        <span className="font-semibold">{resource.rating}</span>
                      </div>
                    </div>
                    <h4 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] line-clamp-2 leading-snug">{resource.title}</h4>
                    <p className="text-xs text-ink-tertiary mt-1.5 line-clamp-2">{resource.description}</p>
                    <div className="text-xs text-ink-placeholder mt-2">{resource.author}, {resource.year}</div>

                    <div className="flex flex-wrap gap-1 mt-3">
                      {resource.linkedCourses.map(c => (
                        <span key={c} className="text-[10px] px-2 py-0.5 rounded-full bg-brand-50 text-brand-600 border border-brand-100 font-medium">{c}</span>
                      ))}
                      <Badge variant={resource.accessLevel === 'public' ? 'success' : 'default'} className="text-[10px]">
                        {resource.accessLevel}
                      </Badge>
                    </div>
                  </div>
                  <div className="border-t border-sand-200 px-5 py-3 flex items-center justify-between">
                    <span className="text-[11px] text-ink-tertiary flex items-center gap-1">
                      <Download size={11} /> {resource.downloads.toLocaleString()}
                      {resource.size && <span className="ml-1">· {resource.size}</span>}
                    </span>
                    <button className={`flex items-center gap-1 text-xs font-medium ${resource.type === 'Link' ? 'text-info' : 'text-brand-500'} hover:underline cursor-pointer`}>
                      {resource.type === 'Link' ? <><ExternalLink size={12} /> Open</> : <><Download size={12} /> Download</>}
                    </button>
                  </div>
                </Card>
              </motion.div>
            );
          })}
        </motion.div>
      ) : (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
          <Card padding="none">
            <div className="divide-y divide-sand-100">
              {filtered.map((resource, i) => {
                const typeMeta = TYPE_META[resource.type];
                return (
                  <motion.div key={resource.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
                    className="flex items-center gap-4 px-5 py-4 hover:bg-sand-50 transition-colors cursor-pointer group">
                    <div className={`w-10 h-10 rounded-xl ${typeMeta.bg} flex items-center justify-center shrink-0`}>
                      <span className={typeMeta.color}>{typeMeta.icon}</span>
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] truncate">{resource.title}</span>
                        {resource.linkedCourses.map(c => (
                          <span key={c} className="text-[10px] px-1.5 py-0.5 rounded-full bg-brand-50 text-brand-600 border border-brand-100 font-medium hidden sm:inline">{c}</span>
                        ))}
                      </div>
                      <div className="flex items-center gap-3 text-xs text-ink-tertiary mt-0.5">
                        <span>{resource.author}</span>
                        <span>{resource.year}</span>
                        <span>{resource.collection}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-3 shrink-0 text-xs text-ink-tertiary">
                      <span className="flex items-center gap-1"><Star size={11} className="text-gold-500" fill="currentColor" />{resource.rating}</span>
                      <span className="hidden sm:flex items-center gap-1"><Download size={11} />{resource.downloads}</span>
                    </div>
                    <button className={`flex items-center gap-1 text-xs font-medium ${resource.type === 'Link' ? 'text-info' : 'text-brand-500'} opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer`}>
                      {resource.type === 'Link' ? <ExternalLink size={14} /> : <Download size={14} />}
                    </button>
                    <ChevronRight size={15} className="text-ink-placeholder opacity-0 group-hover:opacity-100 transition-opacity" />
                  </motion.div>
                );
              })}
            </div>
          </Card>
        </motion.div>
      )}

      {filtered.length === 0 && (
        <div className="text-center py-16 text-ink-tertiary">
          <Book size={36} className="mx-auto mb-3 text-ink-placeholder" />
          <p className="font-medium">No resources found</p>
          <p className="text-sm mt-1">Try a different search or collection</p>
        </div>
      )}
    </div>
  );
}
