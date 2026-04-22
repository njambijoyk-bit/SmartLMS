import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  ChevronLeft, ChevronRight, Plus, Calendar,
  Clock, MapPin, Video,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

const DAYS = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
const SHORT_DAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'];
const HOURS = Array.from({ length: 12 }, (_, i) => i + 7); // 7 AM to 6 PM

interface ClassSlot {
  id: string;
  course: string;
  code: string;
  type: 'lecture' | 'lab' | 'tutorial' | 'exam';
  instructor: string;
  venue: string;
  isOnline?: boolean;
  day: number; // 0–4
  startHour: number;
  duration: number; // hours
  color: string;
  enrolled?: number;
}

const CLASSES: ClassSlot[] = [
  { id: '1', course: 'Data Structures & Algorithms', code: 'CS301', type: 'lecture', instructor: 'Prof. Mwangi', venue: 'LH-201', day: 0, startHour: 8, duration: 2, color: 'bg-brand-500', enrolled: 145 },
  { id: '2', course: 'Database Systems', code: 'CS302', type: 'lecture', instructor: 'Dr. Achieng', venue: 'LH-102', day: 0, startHour: 11, duration: 2, color: 'bg-accent-400', enrolled: 198 },
  { id: '3', course: 'Discrete Mathematics', code: 'MAT301', type: 'lecture', instructor: 'Prof. Kariuki', venue: 'LH-301', day: 1, startHour: 9, duration: 2, color: 'bg-gold-500', enrolled: 132 },
  { id: '4', course: 'Computer Networks', code: 'CS305', type: 'lecture', instructor: 'Dr. Omondi', venue: 'LH-204', day: 1, startHour: 14, duration: 2, color: 'bg-brand-300', enrolled: 89 },
  { id: '5', course: 'Data Structures & Algorithms', code: 'CS301', type: 'lab', instructor: 'Prof. Mwangi', venue: 'Lab-3', day: 2, startHour: 10, duration: 3, color: 'bg-brand-500', enrolled: 145 },
  { id: '6', course: 'Database Systems', code: 'CS302', type: 'tutorial', instructor: 'TA Kamau', venue: 'Online', isOnline: true, day: 2, startHour: 15, duration: 1, color: 'bg-accent-400', enrolled: 45 },
  { id: '7', course: 'Discrete Mathematics', code: 'MAT301', type: 'tutorial', instructor: 'TA Njeri', venue: 'SH-104', day: 3, startHour: 8, duration: 1, color: 'bg-gold-500', enrolled: 40 },
  { id: '8', course: 'Computer Networks', code: 'CS305', type: 'lab', instructor: 'Dr. Omondi', venue: 'Net-Lab', day: 3, startHour: 11, duration: 3, color: 'bg-brand-300', enrolled: 89 },
  { id: '9', course: 'Data Structures & Algorithms', code: 'CS301', type: 'lecture', instructor: 'Prof. Mwangi', venue: 'LH-201', day: 4, startHour: 9, duration: 2, color: 'bg-brand-500', enrolled: 145 },
  { id: '10', course: 'Discrete Mathematics', code: 'MAT301', type: 'lecture', instructor: 'Prof. Kariuki', venue: 'LH-301', day: 4, startHour: 14, duration: 2, color: 'bg-gold-500', enrolled: 132 },
];

const TYPE_LABELS = { lecture: 'Lecture', lab: 'Lab', tutorial: 'Tutorial', exam: 'Exam' };

const today = new Date();
const rawDay = today.getDay();
const todayDayIndex = rawDay === 0 || rawDay === 6 ? 4 : rawDay - 1; // Mon=0

