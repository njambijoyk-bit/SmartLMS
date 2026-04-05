import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  UserCheck, Search, Filter, Check, X, Clock,
  Download, Plus, Users,
  FileText, AlertCircle,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

type RegStatus = 'pending' | 'approved' | 'rejected' | 'incomplete';
type RegMode = 'admin-approval' | 'self-registration' | 'invite-only' | 'payment-gated';

interface Application {
  id: string;
  name: string;
  email: string;
  phone: string;
  programme: string;
  year: string;
  mode: RegMode;
  status: RegStatus;
  submittedAt: string;
  nationalId?: string;
  documents?: number;
  note?: string;
}

const APPLICATIONS: Application[] = [
  { id: '1', name: 'Amira Hassan', email: 'amira.h@gmail.com', phone: '+254 701 234 567', programme: 'BSc Computer Science', year: 'Year 1', mode: 'admin-approval', status: 'pending', submittedAt: 'Apr 5, 2026 · 9:14 AM', nationalId: '37491023', documents: 3 },
  { id: '2', name: 'James Korir', email: 'j.korir@yahoo.com', phone: '+254 722 345 678', programme: 'BSc Information Technology', year: 'Year 1', mode: 'admin-approval', status: 'pending', submittedAt: 'Apr 5, 2026 · 8:30 AM', nationalId: '41209834', documents: 2, note: 'Missing form 4 certificate' },
  { id: '3', name: 'Fatuma Abubakar', email: 'fatuma.a@outlook.com', phone: '+254 733 456 789', programme: 'BSc Software Engineering', year: 'Year 1', mode: 'payment-gated', status: 'pending', submittedAt: 'Apr 4, 2026 · 3:45 PM', nationalId: '38827461', documents: 4 },
  { id: '4', name: 'Peter Njuguna', email: 'peter.n@gmail.com', phone: '+254 712 567 890', programme: 'BSc Computer Science', year: 'Transfer', mode: 'admin-approval', status: 'approved', submittedAt: 'Apr 3, 2026 · 11:20 AM', nationalId: '29384756', documents: 5 },
  { id: '5', name: 'Diana Chebet', email: 'd.chebet@gmail.com', phone: '+254 745 678 901', programme: 'BSc Information Technology', year: 'Year 1', mode: 'self-registration', status: 'approved', submittedAt: 'Apr 2, 2026 · 10:00 AM', documents: 3 },
  { id: '6', name: 'Omar Abdallah', email: 'omar.a@hotmail.com', phone: '+254 756 789 012', programme: 'BSc Software Engineering', year: 'Year 1', mode: 'admin-approval', status: 'rejected', submittedAt: 'Apr 1, 2026 · 4:30 PM', nationalId: '42398147', documents: 1, note: 'Incomplete documents. Requested to resubmit.' },
  { id: '7', name: 'Lucy Wambua', email: 'lucy.w@gmail.com', phone: '+254 767 890 123', programme: 'BSc Computer Science', year: 'Year 2', mode: 'invite-only', status: 'incomplete', submittedAt: 'Mar 30, 2026 · 2:15 PM', documents: 0 },
];

const STATUS_META: Record<RegStatus, { variant: 'success' | 'danger' | 'warning' | 'default'; label: string; icon: React.ReactNode }> = {
  pending: { variant: 'warning', label: 'Pending Review', icon: <Clock size={12} /> },
  approved: { variant: 'success', label: 'Approved', icon: <Check size={12} /> },
  rejected: { variant: 'danger', label: 'Rejected', icon: <X size={12} /> },
  incomplete: { variant: 'default', label: 'Incomplete', icon: <AlertCircle size={12} /> },
};

