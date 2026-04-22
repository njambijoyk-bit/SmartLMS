import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  CreditCard, Download,
  Search, CheckCircle2,
  ChevronRight, Receipt, Plus,
} from 'lucide-react';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type PaymentStatus = 'paid' | 'partial' | 'pending' | 'overdue';

interface FeeRecord {
  id: string;
  studentName: string;
  regNo: string;
  programme: string;
  year: string;
  totalFee: number;
  paid: number;
  balance: number;
  status: PaymentStatus;
  lastPayment?: string;
  dueDate: string;
}

const FEES: FeeRecord[] = [
  { id: '1', studentName: 'Faith Kamau', regNo: 'CS/2022/001', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 85000, balance: 0, status: 'paid', lastPayment: 'Mar 15, 2026', dueDate: 'Apr 15, 2026' },
  { id: '2', studentName: 'Brian Otieno', regNo: 'CS/2022/002', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 60000, balance: 25000, status: 'partial', lastPayment: 'Feb 28, 2026', dueDate: 'Apr 15, 2026' },
  { id: '3', studentName: 'Mary Wanjiku', regNo: 'CS/2022/003', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 0, balance: 85000, status: 'overdue', dueDate: 'Mar 15, 2026' },
  { id: '4', studentName: 'Daniel Mutua', regNo: 'CS/2022/004', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 85000, balance: 0, status: 'paid', lastPayment: 'Jan 20, 2026', dueDate: 'Apr 15, 2026' },
  { id: '5', studentName: 'Rose Adhiambo', regNo: 'CS/2022/005', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 42500, balance: 42500, status: 'partial', lastPayment: 'Mar 1, 2026', dueDate: 'Apr 15, 2026' },
  { id: '6', studentName: 'Kevin Kamau', regNo: 'CS/2022/006', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 0, balance: 85000, status: 'pending', dueDate: 'Apr 15, 2026' },
  { id: '7', studentName: 'Grace Njeri', regNo: 'CS/2022/007', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 85000, balance: 0, status: 'paid', lastPayment: 'Mar 22, 2026', dueDate: 'Apr 15, 2026' },
  { id: '8', studentName: 'Samuel Ochieng', regNo: 'CS/2022/008', programme: 'BSc Computer Science', year: 'Year 3', totalFee: 85000, paid: 70000, balance: 15000, status: 'partial', lastPayment: 'Mar 10, 2026', dueDate: 'Apr 15, 2026' },
];

const revenueData = [
  { month: 'Jan', collected: 1240000, expected: 1700000 },
  { month: 'Feb', collected: 980000, expected: 1700000 },
  { month: 'Mar', collected: 1560000, expected: 1700000 },
  { month: 'Apr', collected: 420000, expected: 1700000 },
];

