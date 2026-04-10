import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Download, QrCode, AlertTriangle,
  CheckCircle2, Clock, GraduationCap, Printer,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

interface ExamEntry {
  course: string;
  code: string;
  date: string;
  time: string;
  venue: string;
  seat: string;
  duration: string;
  type: string;
}

const EXAMS: ExamEntry[] = [
  { course: 'Data Structures & Algorithms', code: 'CS301', date: 'Apr 28, 2026', time: '9:00 AM', venue: 'Main Hall A', seat: 'A-24', duration: '3 hours', type: 'End of Semester' },
  { course: 'Database Systems', code: 'CS302', date: 'Apr 30, 2026', time: '2:00 PM', venue: 'Main Hall B', seat: 'B-11', duration: '3 hours', type: 'End of Semester' },
  { course: 'Discrete Mathematics', code: 'MAT301', date: 'May 2, 2026', time: '9:00 AM', venue: 'Block C, Room 12', seat: 'C-08', duration: '3 hours', type: 'End of Semester' },
  { course: 'Computer Networks', code: 'CS305', date: 'May 5, 2026', time: '2:00 PM', venue: 'Main Hall A', seat: 'A-32', duration: '3 hours', type: 'End of Semester' },
];

const STUDENT = {
  name: 'Faith Wanjiku Kamau',
  regNo: 'CS/2022/001',
  programme: 'BSc Computer Science',
  year: 'Year 3, Semester 1',
  academicYear: '2025/2026',
  photo: null,
  feeStatus: 'cleared',
  cardNo: 'EC-2026-CS-001',
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function ExamCardsPage() {
  const { user } = useAuth();
  const isAdmin = user?.role === 'admin';
  const [showQR, setShowQR] = useState(false);

  return (
    <div className="space-y-5 max-w-4xl">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Exam Card</h1>
            <p className="text-sm text-ink-tertiary mt-1">End of Semester Examinations — Semester 1, 2025/26</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={() => setShowQR(!showQR)}>
              <QrCode size={14} /> {showQR ? 'Hide QR' : 'Show QR'}
            </Button>
            <Button size="sm"><Printer size={14} /> Print Card</Button>
            <Button variant="outline" size="sm"><Download size={14} /> PDF</Button>
          </div>
        </div>
      </motion.div>

      {/* Eligibility check */}
      {STUDENT.feeStatus !== 'cleared' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
          <div className="rounded-xl bg-danger-light border border-danger/20 p-4 flex items-start gap-3">
            <AlertTriangle size={20} className="text-danger shrink-0 mt-0.5" />
            <div>
              <div className="font-semibold text-danger font-[family-name:var(--font-display)]">Exam card not available</div>
              <p className="text-sm text-danger/80 mt-0.5">Your fee account has an outstanding balance. Please clear your fees at the Finance Office to obtain your exam card.</p>
            </div>
          </div>
        </motion.div>
      )}

      {/* Official exam card */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
        <div className="rounded-2xl border-2 border-brand-500 bg-surface-raised overflow-hidden shadow-lg">
          {/* Card header */}
          <div className="bg-brand-500 px-6 py-4 flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
                <GraduationCap size={22} className="text-white" />
              </div>
              <div>
                <div className="text-white font-bold font-[family-name:var(--font-display)] text-lg leading-tight">SmartLMS University</div>
                <div className="text-white/70 text-xs">Examination Card · {STUDENT.academicYear}</div>
              </div>
            </div>
            <div className="text-right">
              <div className="text-white/70 text-[10px] uppercase tracking-wider">Card No.</div>
              <div className="text-white font-mono font-bold text-sm">{STUDENT.cardNo}</div>
              {STUDENT.feeStatus === 'cleared' && (
                <div className="flex items-center gap-1 text-white/90 text-[10px] mt-1">
                  <CheckCircle2 size={11} /> Fee Cleared
                </div>
              )}
            </div>
          </div>

          {/* Student info */}
          <div className="px-6 py-5 border-b border-sand-200">
            <div className="flex items-start gap-5">
              {/* Photo placeholder */}
              <div className="w-20 h-24 rounded-xl bg-sand-200 flex flex-col items-center justify-center text-ink-placeholder shrink-0 border-2 border-sand-300">
                <GraduationCap size={24} />
                <span className="text-[10px] mt-1">Photo</span>
              </div>
              <div className="flex-1 grid grid-cols-2 gap-3">
                {[
                  { label: 'Full Name', value: STUDENT.name },
                  { label: 'Registration No.', value: STUDENT.regNo },
                  { label: 'Programme', value: STUDENT.programme },
                  { label: 'Year of Study', value: STUDENT.year },
                ].map(({ label, value }) => (
                  <div key={label}>
                    <div className="text-[10px] text-ink-tertiary uppercase tracking-wider">{label}</div>
                    <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] mt-0.5">{value}</div>
                  </div>
                ))}
              </div>
              {showQR && (
                <div className="shrink-0">
                  {/* QR code placeholder */}
                  <div className="w-24 h-24 bg-ink rounded-xl flex items-center justify-center p-2">
                    <div className="w-full h-full border-4 border-white rounded-sm flex items-center justify-center">
                      <QrCode size={36} className="text-white" />
                    </div>
                  </div>
                  <div className="text-[9px] text-ink-tertiary text-center mt-1">Scan to verify</div>
                </div>
              )}
            </div>
          </div>

          {/* Exam schedule */}
          <div className="px-6 py-4">
            <div className="text-xs font-bold text-ink-tertiary uppercase tracking-wider mb-3">Examination Schedule</div>
            <div className="space-y-2">
              {EXAMS.map((exam, i) => (
                <motion.div key={i} initial={{ opacity: 0, x: -4 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: 0.2 + i * 0.07 }}
                  className="grid grid-cols-[auto_1fr_auto_auto] gap-3 items-center py-2.5 border-b border-sand-100 last:border-0">
                  <div className="text-xs font-bold text-brand-500 bg-brand-50 px-2 py-1 rounded border border-brand-100 font-[family-name:var(--font-display)]">
                    {exam.code}
                  </div>
                  <div>
                    <div className="text-sm font-medium text-ink">{exam.course}</div>
                    <div className="text-xs text-ink-tertiary">{exam.type}</div>
                  </div>
                  <div className="text-right">
                    <div className="text-xs font-semibold text-ink">{exam.date}</div>
                    <div className="text-xs text-ink-tertiary flex items-center gap-1 justify-end"><Clock size={10} />{exam.time} · {exam.duration}</div>
                  </div>
                  <div className="text-right">
                    <div className="text-xs font-semibold text-ink">{exam.venue}</div>
                    <div className="text-xs text-brand-500 font-bold">Seat {exam.seat}</div>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>

          {/* Footer */}
          <div className="px-6 py-3 bg-sand-100 border-t border-sand-200">
            <p className="text-[10px] text-ink-tertiary leading-relaxed">
              This card is valid only for the examinations listed above. Present it to the invigilator before each examination. Tampering with this card is a disciplinary offence.
              Issued: Apr 5, 2026. Verify at: verify.smartlms.io/{STUDENT.cardNo}
            </p>
          </div>
        </div>
      </motion.div>

      {/* Admin view: student list */}
      {isAdmin && (
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.3 }}>
          <Card>
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Exam Card Status — All Students</h3>
              <Badge variant="warning">3 pending clearance</Badge>
            </div>
            <div className="space-y-2">
              {['Faith Kamau · Cleared', 'Brian Otieno · Cleared', 'Mary Wanjiku · Fee Balance KSh 85,000', 'Daniel Mutua · Cleared', 'Kevin Kamau · Not Applied'].map((s, i) => (
                <div key={i} className="flex items-center justify-between py-2 border-b border-sand-100 last:border-0 text-sm">
                  <span className="text-ink">{s.split(' · ')[0]}</span>
                  <Badge variant={s.includes('Cleared') ? 'success' : 'danger'}>{s.split(' · ')[1]}</Badge>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      )}
    </div>
  );
}
