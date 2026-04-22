import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Award, Download, Search, QrCode, Share2,
  CheckCircle2, ExternalLink, Plus,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

interface Certificate {
  id: string;
  title: string;
  description: string;
  issuer: string;
  issuedDate: string;
  recipientName: string;
  verifyCode: string;
  type: 'completion' | 'achievement' | 'participation' | 'academic';
  course?: string;
}

const CERTIFICATES: Certificate[] = [
  { id: '1', title: 'Certificate of Completion', description: 'Successfully completed Data Structures & Algorithms with a final grade of A (88%)', issuer: 'SmartLMS University', issuedDate: 'Dec 15, 2025', recipientName: 'Faith Wanjiku Kamau', verifyCode: 'CERT-2025-CS301-001', type: 'completion', course: 'CS301' },
  { id: '2', title: 'Certificate of Excellence', description: 'Awarded for achieving top 5% in Discrete Mathematics cohort performance', issuer: 'SmartLMS University', issuedDate: 'Dec 20, 2025', recipientName: 'Faith Wanjiku Kamau', verifyCode: 'CERT-2025-MAT301-002', type: 'achievement', course: 'MAT301' },
  { id: '3', title: 'Database Systems — Semester 1', description: 'Completed Database Systems coursework with distinction', issuer: 'SmartLMS University', issuedDate: 'Dec 18, 2025', recipientName: 'Faith Wanjiku Kamau', verifyCode: 'CERT-2025-CS302-003', type: 'completion', course: 'CS302' },
];

const TYPE_META = {
  completion: { color: 'from-brand-500 to-brand-700', accent: '#0D5E6D', label: 'Completion' },
  achievement: { color: 'from-gold-400 to-gold-600', accent: '#B88D2F', label: 'Excellence' },
  participation: { color: 'from-accent-400 to-accent-600', accent: '#C75C2B', label: 'Participation' },
  academic: { color: 'from-brand-700 to-brand-900', accent: '#073C47', label: 'Academic' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

function CertPreview({ cert }: { cert: Certificate }) {
  const meta = TYPE_META[cert.type];
  return (
    <div className={`relative rounded-2xl bg-gradient-to-br ${meta.color} p-[2px] shadow-lg`}>
      <div className="rounded-[14px] bg-white overflow-hidden">
        {/* Header band */}
        <div className={`bg-gradient-to-r ${meta.color} h-2`} />
        {/* Content */}
        <div className="p-8 text-center">
          <div className="flex justify-center mb-4">
            <div className="w-16 h-16 rounded-full flex items-center justify-center" style={{ background: `${meta.accent}15`, border: `2px solid ${meta.accent}30` }}>
              <Award size={32} style={{ color: meta.accent }} />
            </div>
          </div>
          <div className="text-[10px] uppercase tracking-[0.2em] font-semibold mb-2" style={{ color: meta.accent }}>SmartLMS University</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink mb-1">{cert.title}</div>
          <div className="text-xs text-ink-tertiary mb-4">This certifies that</div>
          <div className="text-xl font-bold font-[family-name:var(--font-display)] mb-2" style={{ color: meta.accent }}>{cert.recipientName}</div>
          <p className="text-xs text-ink-secondary leading-relaxed max-w-xs mx-auto">{cert.description}</p>
          <div className="mt-5 flex items-center justify-center gap-8">
            <div className="text-center">
              <div className="w-20 border-b border-sand-300 mb-1" />
              <div className="text-[10px] text-ink-tertiary">Dean of Faculty</div>
            </div>
            <div className="text-center">
              <div className="w-20 border-b border-sand-300 mb-1" />
              <div className="text-[10px] text-ink-tertiary">Registrar</div>
            </div>
          </div>
          <div className="mt-4 flex items-center justify-between text-[10px] text-ink-placeholder border-t border-sand-100 pt-3">
            <span>Issued: {cert.issuedDate}</span>
            <span className="font-mono">{cert.verifyCode}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export function CertificatesPage() {
  const { user } = useAuth();
  const [selected, setSelected] = useState<string>(CERTIFICATES[0].id);
  const [search, setSearch] = useState('');
  const isAdmin = user?.role === 'admin';

  const selectedCert = CERTIFICATES.find(c => c.id === selected);

  return (
    <div className="space-y-5">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Certificates</h1>
            <p className="text-sm text-ink-tertiary mt-1">{CERTIFICATES.length} certificates earned</p>
          </div>
          {isAdmin && <Button size="sm"><Plus size={14} /> Issue Certificate</Button>}
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-5 gap-5">
        {/* List */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.08 }} className="lg:col-span-2 space-y-3">
          {!isAdmin && (
            <div className="relative">
              <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
              <input type="text" placeholder="Search certificates..." value={search} onChange={e => setSearch(e.target.value)}
                className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
            </div>
          )}
          {CERTIFICATES.filter(c => !search || c.title.toLowerCase().includes(search.toLowerCase()) || c.course?.includes(search.toUpperCase())).map(cert => {
            const meta = TYPE_META[cert.type];
            return (
              <motion.div key={cert.id} whileTap={{ scale: 0.98 }} onClick={() => setSelected(cert.id)}>
                <Card hover padding="none" className={selected === cert.id ? 'border-brand-400 shadow-md' : ''}>
                  <div className="p-4 flex items-center gap-3">
                    <div className={`w-12 h-12 rounded-xl bg-gradient-to-br ${meta.color} flex items-center justify-center shrink-0`}>
                      <Award size={22} className="text-white" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{cert.title}</div>
                      <div className="flex items-center gap-2 mt-1">
                        {cert.course && <span className="text-[10px] font-semibold text-brand-500 bg-brand-50 px-1.5 py-0.5 rounded">{cert.course}</span>}
                        <Badge variant="default">{meta.label}</Badge>
                      </div>
                      <div className="text-xs text-ink-tertiary mt-0.5 flex items-center gap-1">
                        <CheckCircle2 size={10} className="text-success" /> {cert.issuedDate}
                      </div>
                    </div>
                  </div>
                </Card>
              </motion.div>
            );
          })}
        </motion.div>

        {/* Preview + actions */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.16 }} className="lg:col-span-3">
          {selectedCert && (
            <div className="space-y-4">
              <CertPreview cert={selectedCert} />
              <div className="flex gap-2">
                <Button className="flex-1"><Download size={14} /> Download PDF</Button>
                <Button variant="outline"><Share2 size={14} /> Share</Button>
                <Button variant="outline"><QrCode size={14} /> Verify</Button>
              </div>
              <Card className="p-3">
                <div className="flex items-center justify-between">
                  <div className="text-xs text-ink-tertiary">Verification URL</div>
                  <button className="flex items-center gap-1 text-xs text-brand-500 hover:underline cursor-pointer">
                    <ExternalLink size={11} /> Open
                  </button>
                </div>
                <div className="font-mono text-xs text-ink mt-1 break-all">verify.smartlms.io/{selectedCert.verifyCode}</div>
              </Card>
            </div>
          )}
        </motion.div>
      </div>
    </div>
  );
}