const MODE_META: Record<RegMode, { label: string; color: string }> = {
  'admin-approval': { label: 'Admin Approval', color: 'text-brand-500 bg-brand-50 border-brand-100' },
  'self-registration': { label: 'Self Registration', color: 'text-success bg-success-light border-success/20' },
  'invite-only': { label: 'Invite Only', color: 'text-gold-600 bg-gold-50 border-gold-200' },
  'payment-gated': { label: 'Payment Gated', color: 'text-accent-500 bg-accent-50 border-accent-100' },
};


const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function RegistrationPage() {
  const [filter, setFilter] = useState<RegStatus | 'all'>('all');
  const [search, setSearch] = useState('');
  const [selected, setSelected] = useState<string | null>(null);

  const filtered = APPLICATIONS.filter(a => {
    if (filter !== 'all' && a.status !== filter) return false;
    if (search && !a.name.toLowerCase().includes(search.toLowerCase()) && !a.email.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const counts = {
    all: APPLICATIONS.length,
    pending: APPLICATIONS.filter(a => a.status === 'pending').length,
    approved: APPLICATIONS.filter(a => a.status === 'approved').length,
    rejected: APPLICATIONS.filter(a => a.status === 'rejected').length,
    incomplete: APPLICATIONS.filter(a => a.status === 'incomplete').length,
  };

  const selectedApp = APPLICATIONS.find(a => a.id === selected);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Student Registration</h1>
            <p className="text-sm text-ink-tertiary mt-1">{counts.pending} applications pending review</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm"><Download size={14} /> Export</Button>
            <Button size="sm"><Plus size={14} /> Invite Student</Button>
          </div>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-4 gap-3">
        {[
          { key: 'pending', label: 'Pending', count: counts.pending, color: 'text-warning', bg: 'bg-warning-light', border: 'border-warning/20' },
          { key: 'approved', label: 'Approved', count: counts.approved, color: 'text-success', bg: 'bg-success-light', border: 'border-success/20' },
          { key: 'rejected', label: 'Rejected', count: counts.rejected, color: 'text-danger', bg: 'bg-danger-light', border: 'border-danger/20' },
          { key: 'incomplete', label: 'Incomplete', count: counts.incomplete, color: 'text-ink-tertiary', bg: 'bg-sand-100', border: 'border-sand-300' },
        ].map(s => (
          <button key={s.key} onClick={() => setFilter(s.key as RegStatus | 'all')}
            className={`${s.bg} ${s.border} border rounded-xl p-4 text-center cursor-pointer transition-all hover:opacity-80 ${filter === s.key ? 'ring-2 ring-brand-400' : ''}`}>
            <div className={`text-2xl font-bold font-[family-name:var(--font-display)] ${s.color}`}>{s.count}</div>
            <div className="text-xs text-ink-tertiary mt-0.5">{s.label}</div>
          </button>
        ))}
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-5 gap-4">
        {/* Application list */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="lg:col-span-3">
          <div className="flex gap-2 mb-3">
            <div className="relative flex-1">
              <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
              <input type="text" placeholder="Search applicant..." value={search} onChange={e => setSearch(e.target.value)}
                className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
            </div>
            <button className="p-2 rounded-xl border border-sand-300 text-ink-tertiary hover:text-ink cursor-pointer">
              <Filter size={15} />
            </button>
          </div>

          <Card padding="none">
            <div className="divide-y divide-sand-100">
              {filtered.map((app, i) => (
                <motion.div key={app.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
                  onClick={() => setSelected(selected === app.id ? null : app.id)}
                  className={`flex items-start gap-4 px-5 py-4 cursor-pointer transition-colors ${selected === app.id ? 'bg-brand-50' : 'hover:bg-sand-50'}`}>
                  <div className="w-9 h-9 rounded-full bg-brand-500 flex items-center justify-center text-white text-sm font-bold font-[family-name:var(--font-display)] shrink-0">
                    {app.name.charAt(0)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{app.name}</span>
                      <Badge variant={STATUS_META[app.status].variant}>{STATUS_META[app.status].label}</Badge>
                    </div>
                    <div className="text-xs text-ink-tertiary">{app.email}</div>
                    <div className="flex items-center gap-2 mt-1.5">
                      <span className="text-[10px] text-ink-secondary">{app.programme}</span>
                      <span className={`text-[10px] px-1.5 py-0.5 rounded-full border ${MODE_META[app.mode].color}`}>{MODE_META[app.mode].label}</span>
                    </div>
                    <div className="text-[10px] text-ink-placeholder mt-0.5">{app.submittedAt}</div>
                    {app.note && (
                      <div className="mt-1.5 flex items-center gap-1 text-[11px] text-danger">
                        <AlertCircle size={10} /> {app.note}
                      </div>
                    )}
                  </div>
                </motion.div>
              ))}
              {filtered.length === 0 && (
                <div className="text-center py-12 text-ink-tertiary">
                  <Users size={28} className="mx-auto mb-2 text-ink-placeholder" />
                  <p className="text-sm">No applications found</p>
                </div>
              )}
            </div>
          </Card>
        </motion.div>

        {/* Detail panel */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.16 }} className="lg:col-span-2">
          {selectedApp ? (
            <div className="space-y-4 sticky top-4">
              <Card>
                <div className="flex items-center gap-3 mb-4">
                  <div className="w-12 h-12 rounded-full bg-brand-500 flex items-center justify-center text-white text-lg font-bold font-[family-name:var(--font-display)]">
                    {selectedApp.name.charAt(0)}
                  </div>
                  <div>
                    <h3 className="font-bold font-[family-name:var(--font-display)] text-ink">{selectedApp.name}</h3>
                    <Badge variant={STATUS_META[selectedApp.status].variant}>{STATUS_META[selectedApp.status].label}</Badge>
                  </div>
                </div>

                <div className="space-y-2 text-sm">
                  {[
                    { label: 'Email', value: selectedApp.email },
                    { label: 'Phone', value: selectedApp.phone },
                    { label: 'Programme', value: selectedApp.programme },
                    { label: 'Year', value: selectedApp.year },
                    { label: 'Mode', value: MODE_META[selectedApp.mode].label },
                    { label: 'Documents', value: `${selectedApp.documents ?? 0} uploaded` },
                    ...(selectedApp.nationalId ? [{ label: 'National ID', value: selectedApp.nationalId }] : []),
                  ].map(({ label, value }) => (
                    <div key={label} className="flex justify-between items-center border-b border-sand-100 pb-2 last:border-0">
                      <span className="text-ink-tertiary text-xs">{label}</span>
                      <span className="text-ink font-medium text-xs">{value}</span>
                    </div>
                  ))}
                </div>

                {selectedApp.note && (
                  <div className="mt-3 p-2.5 bg-danger-light rounded-lg text-xs text-danger flex gap-1.5">
                    <AlertCircle size={13} className="shrink-0 mt-0.5" />
                    <span>{selectedApp.note}</span>
                  </div>
                )}
              </Card>

              {/* Approval actions */}
              {selectedApp.status === 'pending' && (
                <div className="flex gap-2">
                  <button className="flex-1 py-2.5 rounded-xl bg-success text-white text-sm font-semibold flex items-center justify-center gap-1.5 hover:bg-success/90 transition-colors cursor-pointer">
                    <Check size={15} /> Approve
                  </button>
                  <button className="flex-1 py-2.5 rounded-xl bg-danger text-white text-sm font-semibold flex items-center justify-center gap-1.5 hover:bg-danger/90 transition-colors cursor-pointer">
                    <X size={15} /> Reject
                  </button>
                </div>
              )}

              <Button variant="outline" size="sm" className="w-full"><FileText size={14} /> View Documents</Button>
            </div>
          ) : (
            <Card className="text-center py-12">
              <UserCheck size={32} className="mx-auto text-ink-placeholder mb-3" />
              <p className="text-sm text-ink-tertiary">Select an application to review</p>
            </Card>
          )}
        </motion.div>
      </div>
    </div>
  );
}
