import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Zap, Plus, Play, Pause, MoreHorizontal,
  Clock, GitBranch, Award, Mail, Bell, UserCheck,
  CheckCircle2, AlertTriangle, ChevronRight, Search,
  Settings, Trash2,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

interface AutomationRule {
  id: string;
  name: string;
  description: string;
  trigger: string;
  triggerIcon: React.ReactNode;
  actions: string[];
  enabled: boolean;
  lastRun?: string;
  runCount: number;
  category: 'academic' | 'engagement' | 'admin' | 'gamification';
}

const RULES: AutomationRule[] = [
  {
    id: '1', name: 'Low Attendance Alert', description: 'Notify instructor and parent when attendance drops below 60%',
    trigger: 'Attendance < 60%', triggerIcon: <AlertTriangle size={14} />,
    actions: ['Send email to instructor', 'Send push to parent portal', 'Flag student as at-risk'],
    enabled: true, lastRun: '2h ago', runCount: 23, category: 'engagement',
  },
  {
    id: '2', name: 'Certificate Issuer', description: 'Auto-generate completion certificate when all modules are done',
    trigger: 'Course 100% complete', triggerIcon: <Award size={14} />,
    actions: ['Generate certificate PDF', 'Send email with certificate', 'Award 50 XP'],
    enabled: true, lastRun: '1d ago', runCount: 156, category: 'academic',
  },
  {
    id: '3', name: 'Deadline Reminder', description: 'Send reminder 24h and 1h before assignment deadline',
    trigger: '24h before deadline', triggerIcon: <Clock size={14} />,
    actions: ['Push notification to student', 'Send SMS if mobile number set'],
    enabled: true, lastRun: '4h ago', runCount: 892, category: 'academic',
  },
  {
    id: '4', name: 'Welcome Sequence', description: 'Send onboarding emails to newly registered students',
    trigger: 'New student registered', triggerIcon: <UserCheck size={14} />,
    actions: ['Send welcome email', 'Assign to orientation course', 'Create student profile'],
    enabled: true, lastRun: '3d ago', runCount: 45, category: 'admin',
  },
  {
    id: '5', name: 'Achievement Badges', description: 'Award XP and badges for learning milestones',
    trigger: 'Milestone reached', triggerIcon: <Award size={14} />,
    actions: ['Award badge', 'Add XP to leaderboard', 'Show confetti animation'],
    enabled: false, lastRun: '1w ago', runCount: 340, category: 'gamification',
  },
  {
    id: '6', name: 'Overdue Fee Escalation', description: 'Escalate overdue accounts through notification tiers',
    trigger: 'Fee overdue > 30 days', triggerIcon: <AlertTriangle size={14} />,
    actions: ['Send reminder email', 'Notify finance office', 'Block exam card issuance'],
    enabled: true, lastRun: '6h ago', runCount: 12, category: 'admin',
  },
];

const TRIGGERS = [
  { label: 'Student enrolls', icon: <UserCheck size={14} /> },
  { label: 'Assignment submitted', icon: <CheckCircle2 size={14} /> },
  { label: 'Grade posted', icon: <GitBranch size={14} /> },
  { label: 'Attendance marked', icon: <Clock size={14} /> },
  { label: 'Course completed', icon: <Award size={14} /> },
  { label: 'Fee payment received', icon: <CheckCircle2 size={14} /> },
];