const STATUS_META: Record<PaymentStatus, { variant: 'success' | 'warning' | 'danger' | 'default'; label: string }> = {
  paid: { variant: 'success', label: 'Paid' },
  partial: { variant: 'warning', label: 'Partial' },
  pending: { variant: 'default', label: 'Pending' },
  overdue: { variant: 'danger', label: 'Overdue' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

function ksh(amount: number) {
  return `KSh ${amount.toLocaleString()}`;
}

export function FeesPage() {
  const { user } = useAuth();
  const [filter, setFilter] = useState<PaymentStatus | 'all'>('all');
  const [search, setSearch] = useState('');

  const isAdmin = user?.role === 'admin';

  const filtered = FEES.filter(f => {
    if (filter !== 'all' && f.status !== filter) return false;
    if (search && !f.studentName.toLowerCase().includes(search.toLowerCase()) && !f.regNo.includes(search)) return false;
    return true;
  });

  const totalExpected = FEES.reduce((s, f) => s + f.totalFee, 0);
  const totalCollected = FEES.reduce((s, f) => s + f.paid, 0);
  const totalOutstanding = FEES.reduce((s, f) => s + f.balance, 0);
  const overdueCount = FEES.filter(f => f.status === 'overdue').length;

  // Student view: show own fee statement
  if (!isAdmin && user?.role !== 'instructor') {
    return (
      <div className="space-y-5 max-w-2xl">
        <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
          <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Fee Statement</h1>
          <p className="text-sm text-ink-tertiary mt-1">Semester 1, 2025/26</p>
        </motion.div>

        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
          <div className="rounded-xl border-2 border-brand-200 bg-brand-50 p-6">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-brand-600 font-medium">Outstanding Balance</div>
                <div className="text-4xl font-bold font-[family-name:var(--font-display)] text-brand-700 mt-1">KSh 25,000</div>
              </div>
              <div className="w-16 h-16 rounded-2xl bg-brand-500 flex items-center justify-center">
                <CreditCard size={28} className="text-white" />
              </div>
            </div>
            <div className="mt-4">
              <div className="flex justify-between text-xs text-brand-600 mb-1">
                <span>KSh 60,000 paid of KSh 85,000</span>
                <span>71%</span>
              </div>
              <div className="h-2.5 bg-brand-200 rounded-full overflow-hidden">
                <div className="h-full bg-brand-500 rounded-full" style={{ width: '71%' }} />
              </div>
            </div>
            <div className="mt-4 flex gap-2">
              <Button className="flex-1"><CreditCard size={14} /> Pay Now</Button>
              <Button variant="outline"><Receipt size={14} /> Statement</Button>
            </div>
          </div>
        </motion.div>

        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Payment History</h3>
            <div className="space-y-3">
              {[
                { ref: 'TXN-2026-8821', amount: 30000, date: 'Mar 15, 2026', method: 'M-Pesa', status: 'confirmed' },
                { ref: 'TXN-2026-4412', amount: 30000, date: 'Feb 1, 2026', method: 'Bank Transfer', status: 'confirmed' },
              ].map((tx, i) => (
                <div key={i} className="flex items-center gap-3 p-3 rounded-lg bg-sand-50 border border-sand-200">
                  <div className="w-9 h-9 rounded-xl bg-success-light flex items-center justify-center">
                    <CheckCircle2 size={18} className="text-success" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-semibold text-ink">{ksh(tx.amount)}</span>
                      <Badge variant="success">Confirmed</Badge>
                    </div>
                    <div className="text-xs text-ink-tertiary">{tx.ref} · {tx.method} · {tx.date}</div>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>
    );
  }

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Fee Management</h1>
            <p className="text-sm text-ink-tertiary mt-1">Semester 1, 2025/26 · BSc Computer Science</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm"><Download size={14} /> Export</Button>
            <Button size="sm"><Plus size={14} /> Record Payment</Button>
          </div>
        </div>
      </motion.div>

      {/* KPI strip */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Total Expected</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink mt-1">{ksh(totalExpected)}</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Collected</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success mt-1">{ksh(totalCollected)}</div>
          <div className="text-xs text-success/60 mt-1">{Math.round((totalCollected / totalExpected) * 100)}% collected</div>
        </div>
        <div className="bg-warning-light rounded-xl border border-warning/20 p-4">
          <div className="text-xs text-warning/70">Outstanding</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-warning mt-1">{ksh(totalOutstanding)}</div>
        </div>
        <div className="bg-danger-light rounded-xl border border-danger/20 p-4">
          <div className="text-xs text-danger/70">Overdue accounts</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-danger mt-1">{overdueCount}</div>
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Revenue chart */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="lg:col-span-1">
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-1">Monthly Collection</h3>
            <p className="text-xs text-ink-tertiary mb-4">Expected vs collected (KSh)</p>
            <ResponsiveContainer width="100%" height={160}>
              <BarChart data={revenueData} barSize={12}>
                <XAxis dataKey="month" axisLine={false} tickLine={false} tick={{ fontSize: 11, fill: '#7A7E87' }} />
                <YAxis axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} tickFormatter={v => `${v / 1000000}M`} />
                <Tooltip contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 11 }} formatter={(v: unknown) => typeof v === 'number' ? `KSh ${v.toLocaleString()}` : ''} />
                <Bar dataKey="expected" fill="#EDE6DB" radius={[4, 4, 0, 0]} name="Expected" />
                <Bar dataKey="collected" fill="#0D5E6D" radius={[4, 4, 0, 0]} name="Collected" />
              </BarChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Student fee table */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.16 }} className="lg:col-span-2">
          <div className="flex gap-2 mb-3">
            <div className="relative flex-1">
              <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
              <input type="text" placeholder="Search student or reg. number..." value={search} onChange={e => setSearch(e.target.value)}
                className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
            </div>
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {(['all', 'paid', 'partial', 'overdue', 'pending'] as const).map(s => (
                <button key={s} onClick={() => setFilter(s)}
                  className={`px-2.5 py-1.5 text-[11px] font-medium rounded-lg transition-all capitalize cursor-pointer ${filter === s ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                  {s}
                </button>
              ))}
            </div>
          </div>

          <Card padding="none">
            <div className="divide-y divide-sand-100">
              {filtered.map((fee, i) => (
                <motion.div key={fee.id} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.04 }}
                  className="flex items-center gap-3 px-4 py-3.5 hover:bg-sand-50 transition-colors">
                  <div className="w-8 h-8 rounded-full bg-brand-500 flex items-center justify-center text-white text-xs font-bold font-[family-name:var(--font-display)] shrink-0">
                    {fee.studentName.charAt(0)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-semibold text-ink">{fee.studentName}</span>
                      <span className="text-xs text-ink-tertiary">{fee.regNo}</span>
                    </div>
                    <div className="flex items-center gap-3 mt-1.5">
                      <div className="flex-1 h-1.5 bg-sand-200 rounded-full overflow-hidden max-w-28">
                        <div className={`h-full rounded-full ${fee.status === 'paid' ? 'bg-success' : fee.status === 'overdue' ? 'bg-danger' : 'bg-warning'}`}
                          style={{ width: `${(fee.paid / fee.totalFee) * 100}%` }} />
                      </div>
                      <span className="text-xs text-ink-tertiary">{ksh(fee.paid)}/{ksh(fee.totalFee)}</span>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <div className={`text-sm font-bold font-[family-name:var(--font-display)] ${fee.balance > 0 ? 'text-danger' : 'text-success'}`}>
                      {fee.balance > 0 ? `-${ksh(fee.balance)}` : '✓ Paid'}
                    </div>
                    <Badge variant={STATUS_META[fee.status].variant}>{STATUS_META[fee.status].label}</Badge>
                  </div>
                  <ChevronRight size={14} className="text-ink-placeholder shrink-0" />
                </motion.div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
