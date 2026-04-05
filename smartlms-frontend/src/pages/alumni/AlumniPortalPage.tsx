import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  GraduationCap, Download, Search, Briefcase,
  Users, Award, BookOpen, MapPin, Calendar,
  ExternalLink, ChevronRight, Star,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';

const TABS = ['overview', 'directory', 'jobs', 'courses'] as const;
type Tab = typeof TABS[number];

interface AlumniMember {
  id: string;
  name: string;
  gradYear: number;
  programme: string;
  company: string;
  role: string;
  location: string;
}

const ALUMNI: AlumniMember[] = [
  { id: '1', name: 'Kevin Njoroge', gradYear: 2023, programme: 'BSc Computer Science', company: 'Safaricom PLC', role: 'Software Engineer', location: 'Nairobi, Kenya' },
  { id: '2', name: 'Amina Hassan', gradYear: 2022, programme: 'BSc Computer Science', company: 'Andela', role: 'Senior Developer', location: 'Nairobi, Kenya' },
  { id: '3', name: 'James Ochieng', gradYear: 2021, programme: 'BSc Computer Science', company: 'Google', role: 'SDE II', location: 'London, UK' },
  { id: '4', name: 'Wanjiku Kamau', gradYear: 2023, programme: 'BSc IT', company: 'Microsoft', role: 'Cloud Engineer', location: 'Dublin, Ireland' },
  { id: '5', name: 'Peter Muthoni', gradYear: 2020, programme: 'BSc Computer Science', company: 'Equity Bank', role: 'Tech Lead', location: 'Nairobi, Kenya' },
  { id: '6', name: 'Sarah Njeri', gradYear: 2022, programme: 'BSc IT', company: 'Twiga Foods', role: 'Data Analyst', location: 'Nairobi, Kenya' },
];

const JOBS = [
  { id: '1', title: 'Software Engineer', company: 'Safaricom PLC', location: 'Nairobi', type: 'Full-time', posted: '2d ago', salary: 'KSh 150k - 250k' },
  { id: '2', title: 'Graduate Intern — Data Science', company: 'KCB Group', location: 'Nairobi', type: 'Internship', posted: '1d ago', salary: 'KSh 50k - 80k' },
  { id: '3', title: 'Frontend Developer', company: 'Andela', location: 'Remote', type: 'Full-time', posted: '3d ago', salary: 'USD 2k - 4k' },
  { id: '4', title: 'Cloud Infrastructure Engineer', company: 'Africa\'s Talking', location: 'Nairobi', type: 'Full-time', posted: '5d ago', salary: 'KSh 200k - 350k' },
];

