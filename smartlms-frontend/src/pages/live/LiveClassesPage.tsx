import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Video, Plus, Calendar, Clock, Users,
  Play, Square, ChevronRight, Radio,
  Monitor, Link2, Download,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

interface LiveSession {
  id: string;
  title: string;
  course: string;
  courseCode: string;
  instructor: string;
  scheduledAt: string;
  duration: number; // minutes
  status: 'live' | 'scheduled' | 'ended';
  platform: 'zoom' | 'meet' | 'jitsi';
  attendees?: number;
  maxAttendees?: number;
  recordingAvailable?: boolean;
  joinUrl?: string;
}

const SESSIONS: LiveSession[] = [
  { id: '1', title: 'Binary Search Tree — Live Coding Session', course: 'Data Structures & Algorithms', courseCode: 'CS301', instructor: 'Prof. James Mwangi', scheduledAt: 'Today, 10:00 AM', duration: 90, status: 'live', platform: 'zoom', attendees: 112, maxAttendees: 145, joinUrl: '#' },
  { id: '2', title: 'SQL Joins & Subqueries — Q&A Session', course: 'Database Systems', courseCode: 'CS302', instructor: 'Dr. Achieng Odhiambo', scheduledAt: 'Today, 2:00 PM', duration: 60, status: 'scheduled', platform: 'meet', maxAttendees: 198 },
  { id: '3', title: 'Graph Colouring Problems — Tutorial', course: 'Discrete Mathematics', courseCode: 'MAT301', instructor: 'Prof. Kariuki Maina', scheduledAt: 'Tomorrow, 9:00 AM', duration: 60, status: 'scheduled', platform: 'jitsi', maxAttendees: 132 },
  { id: '4', title: 'CAT 2 Review — Exam Prep', course: 'Data Structures & Algorithms', courseCode: 'CS301', instructor: 'Prof. James Mwangi', scheduledAt: 'Apr 8, 4:00 PM', duration: 90, status: 'scheduled', platform: 'zoom', maxAttendees: 145 },
  { id: '5', title: 'TCP/IP Deep Dive — Lab Session', course: 'Computer Networks', courseCode: 'CS305', instructor: 'Dr. Omondi Were', scheduledAt: 'Apr 6, 11:00 AM', duration: 120, status: 'ended', platform: 'zoom', attendees: 78, maxAttendees: 89, recordingAvailable: true },
  { id: '6', title: 'Normalisation Workshop', course: 'Database Systems', courseCode: 'CS302', instructor: 'Dr. Achieng Odhiambo', scheduledAt: 'Apr 3, 2:00 PM', duration: 60, status: 'ended', platform: 'meet', attendees: 162, maxAttendees: 198, recordingAvailable: true },
];

