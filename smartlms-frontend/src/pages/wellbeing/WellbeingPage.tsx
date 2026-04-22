import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Heart, TrendingUp, TrendingDown, Minus, Brain,
  Moon, AlertTriangle, MessageCircle, Calendar,
  ChevronRight, Users, Activity, Shield,
} from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

const WEEKLY_TREND = [
  { week: 'W1', mood: 4.2, workload: 3.8, sleep: 4.0 },
  { week: 'W2', mood: 4.0, workload: 3.5, sleep: 3.8 },
  { week: 'W3', mood: 3.6, workload: 2.8, sleep: 3.4 },
  { week: 'W4', mood: 3.2, workload: 2.4, sleep: 3.0 },
  { week: 'W5', mood: 3.5, workload: 2.6, sleep: 3.2 },
  { week: 'W6', mood: 3.8, workload: 3.0, sleep: 3.5 },
  { week: 'W7', mood: 3.4, workload: 2.2, sleep: 2.9 },
  { week: 'W8', mood: 3.6, workload: 2.8, sleep: 3.3 },
];

const COHORT_DATA = [
  { week: 'W1', average: 4.1 }, { week: 'W2', average: 3.9 }, { week: 'W3', average: 3.5 },
  { week: 'W4', average: 3.2 }, { week: 'W5', average: 3.3 }, { week: 'W6', average: 3.6 },
  { week: 'W7', average: 3.1 }, { week: 'W8', average: 3.4 },
];

const AT_RISK_STUDENTS = [
  { id: '1', name: 'Kevin Kamau', regNo: 'CS/2022/006', mood: 1.8, trend: 'down' as const, attendance: 38, submissions: 40, lastCheckIn: '12 days ago', flags: ['Low mood', 'Poor attendance', 'Missing submissions'] },
  { id: '2', name: 'Mary Wanjiku', regNo: 'CS/2022/003', mood: 2.2, trend: 'down' as const, attendance: 50, submissions: 60, lastCheckIn: '8 days ago', flags: ['Declining mood', 'Low attendance'] },
  { id: '3', name: 'Brian Otieno', regNo: 'CS/2022/002', mood: 2.5, trend: 'flat' as const, attendance: 81, submissions: 75, lastCheckIn: '3 days ago', flags: ['Low mood score'] },
];