const CPD_COURSES = [
  { id: '1', title: 'Cloud Architecture with AWS', provider: 'University of Nairobi CPD', duration: '6 weeks', price: 'KSh 15,000', enrolled: 45 },
  { id: '2', title: 'Advanced Data Analytics with Python', provider: 'School of Computing', duration: '8 weeks', price: 'KSh 20,000', enrolled: 32 },
  { id: '3', title: 'Cybersecurity Essentials', provider: 'University of Nairobi CPD', duration: '4 weeks', price: 'KSh 12,000', enrolled: 67 },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function AlumniPortalPage() {
  const [tab, setTab] = useState<Tab>('overview');
  const [search, setSearch] = useState('');

  const filteredAlumni = ALUMNI.filter(a =>
    !search || a.name.toLowerCase().includes(search.toLowerCase()) || a.company.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Alumni Portal</h1>
            <p className="text-sm text-ink-tertiary mt-1">Stay connected, find opportunities, and continue learning</p>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm"><Download size={14} /> Transcript</Button>
            <Button size="sm"><Award size={14} /> My Credentials</Button>
          </div>
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-1 bg-sand-100 rounded-xl p-1 w-fit">
          {TABS.map(t => (
            <button key={t} onClick={() => setTab(t)}
              className={`px-4 py-1.5 text-xs font-medium rounded-lg transition-all capitalize cursor-pointer ${
                tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {t}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Overview */}
      {tab === 'overview' && (
        <div className="space-y-5">
          <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.1 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
            <div className="bg-brand-50 rounded-xl border border-brand-200 p-4">
              <GraduationCap size={18} className="text-brand-500 mb-2" />
              <div className="text-xs text-brand-600 font-medium">Class of 2023</div>
              <div className="text-xl font-bold font-[family-name:var(--font-display)] text-brand-700 mt-0.5">BSc Computer Science</div>
            </div>
            <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
              <div className="text-xs text-ink-tertiary">Final GPA</div>
              <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">3.45</div>
              <div className="text-xs text-ink-tertiary mt-0.5">Second Class Honours (Upper)</div>
            </div>
            <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
              <div className="text-xs text-ink-tertiary">Digital Badges</div>
              <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">7</div>
              <div className="text-xs text-brand-500 mt-0.5 cursor-pointer">View all →</div>
            </div>
            <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
              <div className="text-xs text-ink-tertiary">Network</div>
              <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{ALUMNI.length}+</div>
              <div className="text-xs text-ink-tertiary mt-0.5">connected alumni</div>
            </div>
          </motion.div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.15 }}>
              <Card>
                <div className="flex items-center justify-between mb-4">
                  <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">Latest Job Postings</h3>
                  <button onClick={() => setTab('jobs')} className="text-xs text-brand-500 font-medium cursor-pointer">View all →</button>
                </div>
                <div className="space-y-3">
                  {JOBS.slice(0, 3).map(job => (
                    <div key={job.id} className="flex items-center gap-3 p-3 rounded-lg border border-sand-200 hover:border-brand-200 transition-colors cursor-pointer">
                      <Briefcase size={16} className="text-brand-500 shrink-0" />
                      <div className="flex-1 min-w-0">
                        <div className="text-sm font-semibold text-ink">{job.title}</div>
                        <div className="text-xs text-ink-tertiary">{job.company} · {job.location}</div>
                      </div>
                      <Badge variant="default" size="sm">{job.type}</Badge>
                    </div>
                  ))}
                </div>
              </Card>
            </motion.div>
            <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.2 }}>
              <Card>
                <div className="flex items-center justify-between mb-4">
                  <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink">CPD Courses</h3>
                  <button onClick={() => setTab('courses')} className="text-xs text-brand-500 font-medium cursor-pointer">View all →</button>
                </div>
                <div className="space-y-3">
                  {CPD_COURSES.map(course => (
                    <div key={course.id} className="flex items-center gap-3 p-3 rounded-lg border border-sand-200 hover:border-brand-200 transition-colors cursor-pointer">
                      <BookOpen size={16} className="text-accent-400 shrink-0" />
                      <div className="flex-1 min-w-0">
                        <div className="text-sm font-semibold text-ink">{course.title}</div>
                        <div className="text-xs text-ink-tertiary">{course.duration} · {course.price}</div>
                      </div>
                      <Badge variant="accent" size="sm">{course.enrolled} enrolled</Badge>
                    </div>
                  ))}
                </div>
              </Card>
            </motion.div>
          </div>

          <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.25 }}>
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Quick Actions</h3>
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
                {[
                  { icon: <Download size={18} />, label: 'Download Transcript', color: 'text-brand-500' },
                  { icon: <Award size={18} />, label: 'View Certificates', color: 'text-accent-400' },
                  { icon: <Users size={18} />, label: 'Alumni Directory', color: 'text-gold-500' },
                  { icon: <Briefcase size={18} />, label: 'Update Employment', color: 'text-info' },
                ].map(action => (
                  <button key={action.label} className="p-4 rounded-xl border border-sand-200 bg-surface-raised hover:border-brand-200 transition-all cursor-pointer text-center">
                    <div className={`${action.color} mx-auto mb-2`}>{action.icon}</div>
                    <div className="text-xs font-medium text-ink">{action.label}</div>
                  </button>
                ))}
              </div>
            </Card>
          </motion.div>
        </div>
      )}

      {/* Directory */}
      {tab === 'directory' && (
        <div className="space-y-4">
          <div className="relative max-w-md">
            <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
            <input type="text" placeholder="Search alumni by name or company..." value={search} onChange={e => setSearch(e.target.value)}
              className="w-full bg-surface-raised border border-sand-300 rounded-xl pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            {filteredAlumni.map((alum, i) => (
              <motion.div key={alum.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
                <Card>
                  <div className="flex items-start gap-3">
                    <Avatar name={alum.name} size="lg" />
                    <div className="flex-1 min-w-0">
                      <div className="text-sm font-semibold text-ink">{alum.name}</div>
                      <div className="text-xs text-ink-tertiary">{alum.role} at {alum.company}</div>
                      <div className="flex items-center gap-1.5 mt-1">
                        <MapPin size={10} className="text-ink-placeholder" />
                        <span className="text-[10px] text-ink-tertiary">{alum.location}</span>
                      </div>
                      <div className="flex items-center gap-2 mt-2">
                        <Badge variant="default" size="sm">{alum.programme}</Badge>
                        <Badge variant="brand" size="sm">Class of {alum.gradYear}</Badge>
                      </div>
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </div>
      )}

      {/* Jobs */}
      {tab === 'jobs' && (
        <div className="space-y-3">
          {JOBS.map((job, i) => (
            <motion.div key={job.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
              <Card padding="none">
                <div className="flex items-center gap-4 p-4 hover:bg-sand-50 transition-colors cursor-pointer">
                  <div className="w-10 h-10 rounded-xl bg-brand-50 flex items-center justify-center shrink-0">
                    <Briefcase size={18} className="text-brand-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-semibold text-ink">{job.title}</div>
                    <div className="text-xs text-ink-tertiary">{job.company} · {job.location}</div>
                    <div className="flex items-center gap-2 mt-1.5">
                      <Badge variant="default" size="sm">{job.type}</Badge>
                      <span className="text-xs text-ink-tertiary">{job.salary}</span>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <span className="text-xs text-ink-placeholder">{job.posted}</span>
                    <div className="mt-1">
                      <Button variant="outline" size="sm"><ExternalLink size={12} /> Apply</Button>
                    </div>
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>
      )}

      {/* Courses */}
      {tab === 'courses' && (
        <div className="space-y-3">
          {CPD_COURSES.map((course, i) => (
            <motion.div key={course.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
              <Card padding="none">
                <div className="flex items-center gap-4 p-4 hover:bg-sand-50 transition-colors cursor-pointer">
                  <div className="w-10 h-10 rounded-xl bg-accent-50 flex items-center justify-center shrink-0">
                    <BookOpen size={18} className="text-accent-400" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-semibold text-ink">{course.title}</div>
                    <div className="text-xs text-ink-tertiary">{course.provider}</div>
                    <div className="flex items-center gap-2 mt-1.5">
                      <span className="flex items-center gap-1 text-xs text-ink-tertiary"><Clock size={11} /> {course.duration}</span>
                      <span className="text-xs text-ink-tertiary">·</span>
                      <span className="flex items-center gap-1 text-xs text-ink-tertiary"><Users size={11} /> {course.enrolled} enrolled</span>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <div className="text-sm font-bold font-[family-name:var(--font-display)] text-ink">{course.price}</div>
                    <Button size="sm" className="mt-1">Enrol</Button>
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>
      )}
    </div>
  );
}