const CATEGORY_META = {
  academic: { label: 'Academic', variant: 'brand' as const },
  engagement: { label: 'Engagement', variant: 'warning' as const },
  admin: { label: 'Admin', variant: 'default' as const },
  gamification: { label: 'Gamification', variant: 'accent' as const },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function AutomationPage() {
  const [rules, setRules] = useState(RULES);
  const [filter, setFilter] = useState<string>('all');
  const [search, setSearch] = useState('');

  const toggleRule = (id: string) => {
    setRules(prev => prev.map(r => r.id === id ? { ...r, enabled: !r.enabled } : r));
  };

  const filtered = rules.filter(r => {
    if (filter !== 'all' && r.category !== filter) return false;
    if (search && !r.name.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  const activeCount = rules.filter(r => r.enabled).length;
  const totalRuns = rules.reduce((s, r) => s + r.runCount, 0);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Automation Engine</h1>
            <p className="text-sm text-ink-tertiary mt-1">Create IF/THEN rules to automate workflows across your LMS</p>
          </div>
          <Button size="sm"><Plus size={14} /> New Rule</Button>
        </div>
      </motion.div>

      {/* KPIs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Zap size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Total Rules</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{rules.length}</div>
        </div>
        <div className="bg-success-light rounded-xl border border-success/20 p-4">
          <div className="text-xs text-success/70">Active</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{activeCount}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Total Runs</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{totalRuns.toLocaleString()}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="text-xs text-ink-tertiary">Available Triggers</div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{TRIGGERS.length}+</div>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }}>
        <div className="flex gap-2 items-center">
          <div className="relative flex-1 max-w-sm">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search rules..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
          </div>
          <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
            {['all', 'academic', 'engagement', 'admin', 'gamification'].map(c => (
              <button key={c} onClick={() => setFilter(c)}
                className={`px-2.5 py-1.5 text-[11px] font-medium rounded-lg transition-all capitalize cursor-pointer ${filter === c ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                {c}
              </button>
            ))}
          </div>
        </div>
      </motion.div>

      {/* Rules list */}
      <div className="space-y-3">
        {filtered.map((rule, i) => (
          <motion.div key={rule.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 + i * 0.05 }}>
            <Card padding="none">
              <div className="flex items-center gap-4 p-4">
                {/* Toggle */}
                <button
                  onClick={() => toggleRule(rule.id)}
                  className={`w-11 h-6 rounded-full relative transition-colors cursor-pointer shrink-0 ${rule.enabled ? 'bg-success' : 'bg-sand-300'}`}
                >
                  <div className={`absolute top-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform ${rule.enabled ? 'left-[22px]' : 'left-0.5'}`} />
                </button>

                {/* Info */}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className={`text-sm font-semibold ${rule.enabled ? 'text-ink' : 'text-ink-tertiary'}`}>{rule.name}</span>
                    <Badge variant={CATEGORY_META[rule.category].variant} size="sm">{CATEGORY_META[rule.category].label}</Badge>
                  </div>
                  <p className="text-xs text-ink-tertiary mt-0.5">{rule.description}</p>

                  {/* Trigger → Actions flow */}
                  <div className="flex items-center gap-2 mt-2.5 flex-wrap">
                    <span className="inline-flex items-center gap-1.5 px-2 py-1 rounded-lg bg-brand-50 text-brand-600 text-[10px] font-semibold">
                      {rule.triggerIcon} IF {rule.trigger}
                    </span>
                    <ChevronRight size={10} className="text-ink-placeholder" />
                    {rule.actions.map((action, ai) => (
                      <span key={ai} className="inline-flex items-center gap-1 px-2 py-1 rounded-lg bg-sand-100 text-ink-secondary text-[10px] font-medium">
                        {ai === 0 ? <Mail size={9} /> : ai === 1 ? <Bell size={9} /> : <Zap size={9} />}
                        {action}
                      </span>
                    ))}
                  </div>
                </div>

                {/* Meta */}
                <div className="text-right shrink-0 space-y-1">
                  <div className="text-xs text-ink-tertiary">{rule.runCount} runs</div>
                  {rule.lastRun && <div className="text-[10px] text-ink-placeholder">Last: {rule.lastRun}</div>}
                </div>

                {/* Actions */}
                <div className="flex items-center gap-1 shrink-0">
                  <button className="p-1.5 rounded-lg hover:bg-sand-100 text-ink-tertiary transition-colors cursor-pointer">
                    <Settings size={14} />
                  </button>
                  <button className="p-1.5 rounded-lg hover:bg-sand-100 text-ink-tertiary transition-colors cursor-pointer">
                    {rule.enabled ? <Pause size={14} /> : <Play size={14} />}
                  </button>
                  <button className="p-1.5 rounded-lg hover:bg-danger-light text-ink-tertiary hover:text-danger transition-colors cursor-pointer">
                    <Trash2 size={14} />
                  </button>
                </div>
              </div>
            </Card>
          </motion.div>
        ))}
      </div>

      {/* Available triggers card */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.3 }}>
        <Card>
          <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Available Triggers</h3>
          <p className="text-xs text-ink-tertiary mb-4">Use these events as triggers in your automation rules</p>
          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
            {TRIGGERS.map((trigger, i) => (
              <div key={i} className="flex items-center gap-2 p-3 rounded-lg border border-sand-200 bg-surface-raised hover:border-brand-200 transition-colors cursor-pointer">
                <span className="text-brand-500">{trigger.icon}</span>
                <span className="text-xs font-medium text-ink">{trigger.label}</span>
              </div>
            ))}
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
