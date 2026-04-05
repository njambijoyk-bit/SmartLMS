import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  CreditCard, Search, Download, QrCode, CheckCircle2,
  AlertTriangle, User, GraduationCap, Shield,
  Printer, RefreshCw, Eye, MoreHorizontal, Filter,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type CardStatus = 'active' | 'expired' | 'suspended' | 'alumni' | 'pending';
type CardTab = 'all' | 'active' | 'alumni' | 'suspended';

interface StudentCard {
  id: string;
  cardNumber: string;
  studentName: string;
  regNumber: string;
  programme: string;
  yearOfStudy: number;
  status: CardStatus;
  issuedDate: string;
  expiryDate: string;
  lastVerified?: string;
  photoUrl?: string;
}

const CARDS: StudentCard[] = [
  { id: '1', cardNumber: 'SID-2025-0042', studentName: 'James Mwangi', regNumber: 'CS-2022-0042', programme: 'BSc Computer Science', yearOfStudy: 3, status: 'active', issuedDate: 'Sep 2022', expiryDate: 'Jun 2026', lastVerified: '2 hours ago' },
  { id: '2', cardNumber: 'SID-2025-0078', studentName: 'Amina Hassan', regNumber: 'CS-2023-0078', programme: 'BSc Computer Science', yearOfStudy: 2, status: 'active', issuedDate: 'Sep 2023', expiryDate: 'Jun 2027', lastVerified: '1 day ago' },
  { id: '3', cardNumber: 'SID-2024-0156', studentName: 'Peter Odhiambo', regNumber: 'CS-2021-0156', programme: 'BSc Computer Science', yearOfStudy: 4, status: 'active', issuedDate: 'Sep 2021', expiryDate: 'Jun 2025', lastVerified: '5 hours ago' },
  { id: '4', cardNumber: 'AID-2024-0023', studentName: 'Sarah Wanjiku', regNumber: 'CS-2019-0023', programme: 'BSc Computer Science', yearOfStudy: 4, status: 'alumni', issuedDate: 'Dec 2023', expiryDate: 'Permanent', lastVerified: '2 weeks ago' },
  { id: '5', cardNumber: 'SID-2025-0091', studentName: 'Kevin Otieno', regNumber: 'CS-2022-0091', programme: 'BSc Software Engineering', yearOfStudy: 3, status: 'suspended', issuedDate: 'Sep 2022', expiryDate: 'Jun 2026' },
  { id: '6', cardNumber: 'SID-2025-0112', studentName: 'Grace Achieng', regNumber: 'IT-2024-0112', programme: 'BSc Information Technology', yearOfStudy: 1, status: 'pending', issuedDate: '—', expiryDate: '—' },
  { id: '7', cardNumber: 'AID-2024-0045', studentName: 'Daniel Kiprop', regNumber: 'CS-2018-0045', programme: 'BSc Computer Science', yearOfStudy: 4, status: 'alumni', issuedDate: 'Dec 2022', expiryDate: 'Permanent' },
  { id: '8', cardNumber: 'SID-2025-0203', studentName: 'Faith Njeri', regNumber: 'SE-2023-0203', programme: 'BSc Software Engineering', yearOfStudy: 2, status: 'active', issuedDate: 'Sep 2023', expiryDate: 'Jun 2027', lastVerified: '3 days ago' },
];

