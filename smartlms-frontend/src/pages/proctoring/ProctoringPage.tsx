import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  ShieldCheck, Eye, Camera, Monitor, Lock, Users,
  Settings, CheckCircle2, AlertTriangle, BarChart2,
  ChevronRight, Shield,
  Video, Clock,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type ProctoringTier = 1 | 2 | 3 | 4;
type Tab = 'sessions' | 'settings' | 'reports';
type SessionStatus = 'live' | 'scheduled' | 'completed' | 'flagged';

interface ProctoringSession {
  id: string;
  examTitle: string;
  courseCode: string;
  tier: ProctoringTier;
  status: SessionStatus;
  date: string;
  duration: number;
  students: number;
  flaggedCount: number;
  tabSwitches: number;
  avgFocusRate: number;
}

const SESSIONS: ProctoringSession[] = [
  { id: '1', examTitle: 'CAT 2 — Binary Trees & Hash Tables', courseCode: 'CS301', tier: 2, status: 'live', date: 'Now', duration: 45, students: 67, flaggedCount: 3, tabSwitches: 12, avgFocusRate: 94 },
  { id: '2', examTitle: 'End of Semester — Database Systems', courseCode: 'CS302', tier: 3, status: 'scheduled', date: 'Apr 10, 9:00 AM', duration: 180, students: 198, flaggedCount: 0, tabSwitches: 0, avgFocusRate: 0 },
  { id: '3', examTitle: 'CAT 1 — Network Layers', courseCode: 'CS305', tier: 2, status: 'completed', date: 'Mar 22, 2:00 PM', duration: 45, students: 89, flaggedCount: 5, tabSwitches: 34, avgFocusRate: 91 },
  { id: '4', examTitle: 'Mid-Semester — Discrete Maths', courseCode: 'MAT301', tier: 4, status: 'completed', date: 'Mar 15, 9:00 AM', duration: 120, students: 132, flaggedCount: 8, tabSwitches: 47, avgFocusRate: 88 },
  { id: '5', examTitle: 'CAT 1 — Logic & Set Theory', courseCode: 'MAT301', tier: 1, status: 'completed', date: 'Mar 1, 2:00 PM', duration: 45, students: 132, flaggedCount: 0, tabSwitches: 0, avgFocusRate: 0 },
  { id: '6', examTitle: 'End of Semester — Data Structures', courseCode: 'CS301', tier: 3, status: 'scheduled', date: 'May 2, 9:00 AM', duration: 180, students: 145, flaggedCount: 0, tabSwitches: 0, avgFocusRate: 0 },
];

const TIER_META: Record<ProctoringTier, { label: string; description: string; color: string; bg: string; icon: React.ReactNode; features: string[] }> = {
  1: { label: 'Honour-based', description: 'Student confirms they work alone. No technical enforcement.', color: 'text-ink-tertiary', bg: 'bg-sand-100', icon: <CheckCircle2 size={16} />, features: ['Student honour pledge', 'No monitoring'] },
  2: { label: 'Behavioural', description: 'Tab tracking, copy-paste block, IP + device fingerprinting.', color: 'text-brand-500', bg: 'bg-brand-50', icon: <Monitor size={16} />, features: ['Tab/focus tracking', 'Copy-paste disabled', 'IP + device logging', 'Full audit trail'] },
  3: { label: 'Camera Monitoring', description: 'Periodic screenshots, post-exam review, ML anomaly detection.', color: 'text-warning', bg: 'bg-warning-light', icon: <Camera size={16} />, features: ['All Tier 2 features', 'Camera capture', 'Screenshot intervals', 'ML face detection', 'Post-exam instructor review'] },
  4: { label: 'Live Invigilation', description: 'Real-time camera monitoring by invigilator, grid view.', color: 'text-danger', bg: 'bg-danger-light', icon: <Video size={16} />, features: ['All Tier 3 features', 'Live camera feed', '16-student grid view', 'Real-time warnings', 'Session termination'] },
};