export function TimetablePage() {
  const [week, setWeek] = useState(0);
  const [view, setView] = useState<'week' | 'list'>('week');

  const todayClasses = CLASSES.filter(c => c.day === todayDayIndex).sort((a, b) => a.startHour - b.startHour);

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Timetable</h1>
            <p className="text-sm text-ink-tertiary mt-1">Semester 1, 2025/26 academic year</p>
          </div>
          <div className="flex gap-2">
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {(['week', 'list'] as const).map(v => (
                <button key={v} onClick={() => setView(v)} className={`px-3 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${view === v ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'}`}>
                  {v === 'week' ? 'Week' : 'List'}
                </button>
              ))}
            </div>
            <Button size="sm"><Plus size={15} /> Add Class</Button>
          </div>
        </div>
      </motion.div>

      {/* Week navigator */}
      <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.3, delay: 0.1 }}>
        <div className="flex items-center gap-3">
          <button onClick={() => setWeek(w => w - 1)} className="p-1.5 rounded-lg border border-sand-300 hover:border-brand-300 text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
            <ChevronLeft size={16} />
          </button>
          <span className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">
            {week === 0 ? 'This Week' : week === 1 ? 'Next Week' : week === -1 ? 'Last Week' : `Week ${week > 0 ? '+' : ''}${week}`}
          </span>
          <button onClick={() => setWeek(w => w + 1)} className="p-1.5 rounded-lg border border-sand-300 hover:border-brand-300 text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
            <ChevronRight size={16} />
          </button>
          {week !== 0 && (
            <button onClick={() => setWeek(0)} className="text-xs text-brand-500 hover:text-brand-600 font-medium cursor-pointer">
              Today
            </button>
          )}
        </div>
      </motion.div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
        {/* Weekly grid */}
        <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4, delay: 0.15 }} className="lg:col-span-3">
          <Card padding="none">
            {/* Day headers */}
            <div className="grid grid-cols-[60px_repeat(5,1fr)] border-b border-sand-200">
              <div className="p-3" />
              {DAYS.map((day, i) => (
                <div key={day} className={`p-3 text-center border-l border-sand-100 ${i === todayDayIndex && week === 0 ? 'bg-brand-50' : ''}`}>
                  <div className={`text-xs font-medium ${i === todayDayIndex && week === 0 ? 'text-brand-600' : 'text-ink-tertiary'}`}>{SHORT_DAYS[i]}</div>
                  <div className={`text-lg font-bold font-[family-name:var(--font-display)] ${i === todayDayIndex && week === 0 ? 'text-brand-600' : 'text-ink'}`}>
                    {today.getDate() - todayDayIndex + i + (week * 7)}
                  </div>
                </div>
              ))}
            </div>

            {/* Time slots */}
            <div className="relative overflow-auto max-h-[520px]">
              {HOURS.map(hour => (
                <div key={hour} className="grid grid-cols-[60px_repeat(5,1fr)] border-b border-sand-100 min-h-[60px]">
                  <div className="p-2 text-right">
                    <span className="text-[10px] text-ink-placeholder font-medium">{hour > 12 ? `${hour - 12}PM` : hour === 12 ? '12PM' : `${hour}AM`}</span>
                  </div>
                  {DAYS.map((_, dayIndex) => {
                    const slot = CLASSES.find(c => c.day === dayIndex && c.startHour === hour);
                    return (
                      <div key={dayIndex} className={`relative border-l border-sand-100 min-h-[60px] ${dayIndex === todayDayIndex && week === 0 ? 'bg-brand-50/30' : ''}`}>
                        {slot && (
                          <div
                            className={`absolute inset-x-1 top-1 ${slot.color} text-white rounded-lg px-2 py-1.5 overflow-hidden cursor-pointer hover:opacity-90 transition-opacity`}
                            style={{ height: `${slot.duration * 60 - 6}px`, zIndex: 1 }}
                          >
                            <div className="text-[10px] font-bold leading-tight font-[family-name:var(--font-display)]">{slot.code}</div>
                            <div className="text-[9px] opacity-85 leading-tight truncate">{TYPE_LABELS[slot.type]}</div>
                            {slot.duration >= 1.5 && (
                              <div className="text-[9px] opacity-75 mt-0.5 flex items-center gap-0.5 truncate">
                                {slot.isOnline ? <Video size={8} /> : <MapPin size={8} />}
                                {slot.venue}
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              ))}
            </div>
          </Card>
        </motion.div>

        {/* Sidebar — today's classes */}
        <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4, delay: 0.2 }} className="space-y-4">
          <Card>
            <div className="flex items-center gap-2 mb-4">
              <Calendar size={16} className="text-brand-500" />
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Today's Classes</h3>
            </div>
            {todayClasses.length === 0 ? (
              <p className="text-sm text-ink-tertiary text-center py-6">No classes today</p>
            ) : (
              <div className="space-y-3">
                {todayClasses.map(cls => (
                  <div key={cls.id} className="flex gap-3 group cursor-pointer">
                    <div className={`w-1 rounded-full shrink-0 ${cls.color}`} />
                    <div className="flex-1 min-w-0">
                      <div className="text-xs font-bold font-[family-name:var(--font-display)] text-ink">{cls.code}</div>
                      <div className="text-xs text-ink-tertiary truncate">{cls.course}</div>
                      <div className="flex items-center gap-2 mt-1.5">
                        <span className="flex items-center gap-1 text-[10px] text-ink-tertiary">
                          <Clock size={10} />
                          {cls.startHour > 12 ? `${cls.startHour - 12}:00 PM` : `${cls.startHour}:00 AM`}
                        </span>
                        <span className="flex items-center gap-1 text-[10px] text-ink-tertiary">
                          {cls.isOnline ? <Video size={10} /> : <MapPin size={10} />}
                          {cls.venue}
                        </span>
                      </div>
                      <Badge variant="default" className="mt-1.5 text-[9px] px-1.5 py-0.5">{TYPE_LABELS[cls.type]}</Badge>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </Card>

          {/* Legend */}
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink text-sm mb-3">Courses</h3>
            <div className="space-y-2">
              {[
                { code: 'CS301', name: 'Data Structures', color: 'bg-brand-500' },
                { code: 'CS302', name: 'Database Systems', color: 'bg-accent-400' },
                { code: 'MAT301', name: 'Discrete Math', color: 'bg-gold-500' },
                { code: 'CS305', name: 'Computer Networks', color: 'bg-brand-300' },
              ].map(c => (
                <div key={c.code} className="flex items-center gap-2.5">
                  <div className={`w-3 h-3 rounded-sm ${c.color} shrink-0`} />
                  <span className="text-xs font-medium text-ink-secondary">{c.code}</span>
                  <span className="text-xs text-ink-tertiary truncate">{c.name}</span>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