const STATUS_META: Record<CardStatus, { label: string; variant: 'success' | 'danger' | 'warning' | 'brand' | 'default' }> = {
  active: { label: 'Active', variant: 'success' },
  expired: { label: 'Expired', variant: 'danger' },
  suspended: { label: 'Suspended', variant: 'danger' },
  alumni: { label: 'Alumni', variant: 'brand' },
  pending: { label: 'Pending', variant: 'warning' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function IDCardsPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<CardTab>('all');
  const [search, setSearch] = useState('');
  const [showQR, setShowQR] = useState(false);

  const isAdmin = user?.role === 'admin';
  const isStudent = user?.role === 'learner';

  const filtered = CARDS.filter(c => {
    if (tab !== 'all' && c.status !== tab) return false;
    if (search && !c.studentName.toLowerCase().includes(search.toLowerCase()) && !c.regNumber.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const myCard = CARDS.find(c => c.studentName === user?.name) ?? CARDS[0];

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">ID Cards</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isAdmin ? 'Manage student and alumni identification cards' : 'Your digital student identification'}
            </p>
          </div>
          {isAdmin && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><Printer size={15} /> Batch Print</Button>
              <Button size="sm"><CreditCard size={15} /> Issue Card</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Student: My ID Card */}
      {isStudent && (
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }}>
          <Card padding="none" className="overflow-hidden">
            <div className="border-pattern" />
            <div className="p-6">
              <div className="flex gap-8">
                {/* Card visual */}
                <div className="w-[340px] shrink-0">
                  <div className="relative rounded-2xl overflow-hidden bg-gradient-to-br from-brand-700 via-brand-600 to-brand-800 p-6 text-white shadow-xl">
                    <div className="absolute top-0 right-0 w-40 h-40 bg-white/5 rounded-full -translate-y-1/2 translate-x-1/2" />
                    <div className="absolute bottom-0 left-0 w-32 h-32 bg-white/5 rounded-full translate-y-1/2 -translate-x-1/2" />

                    <div className="relative">
                      <div className="flex items-center gap-2 mb-5">
                        <div className="w-7 h-7 rounded-md bg-white/20 flex items-center justify-center">
                          <GraduationCap size={15} />
                        </div>
                        <div>
                          <div className="text-[10px] font-medium text-white/70 uppercase tracking-wider">SmartLMS University</div>
                          <div className="text-xs font-semibold leading-tight">Student Identification</div>
                        </div>
                      </div>

                      <div className="flex gap-4 items-end">
                        <div className="w-20 h-24 rounded-lg bg-white/15 flex items-center justify-center border border-white/20">
                          <User size={28} className="text-white/50" />
                        </div>
                        <div className="flex-1">
                          <div className="text-lg font-bold font-[family-name:var(--font-display)]">{myCard.studentName}</div>
                          <div className="text-xs text-white/70 mt-0.5">{myCard.regNumber}</div>
                          <div className="text-[11px] text-white/60 mt-1">{myCard.programme}</div>
                          <div className="text-[11px] text-white/50">Year {myCard.yearOfStudy}</div>
                        </div>
                      </div>

                      <div className="flex items-center justify-between mt-5 pt-3 border-t border-white/15">
                        <div>
                          <div className="text-[9px] text-white/40 uppercase">Card No</div>
                          <div className="text-xs font-mono font-medium">{myCard.cardNumber}</div>
                        </div>
                        <div>
                          <div className="text-[9px] text-white/40 uppercase">Valid Until</div>
                          <div className="text-xs font-medium">{myCard.expiryDate}</div>
                        </div>
                        <div className="w-10 h-10 bg-white/15 rounded-lg flex items-center justify-center">
                          <QrCode size={20} className="text-white/60" />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Card details */}
                <div className="flex-1 space-y-4">
                  <div className="flex items-center gap-3">
                    <Badge variant="success" size="md">Active</Badge>
                    <span className="text-xs text-ink-tertiary">Issued {myCard.issuedDate}</span>
                    {myCard.lastVerified && (
                      <span className="text-xs text-ink-placeholder">Last verified {myCard.lastVerified}</span>
                    )}
                  </div>

                  <div className="grid grid-cols-2 gap-3">
                    <div className="p-3 rounded-lg bg-surface-sunken">
                      <div className="text-xs text-ink-tertiary">Registration Number</div>
                      <div className="text-sm font-semibold font-mono text-ink mt-0.5">{myCard.regNumber}</div>
                    </div>
                    <div className="p-3 rounded-lg bg-surface-sunken">
                      <div className="text-xs text-ink-tertiary">Card Number</div>
                      <div className="text-sm font-semibold font-mono text-ink mt-0.5">{myCard.cardNumber}</div>
                    </div>
                    <div className="p-3 rounded-lg bg-surface-sunken">
                      <div className="text-xs text-ink-tertiary">Programme</div>
                      <div className="text-sm font-semibold text-ink mt-0.5">{myCard.programme}</div>
                    </div>
                    <div className="p-3 rounded-lg bg-surface-sunken">
                      <div className="text-xs text-ink-tertiary">Validity</div>
                      <div className="text-sm font-semibold text-ink mt-0.5">{myCard.issuedDate} — {myCard.expiryDate}</div>
                    </div>
                  </div>

                  <div className="flex gap-2 pt-2">
                    <Button size="sm" onClick={() => setShowQR(!showQR)}>
                      <QrCode size={14} /> {showQR ? 'Hide' : 'Show'} QR Code
                    </Button>
                    <Button variant="outline" size="sm"><Download size={14} /> Download PDF</Button>
                    <Button variant="ghost" size="sm"><RefreshCw size={14} /> Request Reprint</Button>
                  </div>

                  {showQR && (
                    <motion.div initial={{ opacity: 0, scale: 0.95 }} animate={{ opacity: 1, scale: 1 }} className="inline-flex flex-col items-center p-4 bg-white rounded-xl border border-sand-200 shadow-sm">
                      <div className="w-32 h-32 bg-ink rounded-lg flex items-center justify-center mb-2">
                        <QrCode size={64} className="text-white" />
                      </div>
                      <span className="text-[10px] text-ink-placeholder">Scan to verify identity</span>
                    </motion.div>
                  )}
                </div>
              </div>
            </div>
          </Card>
        </motion.div>
      )}

      {/* Admin: Stats */}
      {isAdmin && (
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 sm:grid-cols-4 gap-3">
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-success-light flex items-center justify-center shrink-0">
              <CheckCircle2 size={18} className="text-success" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{CARDS.filter(c => c.status === 'active').length}</div>
              <div className="text-xs text-ink-tertiary">Active</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-brand-50 flex items-center justify-center shrink-0">
              <GraduationCap size={18} className="text-brand-500" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{CARDS.filter(c => c.status === 'alumni').length}</div>
              <div className="text-xs text-ink-tertiary">Alumni</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-warning-light flex items-center justify-center shrink-0">
              <AlertTriangle size={18} className="text-warning" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{CARDS.filter(c => c.status === 'pending').length}</div>
              <div className="text-xs text-ink-tertiary">Pending</div>
            </div>
          </div>
          <div className="bg-surface-raised rounded-xl border border-sand-200 p-4 flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-danger-light flex items-center justify-center shrink-0">
              <Shield size={18} className="text-danger" />
            </div>
            <div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)]">{CARDS.filter(c => c.status === 'suspended').length}</div>
              <div className="text-xs text-ink-tertiary">Suspended</div>
            </div>
          </div>
        </motion.div>
      )}

      {/* Admin: Tabs + Search + Cards List */}
      {isAdmin && (
        <>
          <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }} className="flex items-center justify-between gap-4 flex-wrap">
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {(['all', 'active', 'alumni', 'suspended'] as CardTab[]).map(t => (
                <button key={t} onClick={() => setTab(t)}
                  className={`px-4 py-1.5 text-sm font-medium rounded-lg transition-all capitalize cursor-pointer ${tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                  {t === 'all' ? 'All' : STATUS_META[t as CardStatus]?.label || t}
                </button>
              ))}
            </div>
            <div className="flex gap-2 flex-1 justify-end">
              <div className="relative max-w-xs flex-1">
                <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
                <input type="text" placeholder="Search by name or reg number..." value={search} onChange={e => setSearch(e.target.value)}
                  className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400" />
              </div>
              <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
                <Filter size={16} />
              </button>
            </div>
          </motion.div>

          <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.14 }}>
            <Card padding="none">
              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-sand-200">
                      <th className="text-left py-3 px-4 text-xs font-semibold text-ink-tertiary">Student</th>
                      <th className="text-left py-3 px-4 text-xs font-semibold text-ink-tertiary">Card No</th>
                      <th className="text-left py-3 px-4 text-xs font-semibold text-ink-tertiary">Programme</th>
                      <th className="text-left py-3 px-4 text-xs font-semibold text-ink-tertiary">Status</th>
                      <th className="text-left py-3 px-4 text-xs font-semibold text-ink-tertiary">Validity</th>
                      <th className="text-right py-3 px-4 text-xs font-semibold text-ink-tertiary">Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {filtered.map((card, i) => {
                      const statusMeta = STATUS_META[card.status];
                      return (
                        <motion.tr key={card.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.03 }}
                          className="border-b border-sand-100 last:border-0 hover:bg-sand-50/50 transition-colors">
                          <td className="py-3 px-4">
                            <div className="flex items-center gap-3">
                              <div className="w-8 h-8 rounded-full bg-brand-50 flex items-center justify-center">
                                <User size={14} className="text-brand-500" />
                              </div>
                              <div>
                                <div className="font-medium text-ink">{card.studentName}</div>
                                <div className="text-xs text-ink-tertiary">{card.regNumber}</div>
                              </div>
                            </div>
                          </td>
                          <td className="py-3 px-4 font-mono text-xs text-ink-secondary">{card.cardNumber}</td>
                          <td className="py-3 px-4">
                            <div className="text-xs text-ink-secondary">{card.programme}</div>
                            <div className="text-[11px] text-ink-tertiary">Year {card.yearOfStudy}</div>
                          </td>
                          <td className="py-3 px-4"><Badge variant={statusMeta.variant}>{statusMeta.label}</Badge></td>
                          <td className="py-3 px-4 text-xs text-ink-tertiary">{card.issuedDate} — {card.expiryDate}</td>
                          <td className="py-3 px-4">
                            <div className="flex items-center justify-end gap-1">
                              <Button variant="ghost" size="sm"><Eye size={13} /></Button>
                              <Button variant="ghost" size="sm"><Download size={13} /></Button>
                              <button className="p-1.5 rounded-md hover:bg-sand-100 text-ink-tertiary cursor-pointer"><MoreHorizontal size={14} /></button>
                            </div>
                          </td>
                        </motion.tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </Card>
          </motion.div>
        </>
      )}
    </div>
  );
}