const MOOD_LABELS = ['', 'Struggling', 'Difficult', 'Okay', 'Good', 'Great'];
const MOOD_COLORS = ['', 'text-danger', 'text-accent-400', 'text-warning', 'text-brand-500', 'text-success'];
const MOOD_BG = ['', 'bg-danger-light', 'bg-accent-50', 'bg-warning-light', 'bg-brand-50', 'bg-success-light'];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function WellbeingPage() {
  const { user } = useAuth();
  const [checkInDone, setCheckInDone] = useState(false);
  const [moodScore, setMoodScore] = useState(0);
  const [workloadScore, setWorkloadScore] = useState(0);
  const [sleepOk, setSleepOk] = useState<boolean | null>(null);

  const isStaff = user?.role === 'admin' || user?.role === 'instructor' || user?.role === 'counsellor';

  const handleSubmitCheckIn = () => {
    if (moodScore > 0 && workloadScore > 0 && sleepOk !== null) {
      setCheckInDone(true);
    }
  };

  // Student view
  if (!isStaff) {
    return (
      <div className="space-y-5 max-w-3xl">
        <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
          <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Wellbeing</h1>
          <p className="text-sm text-ink-tertiary mt-1">Your weekly check-in and wellbeing trends</p>
        </motion.div>

        {/* Weekly check-in */}
        {!checkInDone ? (
          <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
            <Card>
              <div className="flex items-center gap-2 mb-5">
                <div className="w-10 h-10 rounded-xl bg-brand-50 flex items-center justify-center">
                  <Heart size={20} className="text-brand-500" />
                </div>
                <div>
                  <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Weekly Check-In</h3>
                  <p className="text-xs text-ink-tertiary">Private and confidential. Takes 30 seconds.</p>
                </div>
              </div>

              <div className="space-y-6">
                <div>
                  <p className="text-sm font-medium text-ink mb-3">How are you feeling this week?</p>
                  <div className="flex gap-2">
                    {[1, 2, 3, 4, 5].map(score => (
                      <button
                        key={score}
                        onClick={() => setMoodScore(score)}
                        className={`flex-1 py-3 rounded-xl border-2 transition-all cursor-pointer text-center ${
                          moodScore === score
                            ? `${MOOD_BG[score]} border-current ${MOOD_COLORS[score]}`
                            : 'border-sand-200 bg-surface-raised text-ink-tertiary hover:border-sand-300'
                        }`}
                      >
                        <div className="text-xl mb-1">{['', '😔', '😕', '😐', '🙂', '😊'][score]}</div>
                        <div className="text-[10px] font-medium">{MOOD_LABELS[score]}</div>
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <p className="text-sm font-medium text-ink mb-3">How manageable is your workload?</p>
                  <div className="flex gap-2">
                    {[1, 2, 3, 4, 5].map(score => (
                      <button
                        key={score}
                        onClick={() => setWorkloadScore(score)}
                        className={`flex-1 py-2.5 rounded-xl border-2 transition-all cursor-pointer text-center ${
                          workloadScore === score
                            ? 'border-brand-300 bg-brand-50 text-brand-600'
                            : 'border-sand-200 bg-surface-raised text-ink-tertiary hover:border-sand-300'
                        }`}
                      >
                        <div className="text-xs font-semibold">{score}</div>
                        <div className="text-[9px]">{['', 'Overwhelmed', 'Heavy', 'Moderate', 'Manageable', 'Light'][score]}</div>
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <p className="text-sm font-medium text-ink mb-3">Are you sleeping enough?</p>
                  <div className="flex gap-3">
                    {[true, false].map(val => (
                      <button
                        key={String(val)}
                        onClick={() => setSleepOk(val)}
                        className={`flex-1 py-3 rounded-xl border-2 transition-all cursor-pointer flex items-center justify-center gap-2 ${
                          sleepOk === val
                            ? val ? 'border-success/40 bg-success-light text-success' : 'border-danger/40 bg-danger-light text-danger'
                            : 'border-sand-200 bg-surface-raised text-ink-tertiary hover:border-sand-300'
                        }`}
                      >
                        <Moon size={16} />
                        <span className="text-sm font-medium">{val ? 'Yes' : 'No'}</span>
                      </button>
                    ))}
                  </div>
                </div>

                <Button
                  onClick={handleSubmitCheckIn}
                  className="w-full"
                  disabled={moodScore === 0 || workloadScore === 0 || sleepOk === null}
                >
                  Submit Check-In
                </Button>
              </div>
            </Card>
          </motion.div>
        ) : (
          <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
            <div className="rounded-xl border-2 border-success/30 bg-success-light p-6 text-center">
              <div className="w-14 h-14 rounded-full bg-success/10 flex items-center justify-center mx-auto mb-3">
                <Heart size={24} className="text-success" />
              </div>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-success">Check-in Submitted</h3>
              <p className="text-sm text-success/80 mt-1">Thank you! Your response is private. Next check-in opens next Monday.</p>
            </div>
          </motion.div>
        )}

        {/* Personal trend */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.2 }}>
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Your Trends</h3>
              <Badge variant="default">Last 8 weeks</Badge>
            </div>
            <ResponsiveContainer width="100%" height={180}>
              <LineChart data={WEEKLY_TREND}>
                <XAxis dataKey="week" axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <YAxis domain={[1, 5]} axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <Tooltip contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 11 }} />
                <Line type="monotone" dataKey="mood" stroke="#0D5E6D" strokeWidth={2} name="Mood" dot={{ r: 3, fill: '#0D5E6D' }} />
                <Line type="monotone" dataKey="workload" stroke="#C75C2B" strokeWidth={2} name="Workload" dot={{ r: 3, fill: '#C75C2B' }} />
                <Line type="monotone" dataKey="sleep" stroke="#B88D2F" strokeWidth={2} name="Sleep" dot={{ r: 3, fill: '#B88D2F' }} />
              </LineChart>
            </ResponsiveContainer>
            <div className="flex items-center justify-center gap-4 mt-2">
              <span className="flex items-center gap-1.5 text-[10px] text-ink-tertiary"><span className="w-2.5 h-2.5 rounded-full bg-brand-500" /> Mood</span>
              <span className="flex items-center gap-1.5 text-[10px] text-ink-tertiary"><span className="w-2.5 h-2.5 rounded-full bg-accent-400" /> Workload</span>
              <span className="flex items-center gap-1.5 text-[10px] text-ink-tertiary"><span className="w-2.5 h-2.5 rounded-full bg-gold-500" /> Sleep</span>
            </div>
          </Card>
        </motion.div>

        {/* Quick actions */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.3 }}>
          <div className="grid grid-cols-2 gap-3">
            <button className="p-4 rounded-xl border border-sand-200 bg-surface-raised hover:border-brand-300 transition-all cursor-pointer text-left group">
              <MessageCircle size={20} className="text-brand-500 mb-2" />
              <div className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink">Talk to a Counsellor</div>
              <div className="text-xs text-ink-tertiary mt-0.5">Private & confidential</div>
            </button>
            <button className="p-4 rounded-xl border border-sand-200 bg-surface-raised hover:border-brand-300 transition-all cursor-pointer text-left group">
              <Calendar size={20} className="text-accent-400 mb-2" />
              <div className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink">Book Appointment</div>
              <div className="text-xs text-ink-tertiary mt-0.5">Available this week</div>
            </button>
          </div>
        </motion.div>
      </div>
    );
  }

  // Staff / Admin / Counsellor view
  return (
    <div className="space-y-5">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Student Wellbeing</h1>
            <p className="text-sm text-ink-tertiary mt-1">Aggregate trends and at-risk student monitoring</p>
          </div>
          <Button variant="outline" size="sm"><Shield size={14} /> Anonymised View</Button>
        </div>
      </motion.div>

      {/* KPIs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Activity size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Avg Mood</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">3.4<span className="text-sm text-ink-tertiary font-normal">/5</span></div>
          <div className="flex items-center gap-1 text-xs text-danger mt-1"><TrendingDown size={11} /> -0.2 from last week</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Users size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Check-In Rate</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">72%</div>
          <div className="flex items-center gap-1 text-xs text-success mt-1"><TrendingUp size={11} /> +5% from last week</div>
        </div>
        <div className="bg-danger-light rounded-xl border border-danger/20 p-4">
          <div className="flex items-center gap-1.5 mb-1"><AlertTriangle size={14} className="text-danger" /><span className="text-xs text-danger/70">At-Risk Students</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-danger">3</div>
          <div className="text-xs text-danger/60 mt-1">Mood ≤ 2.5 + declining</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Brain size={14} className="text-accent-400" /><span className="text-xs text-ink-tertiary">Top Concern</span></div>
          <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink mt-0.5">Workload</div>
          <div className="text-xs text-ink-tertiary mt-1">Correlates with exam proximity</div>
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Cohort trend chart */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.12 }} className="lg:col-span-2">
          <Card>
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Cohort Wellbeing Trend</h3>
                <p className="text-xs text-ink-tertiary">CS301 — Data Structures · Average mood score</p>
              </div>
              <Badge variant="warning">Declining</Badge>
            </div>
            <ResponsiveContainer width="100%" height={200}>
              <AreaChart data={COHORT_DATA}>
                <defs>
                  <linearGradient id="moodGrad" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#0D5E6D" stopOpacity={0.15} />
                    <stop offset="100%" stopColor="#0D5E6D" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <XAxis dataKey="week" axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <YAxis domain={[1, 5]} axisLine={false} tickLine={false} tick={{ fontSize: 10, fill: '#7A7E87' }} />
                <Tooltip contentStyle={{ borderRadius: 8, border: '1px solid #EDE6DB', fontSize: 11 }} />
                <Area type="monotone" dataKey="average" stroke="#0D5E6D" strokeWidth={2} fill="url(#moodGrad)" name="Avg Mood" />
              </AreaChart>
            </ResponsiveContainer>
          </Card>
        </motion.div>

        {/* Insight panel */}
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.16 }}>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Wellbeing Insights</h3>
            <div className="space-y-3">
              {[
                { icon: <TrendingDown size={14} />, text: 'CS301 cohort mood dropped this week', color: 'text-danger', bg: 'bg-danger-light' },
                { icon: <AlertTriangle size={14} />, text: 'Week 7 dip correlates with midterm deadlines', color: 'text-warning', bg: 'bg-warning-light' },
                { icon: <TrendingUp size={14} />, text: 'Workload scores improving post-extension', color: 'text-success', bg: 'bg-success-light' },
              ].map((insight, i) => (
                <div key={i} className={`flex items-start gap-2.5 p-3 rounded-lg ${insight.bg}`}>
                  <span className={`mt-0.5 ${insight.color}`}>{insight.icon}</span>
                  <span className="text-xs font-medium text-ink">{insight.text}</span>
                </div>
              ))}
            </div>
            <div className="mt-4 p-3 rounded-lg bg-brand-50 border border-brand-200">
              <p className="text-xs font-medium text-brand-700">Suggested Action</p>
              <p className="text-xs text-brand-600 mt-1">Consider extending the upcoming CS301 deadline — Week 8 assessment density is high.</p>
            </div>
          </Card>
        </motion.div>
      </div>

      {/* At-risk students */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.2 }}>
        <Card>
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">At-Risk Students</h3>
            <Badge variant="danger">{AT_RISK_STUDENTS.length} flagged</Badge>
          </div>
          <div className="space-y-3">
            {AT_RISK_STUDENTS.map((student, i) => (
              <motion.div
                key={student.id}
                initial={{ opacity: 0, x: -8 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: 0.25 + i * 0.06 }}
                className="flex items-center gap-4 p-4 rounded-xl border border-sand-200 hover:border-danger/30 hover:bg-danger-light/50 transition-all cursor-pointer group"
              >
                <div className="w-10 h-10 rounded-full bg-danger-light flex items-center justify-center text-danger text-sm font-bold font-[family-name:var(--font-display)] shrink-0">
                  {student.name.charAt(0)}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-semibold text-ink">{student.name}</span>
                    <span className="text-xs text-ink-tertiary">{student.regNo}</span>
                  </div>
                  <div className="flex flex-wrap gap-1.5 mt-1.5">
                    {student.flags.map(flag => (
                      <span key={flag} className="px-2 py-0.5 text-[10px] rounded-full bg-danger-light text-danger font-medium">{flag}</span>
                    ))}
                  </div>
                </div>
                <div className="text-right shrink-0">
                  <div className={`text-lg font-bold font-[family-name:var(--font-display)] ${student.mood <= 2 ? 'text-danger' : 'text-warning'}`}>
                    {student.mood.toFixed(1)}
                    {student.trend === 'down' && <TrendingDown size={12} className="inline ml-1" />}
                    {student.trend === 'flat' && <Minus size={12} className="inline ml-1" />}
                  </div>
                  <div className="text-[10px] text-ink-tertiary">Last: {student.lastCheckIn}</div>
                </div>
                <ChevronRight size={14} className="text-ink-placeholder group-hover:text-danger transition-colors shrink-0" />
              </motion.div>
            ))}
          </div>
        </Card>
      </motion.div>
    </div>
  );
}