const STATUS_META: Record<SessionStatus, { label: string; variant: 'success' | 'default' | 'brand' | 'danger' }> = {
  live: { label: 'Live', variant: 'success' },
  scheduled: { label: 'Scheduled', variant: 'default' },
  completed: { label: 'Completed', variant: 'brand' },
  flagged: { label: 'Flagged', variant: 'danger' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ProctoringPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('sessions');
  const [selectedTier, setSelectedTier] = useState<ProctoringTier>(2);

  const isAdmin = user?.role === 'admin' || user?.role === 'instructor';

  const liveCount = SESSIONS.filter(s => s.status === 'live').length;
  const totalFlagged = SESSIONS.reduce((s, sess) => s + sess.flaggedCount, 0);
  const monitoredSessions = SESSIONS.filter(s => s.avgFocusRate > 0);
  const avgFocus = monitoredSessions.length > 0
    ? Math.round(monitoredSessions.reduce((acc, s) => acc + s.avgFocusRate, 0) / monitoredSessions.length)
    : 0;

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Proctoring</h1>
            <p className="text-sm text-ink-tertiary mt-1">Exam integrity monitoring with 4-tier progressive enforcement</p>
          </div>
          {isAdmin && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><BarChart2 size={15} /> Integrity Report</Button>
              <Button size="sm"><ShieldCheck size={15} /> Configure Exam</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Eye size={14} className="text-success" /><span className="text-xs text-ink-tertiary">Live Sessions</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">{liveCount}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><AlertTriangle size={14} className="text-danger" /><span className="text-xs text-ink-tertiary">Flagged Incidents</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-danger">{totalFlagged}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Monitor size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Avg Focus Rate</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-brand-500">{avgFocus}%</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Shield size={14} className="text-gold-500" /><span className="text-xs text-ink-tertiary">Cost Saved</span></div>
          <div className="text-lg font-bold font-[family-name:var(--font-display)] text-gold-500">$12,400</div>
          <div className="text-[10px] text-ink-placeholder">vs. third-party proctoring</div>
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5 max-w-md">
          {([
            { key: 'sessions' as Tab, label: 'Sessions' },
            { key: 'settings' as Tab, label: 'Tier Settings' },
            { key: 'reports' as Tab, label: 'Reports' },
          ]).map(t => (
            <button key={t.key} onClick={() => setTab(t.key)}
              className={`flex-1 px-4 py-1.5 text-sm font-medium rounded-lg transition-all cursor-pointer ${
                tab === t.key ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {t.label}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Sessions */}
      {tab === 'sessions' && (
        <div className="space-y-3">
          {SESSIONS.map((session, i) => {
            const tierMeta = TIER_META[session.tier];
            const statusMeta = STATUS_META[session.status];
            const isLive = session.status === 'live';

            return (
              <motion.div key={session.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.04 }}>
                <Card hover padding="none" className={isLive ? 'border-success/40 bg-success-light/20' : ''}>
                  <div className="p-5">
                    <div className="flex items-start gap-4">
                      <div className={`w-12 h-12 rounded-xl ${tierMeta.bg} flex flex-col items-center justify-center shrink-0`}>
                        <span className={tierMeta.color}>{tierMeta.icon}</span>
                        <span className={`text-[9px] font-bold mt-0.5 ${tierMeta.color}`}>T{session.tier}</span>
                      </div>

                      <div className="flex-1 min-w-0">
                        <div className="flex items-start justify-between gap-3">
                          <div>
                            <div className="flex items-center gap-2 flex-wrap">
                              <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{session.examTitle}</h3>
                              {isLive && (
                                <span className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-success text-white text-[10px] font-bold uppercase tracking-wide animate-pulse">
                                  <span className="w-1.5 h-1.5 rounded-full bg-white" /> Live
                                </span>
                              )}
                            </div>
                            <div className="flex items-center gap-2 mt-1">
                              <span className="text-xs px-2 py-0.5 rounded-full font-medium bg-brand-50 text-brand-600 border border-brand-100">{session.courseCode}</span>
                              <span className={`text-xs px-2 py-0.5 rounded-full font-medium ${tierMeta.bg} ${tierMeta.color}`}>{tierMeta.label}</span>
                            </div>
                          </div>
                          <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                        </div>

                        <div className="flex items-center gap-5 mt-3 flex-wrap">
                          <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                            <Clock size={13} />
                            <span>{session.date} · {session.duration} min</span>
                          </div>
                          <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                            <Users size={13} />
                            <span>{session.students} students</span>
                          </div>
                          {session.status !== 'scheduled' && session.tier >= 2 && (
                            <>
                              {session.flaggedCount > 0 && (
                                <div className="flex items-center gap-1.5 text-xs text-danger font-medium">
                                  <AlertTriangle size={13} />
                                  <span>{session.flaggedCount} flagged</span>
                                </div>
                              )}
                              {session.tabSwitches > 0 && (
                                <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                                  <Monitor size={13} />
                                  <span>{session.tabSwitches} tab switches</span>
                                </div>
                              )}
                              {session.avgFocusRate > 0 && (
                                <div className="flex items-center gap-1.5 text-xs text-ink-tertiary">
                                  <Eye size={13} />
                                  <span>Focus: <span className={`font-medium ${session.avgFocusRate >= 90 ? 'text-success' : 'text-warning'}`}>{session.avgFocusRate}%</span></span>
                                </div>
                              )}
                            </>
                          )}
                        </div>
                      </div>

                      <div className="flex items-center gap-2 shrink-0">
                        {isLive && isAdmin && (
                          <>
                            <Button size="sm"><Eye size={14} /> Monitor</Button>
                            {session.tier >= 4 && <Button variant="outline" size="sm"><Video size={14} /> Grid View</Button>}
                          </>
                        )}
                        {session.status === 'completed' && <Button variant="ghost" size="sm"><BarChart2 size={14} /> Report</Button>}
                        <ChevronRight size={16} className="text-ink-tertiary" />
                      </div>
                    </div>
                  </div>
                </Card>
              </motion.div>
            );
          })}
        </div>
      )}

      {/* Tier Settings */}
      {tab === 'settings' && (
        <div className="space-y-4">
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
            {([1, 2, 3, 4] as ProctoringTier[]).map(tier => {
              const meta = TIER_META[tier];
              const isSelected = selectedTier === tier;
              return (
                <button key={tier} onClick={() => setSelectedTier(tier)}
                  className={`p-4 rounded-xl border-2 text-left transition-all cursor-pointer ${
                    isSelected ? 'border-brand-300 bg-brand-50/50 shadow-sm' : 'border-sand-200 bg-surface-raised hover:border-brand-200'
                  }`}>
                  <div className={`w-10 h-10 rounded-xl ${meta.bg} flex items-center justify-center mb-3`}>
                    <span className={meta.color}>{meta.icon}</span>
                  </div>
                  <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">Tier {tier}</div>
                  <div className="text-xs font-medium text-ink-secondary mt-0.5">{meta.label}</div>
                  <div className="text-[11px] text-ink-tertiary mt-1 leading-relaxed">{meta.description}</div>
                </button>
              );
            })}
          </div>

          <Card>
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">
              Tier {selectedTier} — {TIER_META[selectedTier].label} Features
            </h3>
            <div className="space-y-2">
              {TIER_META[selectedTier].features.map((feature, i) => (
                <div key={i} className="flex items-center gap-3 p-2.5 rounded-lg bg-surface-sunken">
                  <CheckCircle2 size={14} className="text-success shrink-0" />
                  <span className="text-sm text-ink">{feature}</span>
                </div>
              ))}
            </div>

            {selectedTier >= 3 && (
              <div className="mt-4 p-3 rounded-lg bg-warning-light/50 border border-warning/20">
                <div className="flex items-start gap-2">
                  <AlertTriangle size={14} className="text-warning shrink-0 mt-0.5" />
                  <div className="text-xs text-warning">
                    <span className="font-semibold">Privacy notice:</span> Camera data is processed on the institution's server only. Students see exactly what data is collected before the exam begins. Data is deleted after the configured retention period (default: 30 days).
                  </div>
                </div>
              </div>
            )}

            {selectedTier >= 2 && (
              <div className="mt-4 space-y-3">
                <h4 className="text-xs font-semibold text-ink-tertiary uppercase tracking-wider">Configuration</h4>
                <div className="grid grid-cols-2 gap-3">
                  <div className="p-3 rounded-lg bg-surface-sunken">
                    <div className="text-xs text-ink-tertiary">Tab switch threshold</div>
                    <div className="text-sm font-semibold text-ink mt-0.5">5 switches → auto-flag</div>
                  </div>
                  <div className="p-3 rounded-lg bg-surface-sunken">
                    <div className="text-xs text-ink-tertiary">Copy-paste</div>
                    <div className="text-sm font-semibold text-danger mt-0.5">Disabled</div>
                  </div>
                  {selectedTier >= 3 && (
                    <>
                      <div className="p-3 rounded-lg bg-surface-sunken">
                        <div className="text-xs text-ink-tertiary">Screenshot interval</div>
                        <div className="text-sm font-semibold text-ink mt-0.5">Every 60 seconds</div>
                      </div>
                      <div className="p-3 rounded-lg bg-surface-sunken">
                        <div className="text-xs text-ink-tertiary">Data retention</div>
                        <div className="text-sm font-semibold text-ink mt-0.5">30 days</div>
                      </div>
                    </>
                  )}
                  {selectedTier >= 4 && (
                    <div className="p-3 rounded-lg bg-surface-sunken">
                      <div className="text-xs text-ink-tertiary">Grid view capacity</div>
                      <div className="text-sm font-semibold text-ink mt-0.5">16 students per view</div>
                    </div>
                  )}
                </div>
              </div>
            )}
          </Card>

          {/* Browser extension */}
          <Card padding="sm" className="bg-brand-50/50 border-brand-100">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 rounded-lg bg-brand-100 flex items-center justify-center">
                  <Lock size={16} className="text-brand-600" />
                </div>
                <div>
                  <div className="text-sm font-semibold text-brand-700">Browser Lockdown Extension</div>
                  <div className="text-xs text-brand-500/70">Optional — open-source Chrome/Firefox extension</div>
                </div>
              </div>
              <Button variant="outline" size="sm"><Settings size={14} /> Configure</Button>
            </div>
          </Card>
        </div>
      )}

      {/* Reports */}
      {tab === 'reports' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <Card>
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Integrity Overview</h3>
            <div className="space-y-3">
              {[
                { label: 'Average focus rate', value: `${avgFocus}%`, color: 'text-success' },
                { label: 'Total tab switches', value: '93', color: 'text-ink-secondary' },
                { label: 'Flagged incidents', value: String(totalFlagged), color: 'text-danger' },
                { label: 'Sessions proctored', value: String(SESSIONS.filter(s => s.status === 'completed').length), color: 'text-ink-secondary' },
                { label: 'Students monitored', value: '353', color: 'text-ink-secondary' },
              ].map((item, i) => (
                <div key={i} className="flex items-center justify-between p-2.5 rounded-lg bg-surface-sunken">
                  <span className="text-xs text-ink-tertiary">{item.label}</span>
                  <span className={`text-sm font-bold font-[family-name:var(--font-display)] ${item.color}`}>{item.value}</span>
                </div>
              ))}
            </div>
          </Card>
          <Card>
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Recent Flags</h3>
            <div className="space-y-3">
              {[
                { student: 'Student #0087', exam: 'CS301 CAT 2', reason: '8 tab switches in 5 minutes', severity: 'high' },
                { student: 'Student #0134', exam: 'MAT301 Mid-Sem', reason: 'Multiple faces detected in frame', severity: 'high' },
                { student: 'Student #0042', exam: 'CS301 CAT 2', reason: '3 tab switches', severity: 'low' },
                { student: 'Student #0091', exam: 'MAT301 Mid-Sem', reason: 'Student absent from frame (2 min)', severity: 'medium' },
                { student: 'Student #0156', exam: 'MAT301 Mid-Sem', reason: 'Copy-paste attempt blocked', severity: 'low' },
              ].map((flag, i) => (
                <div key={i} className="flex items-start gap-3 p-2.5 rounded-lg bg-surface-sunken">
                  <AlertTriangle size={13} className={`shrink-0 mt-0.5 ${flag.severity === 'high' ? 'text-danger' : flag.severity === 'medium' ? 'text-warning' : 'text-ink-placeholder'}`} />
                  <div className="flex-1">
                    <div className="text-xs font-medium text-ink">{flag.reason}</div>
                    <div className="text-[11px] text-ink-tertiary mt-0.5">{flag.student} · {flag.exam}</div>
                  </div>
                  <Badge variant={flag.severity === 'high' ? 'danger' : flag.severity === 'medium' ? 'warning' : 'default'}>{flag.severity}</Badge>
                </div>
              ))}
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