const PLATFORM_META = {
  zoom: { label: 'Zoom', color: 'text-info', bg: 'bg-info-light' },
  meet: { label: 'Google Meet', color: 'text-success', bg: 'bg-success-light' },
  jitsi: { label: 'Jitsi', color: 'text-brand-500', bg: 'bg-brand-50' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

function AttendeeBar({ current, max }: { current?: number; max?: number }) {
  if (!current || !max) return null;
  const pct = Math.round((current / max) * 100);
  return (
    <div className="flex items-center gap-2">
      <div className="flex-1 h-1.5 bg-sand-200 rounded-full overflow-hidden">
        <div className="h-full bg-success rounded-full" style={{ width: `${pct}%` }} />
      </div>
      <span className="text-xs text-ink-tertiary tabular-nums">{current}/{max}</span>
    </div>
  );
}

export function LiveClassesPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<'all' | 'live' | 'scheduled' | 'ended'>('all');
  const isInstructor = user?.role === 'admin' || user?.role === 'instructor';

  const liveSession = SESSIONS.find(s => s.status === 'live');
  const filtered = SESSIONS.filter(s => tab === 'all' || s.status === tab);

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Live Classes</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {SESSIONS.filter(s => s.status === 'live').length > 0 && (
                <span className="text-danger font-semibold">{SESSIONS.filter(s => s.status === 'live').length} session live now · </span>
              )}
              {SESSIONS.filter(s => s.status === 'scheduled').length} upcoming
            </p>
          </div>
          {isInstructor && (
            <Button size="sm"><Plus size={15} /> Schedule Session</Button>
          )}
        </div>
      </motion.div>

      {/* Live now banner */}
      {liveSession && (
        <motion.div
          initial={{ opacity: 0, scale: 0.98 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.4, delay: 0.1 }}
        >
          <div className="rounded-xl border-2 border-danger/30 bg-danger-light/40 overflow-hidden">
            <div className="flex items-center gap-2 bg-danger px-5 py-2">
              <span className="w-2 h-2 rounded-full bg-white animate-pulse" />
              <span className="text-white text-xs font-bold uppercase tracking-wider">Live Now</span>
            </div>
            <div className="p-5 flex items-center gap-4 flex-wrap">
              <div className="w-12 h-12 rounded-xl bg-danger/10 flex items-center justify-center shrink-0">
                <Radio size={24} className="text-danger" />
              </div>
              <div className="flex-1 min-w-0">
                <h3 className="font-bold font-[family-name:var(--font-display)] text-ink">{liveSession.title}</h3>
                <div className="flex items-center gap-3 mt-1 text-sm text-ink-secondary flex-wrap">
                  <span className="flex items-center gap-1.5"><span className="text-xs font-semibold text-brand-500 bg-brand-50 px-1.5 py-0.5 rounded">{liveSession.courseCode}</span>{liveSession.course}</span>
                  <span className="flex items-center gap-1.5"><Users size={13} />{liveSession.attendees} in session</span>
                </div>
                <AttendeeBar current={liveSession.attendees} max={liveSession.maxAttendees} />
              </div>
              <Button>
                <Play size={15} /> Join Now
              </Button>
            </div>
          </div>
        </motion.div>
      )}

      {/* Tabs */}
      <div className="flex bg-sand-100 rounded-xl p-1 w-fit gap-0.5">
        {(['all', 'live', 'scheduled', 'ended'] as const).map(t => (
          <button key={t} onClick={() => setTab(t)}
            className={`px-4 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
            {t === 'live' ? (
              <span className="flex items-center gap-1.5">
                <span className="w-1.5 h-1.5 rounded-full bg-danger animate-pulse" /> Live
              </span>
            ) : t === 'all' ? 'All' : t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </div>

      {/* Sessions */}
      <div className="space-y-3">
        {filtered.map((session, i) => {
          const platform = PLATFORM_META[session.platform];
          return (
            <motion.div key={session.id} initial={{ opacity: 0, x: -6 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.05 }}>
              <Card hover padding="none">
                <div className="p-5">
                  <div className="flex items-start gap-4">
                    {/* Icon */}
                    <div className={`w-12 h-12 rounded-xl flex items-center justify-center shrink-0 ${session.status === 'live' ? 'bg-danger-light' : session.status === 'scheduled' ? 'bg-brand-50' : 'bg-sand-100'}`}>
                      {session.status === 'live' ? <Radio size={22} className="text-danger" /> : session.status === 'scheduled' ? <Video size={22} className="text-brand-500" /> : <Video size={22} className="text-ink-tertiary" />}
                    </div>

                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 flex-wrap">
                        <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{session.title}</h3>
                        {session.status === 'live' && (
                          <span className="flex items-center gap-1 px-2 py-0.5 rounded-full bg-danger text-white text-[10px] font-bold animate-pulse">
                            <span className="w-1.5 h-1.5 rounded-full bg-white" /> LIVE
                          </span>
                        )}
                        {session.recordingAvailable && (
                          <Badge variant="default">Recording</Badge>
                        )}
                      </div>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-xs font-semibold text-brand-500 bg-brand-50 px-1.5 py-0.5 rounded border border-brand-100">{session.courseCode}</span>
                        <span className="text-xs text-ink-tertiary">{session.instructor}</span>
                      </div>

                      <div className="flex items-center gap-5 mt-3 flex-wrap text-xs text-ink-tertiary">
                        <span className="flex items-center gap-1.5"><Calendar size={13} />{session.scheduledAt}</span>
                        <span className="flex items-center gap-1.5"><Clock size={13} />{session.duration} min</span>
                        <span className={`flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-semibold ${platform.bg} ${platform.color}`}>
                          <Monitor size={10} /> {platform.label}
                        </span>
                        {session.attendees && (
                          <span className="flex items-center gap-1.5"><Users size={13} />{session.attendees}/{session.maxAttendees}</span>
                        )}
                      </div>
                    </div>

                    {/* CTA */}
                    <div className="flex items-center gap-2 shrink-0">
                      {session.status === 'live' && (
                        <Button size="sm"><Play size={14} /> Join</Button>
                      )}
                      {session.status === 'scheduled' && (
                        <Button variant="outline" size="sm"><Link2 size={14} /> Get Link</Button>
                      )}
                      {session.status === 'ended' && session.recordingAvailable && (
                        <Button variant="ghost" size="sm"><Download size={14} /> Recording</Button>
                      )}
                      {isInstructor && session.status === 'scheduled' && (
                        <Button variant="ghost" size="sm"><Square size={14} /> Cancel</Button>
                      )}
                      <ChevronRight size={16} className="text-ink-tertiary" />
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          );
        })}
      </div>
    </div>
  );
}
