import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Briefcase, Search, Plus, MapPin, Clock, Building2,
  Users, Star, ExternalLink, ChevronRight, Eye,
  Filter, TrendingUp, GraduationCap, FileText,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { ProgressBar } from '../../components/ui/ProgressBar';
import { useAuth } from '../../context/AuthContext';

type Tab = 'jobs' | 'internships' | 'career_profile';
type JobType = 'full_time' | 'part_time' | 'internship' | 'contract';
type JobStatus = 'open' | 'closed' | 'applied' | 'interview' | 'offered';

interface JobListing {
  id: string;
  title: string;
  company: string;
  location: string;
  type: JobType;
  status: JobStatus;
  postedDate: string;
  deadline: string;
  description: string;
  requirements: string[];
  salary?: string;
  applicants: number;
  isSponsored: boolean;
}

interface InternshipTracking {
  id: string;
  company: string;
  role: string;
  startDate: string;
  endDate: string;
  hoursLogged: number;
  totalHours: number;
  supervisorName: string;
  rating?: number;
  status: 'active' | 'completed' | 'pending_evaluation';
}

const JOBS: JobListing[] = [
  { id: '1', title: 'Junior Software Engineer', company: 'Safaricom PLC', location: 'Nairobi, Kenya', type: 'full_time', status: 'open', postedDate: '3 days ago', deadline: 'Apr 30, 2026', description: 'Join our engineering team building M-Pesa and digital services', requirements: ['BSc CS or SE', 'Java/Kotlin', 'REST APIs'], salary: 'KES 120k–180k/month', applicants: 42, isSponsored: true },
  { id: '2', title: 'Data Analyst Intern', company: 'Kenya Airways', location: 'Nairobi, Kenya', type: 'internship', status: 'open', postedDate: '1 week ago', deadline: 'Apr 20, 2026', description: 'Support the data analytics team during the summer period', requirements: ['Python/R', 'SQL', 'Data Visualization'], applicants: 87, isSponsored: false },
  { id: '3', title: 'Frontend Developer', company: 'Andela', location: 'Remote (Africa)', type: 'full_time', status: 'applied', postedDate: '2 weeks ago', deadline: 'Apr 15, 2026', description: 'Build web applications for global clients', requirements: ['React/Vue', 'TypeScript', '2+ years experience'], salary: '$2k–4k/month', applicants: 156, isSponsored: false },
  { id: '4', title: 'IT Support Intern', company: 'University of Nairobi', location: 'Nairobi, Kenya', type: 'internship', status: 'interview', postedDate: '3 weeks ago', deadline: 'Mar 31, 2026', description: 'Assist with campus IT infrastructure and support', requirements: ['Networking basics', 'Windows/Linux', 'Customer service'], applicants: 23, isSponsored: false },
  { id: '5', title: 'Mobile Developer (Contract)', company: 'Twiga Foods', location: 'Nairobi, Kenya', type: 'contract', status: 'open', postedDate: '5 days ago', deadline: 'May 10, 2026', description: '6-month contract building the supply chain mobile app', requirements: ['Flutter/React Native', 'Firebase', 'Agile'], salary: 'KES 200k–300k/month', applicants: 31, isSponsored: true },
  { id: '6', title: 'Cloud Engineer', company: 'Microsoft ADC', location: 'Nairobi, Kenya', type: 'full_time', status: 'open', postedDate: '1 day ago', deadline: 'May 15, 2026', description: 'Work on Azure cloud infrastructure and developer tools', requirements: ['Cloud certifications', 'DevOps', 'Distributed systems'], salary: 'KES 350k–500k/month', applicants: 19, isSponsored: true },
];

const INTERNSHIPS: InternshipTracking[] = [
  { id: '1', company: 'Safaricom PLC', role: 'Software Engineering Intern', startDate: 'Jan 2026', endDate: 'Apr 2026', hoursLogged: 320, totalHours: 480, supervisorName: 'Eng. John Karanja', rating: 4.5, status: 'active' },
  { id: '2', company: 'KCB Group', role: 'IT Security Intern', startDate: 'Jun 2025', endDate: 'Sep 2025', hoursLogged: 480, totalHours: 480, supervisorName: 'Ms. Patricia Njoki', rating: 4.8, status: 'completed' },
];

const TYPE_META: Record<JobType, { label: string; color: string }> = {
  full_time: { label: 'Full-time', color: 'bg-brand-50 text-brand-600 border-brand-100' },
  part_time: { label: 'Part-time', color: 'bg-accent-50 text-accent-500 border-accent-100' },
  internship: { label: 'Internship', color: 'bg-gold-50 text-gold-600 border-gold-100' },
  contract: { label: 'Contract', color: 'bg-info-light text-info border-info/20' },
};

const STATUS_META: Record<JobStatus, { label: string; variant: 'success' | 'default' | 'brand' | 'warning' | 'danger' }> = {
  open: { label: 'Open', variant: 'success' },
  closed: { label: 'Closed', variant: 'default' },
  applied: { label: 'Applied', variant: 'brand' },
  interview: { label: 'Interview', variant: 'warning' },
  offered: { label: 'Offered', variant: 'success' },
};

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function EmployerPortalPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<Tab>('jobs');
  const [search, setSearch] = useState('');
  const [typeFilter, setTypeFilter] = useState<JobType | 'all'>('all');

  const isAdmin = user?.role === 'admin';
  const isAlumni = user?.role === 'alumni';

  const filteredJobs = JOBS.filter(j => {
    if (typeFilter !== 'all' && j.type !== typeFilter) return false;
    if (search && !j.title.toLowerCase().includes(search.toLowerCase()) && !j.company.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Career & Employer Portal</h1>
            <p className="text-sm text-ink-tertiary mt-1">
              {isAdmin ? 'Manage employer partnerships, job board, and graduate outcomes' : isAlumni ? 'Explore career opportunities and manage your graduate profile' : 'Find jobs, track internships, and build your career profile'}
            </p>
          </div>
          {isAdmin && (
            <div className="flex gap-2">
              <Button variant="outline" size="sm"><TrendingUp size={15} /> Outcomes Report</Button>
              <Button size="sm"><Plus size={15} /> Add Employer</Button>
            </div>
          )}
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.06 }} className="grid grid-cols-2 lg:grid-cols-4 gap-3">
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Briefcase size={14} className="text-brand-500" /><span className="text-xs text-ink-tertiary">Open Positions</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{JOBS.filter(j => j.status === 'open').length}</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><Building2 size={14} className="text-accent-400" /><span className="text-xs text-ink-tertiary">Partner Companies</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-accent-500">18</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><GraduationCap size={14} className="text-success" /><span className="text-xs text-ink-tertiary">Placed Graduates</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-success">87%</div>
          <div className="text-[10px] text-ink-placeholder">Class of 2025</div>
        </div>
        <div className="bg-surface-raised rounded-xl border border-sand-200 p-4">
          <div className="flex items-center gap-1.5 mb-1"><FileText size={14} className="text-gold-500" /><span className="text-xs text-ink-tertiary">My Applications</span></div>
          <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">{JOBS.filter(j => j.status === 'applied' || j.status === 'interview').length}</div>
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.1 }}>
        <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5 max-w-md">
          {([
            { key: 'jobs' as Tab, label: 'Job Board' },
            { key: 'internships' as Tab, label: 'Internships' },
            { key: 'career_profile' as Tab, label: 'Career Profile' },
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

      {/* Job Board */}
      {tab === 'jobs' && (
        <>
          <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.12 }} className="flex items-center justify-between gap-4 flex-wrap">
            <div className="flex bg-sand-100 rounded-xl p-1 gap-0.5">
              {(['all', 'full_time', 'internship', 'contract'] as (JobType | 'all')[]).map(t => (
                <button key={t} onClick={() => setTypeFilter(t)}
                  className={`px-3 py-1.5 text-xs font-medium rounded-lg transition-all cursor-pointer ${
                    typeFilter === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
                  }`}>
                  {t === 'all' ? 'All' : TYPE_META[t as JobType].label}
                </button>
              ))}
            </div>
            <div className="flex gap-2 flex-1 justify-end">
              <div className="relative max-w-xs flex-1">
                <Search size={15} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
                <input type="text" placeholder="Search jobs..." value={search} onChange={e => setSearch(e.target.value)}
                  className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400" />
              </div>
              <button className="p-2 rounded-lg border border-sand-300 text-ink-tertiary hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">
                <Filter size={16} />
              </button>
            </div>
          </motion.div>

          <div className="space-y-3">
            {filteredJobs.map((job, i) => {
              const typeMeta = TYPE_META[job.type];
              const statusMeta = STATUS_META[job.status];
              return (
                <motion.div key={job.id} initial={{ opacity: 0, x: -8 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: i * 0.04 }}>
                  <Card hover padding="none">
                    <div className="p-5">
                      <div className="flex items-start gap-4">
                        <div className="w-12 h-12 rounded-xl bg-surface-sunken flex items-center justify-center shrink-0">
                          <Building2 size={20} className="text-ink-tertiary" />
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-start justify-between gap-3">
                            <div>
                              <div className="flex items-center gap-2 flex-wrap">
                                <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{job.title}</h3>
                                {job.isSponsored && <span className="text-[10px] px-1.5 py-0.5 rounded bg-gold-50 text-gold-600 font-semibold border border-gold-200">Sponsored</span>}
                              </div>
                              <div className="flex items-center gap-3 mt-1 text-xs text-ink-tertiary">
                                <span className="font-medium text-ink-secondary">{job.company}</span>
                                <span className="flex items-center gap-1"><MapPin size={11} /> {job.location}</span>
                              </div>
                            </div>
                            <div className="flex items-center gap-2 shrink-0">
                              <span className={`text-xs px-2 py-0.5 rounded-full font-medium border ${typeMeta.color}`}>{typeMeta.label}</span>
                              {(job.status === 'applied' || job.status === 'interview' || job.status === 'offered') && (
                                <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
                              )}
                            </div>
                          </div>
                          <p className="text-xs text-ink-tertiary mt-2 line-clamp-1">{job.description}</p>
                          <div className="flex items-center gap-4 mt-3 flex-wrap">
                            {job.salary && <span className="text-xs font-semibold text-success">{job.salary}</span>}
                            <span className="text-xs text-ink-placeholder flex items-center gap-1"><Clock size={11} /> {job.postedDate}</span>
                            <span className="text-xs text-ink-placeholder flex items-center gap-1"><Users size={11} /> {job.applicants} applicants</span>
                            <span className="text-xs text-ink-placeholder">Deadline: {job.deadline}</span>
                          </div>
                          <div className="flex items-center gap-1.5 mt-2 flex-wrap">
                            {job.requirements.map(r => (
                              <span key={r} className="text-[10px] px-1.5 py-0.5 rounded bg-sand-100 text-ink-secondary font-medium">{r}</span>
                            ))}
                          </div>
                        </div>
                        <div className="flex items-center gap-2 shrink-0">
                          {job.status === 'open' && <Button size="sm">Apply</Button>}
                          {job.status === 'applied' && <Button variant="outline" size="sm"><Eye size={14} /> View</Button>}
                          {job.status === 'interview' && <Button size="sm" className="bg-warning hover:bg-warning/90">Prepare</Button>}
                          <ChevronRight size={16} className="text-ink-tertiary" />
                        </div>
                      </div>
                    </div>
                  </Card>
                </motion.div>
              );
            })}
          </div>
        </>
      )}

      {/* Internships tracking */}
      {tab === 'internships' && (
        <div className="space-y-4">
          {INTERNSHIPS.map((intern, i) => (
            <motion.div key={intern.id} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
              <Card>
                <div className="flex items-start gap-4">
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center shrink-0 ${intern.status === 'active' ? 'bg-success-light' : 'bg-surface-sunken'}`}>
                    <Building2 size={20} className={intern.status === 'active' ? 'text-success' : 'text-ink-tertiary'} />
                  </div>
                  <div className="flex-1">
                    <div className="flex items-start justify-between">
                      <div>
                        <h3 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{intern.role}</h3>
                        <div className="text-xs text-ink-tertiary mt-0.5">{intern.company} · {intern.startDate} — {intern.endDate}</div>
                      </div>
                      <Badge variant={intern.status === 'active' ? 'success' : intern.status === 'completed' ? 'default' : 'warning'}>
                        {intern.status === 'active' ? 'Active' : intern.status === 'completed' ? 'Completed' : 'Pending Eval'}
                      </Badge>
                    </div>

                    <div className="mt-3 grid grid-cols-3 gap-4">
                      <div>
                        <span className="text-xs text-ink-tertiary">Hours Logged</span>
                        <div className="mt-1 flex items-center gap-2">
                          <div className="flex-1"><ProgressBar value={(intern.hoursLogged / intern.totalHours) * 100} size="sm" color={intern.status === 'completed' ? 'success' : 'brand'} /></div>
                          <span className="text-xs font-semibold text-ink-secondary">{intern.hoursLogged}/{intern.totalHours}</span>
                        </div>
                      </div>
                      <div>
                        <span className="text-xs text-ink-tertiary">Supervisor</span>
                        <div className="text-xs font-medium text-ink mt-0.5">{intern.supervisorName}</div>
                      </div>
                      {intern.rating && (
                        <div>
                          <span className="text-xs text-ink-tertiary">Rating</span>
                          <div className="flex items-center gap-1 mt-0.5">
                            <Star size={12} className="text-gold-500" fill="currentColor" />
                            <span className="text-sm font-bold text-ink font-[family-name:var(--font-display)]">{intern.rating}</span>
                          </div>
                        </div>
                      )}
                    </div>

                    {intern.status === 'active' && (
                      <div className="flex gap-2 mt-3">
                        <Button variant="outline" size="sm"><Clock size={12} /> Log Hours</Button>
                        <Button variant="ghost" size="sm"><FileText size={12} /> Weekly Report</Button>
                      </div>
                    )}
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>
      )}

      {/* Career Profile */}
      {tab === 'career_profile' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <Card>
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Career Profile</h3>
            <div className="space-y-4">
              <div>
                <label className="text-xs text-ink-tertiary font-medium">Skills</label>
                <div className="flex items-center gap-1.5 mt-1.5 flex-wrap">
                  {['React', 'TypeScript', 'Python', 'SQL', 'Java', 'Git', 'REST APIs', 'Docker'].map(s => (
                    <span key={s} className="text-xs px-2 py-1 rounded-lg bg-brand-50 text-brand-600 font-medium">{s}</span>
                  ))}
                  <button className="text-xs px-2 py-1 rounded-lg border border-dashed border-sand-300 text-ink-placeholder hover:text-ink hover:border-brand-300 transition-colors cursor-pointer">+ Add</button>
                </div>
              </div>
              <div>
                <label className="text-xs text-ink-tertiary font-medium">Bio</label>
                <p className="text-sm text-ink-secondary mt-1">Third-year Computer Science student passionate about full-stack development and machine learning applications.</p>
              </div>
              <div>
                <label className="text-xs text-ink-tertiary font-medium">Portfolio Link</label>
                <div className="flex items-center gap-2 mt-1">
                  <span className="text-sm text-brand-500">portfolio.smartlms.io/james-mwangi</span>
                  <ExternalLink size={12} className="text-ink-placeholder" />
                </div>
              </div>
            </div>
          </Card>
          <Card>
            <h3 className="text-sm font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Graduate Outcome Declaration</h3>
            <div className="p-4 rounded-xl bg-surface-sunken text-center">
              <GraduationCap size={32} className="mx-auto text-ink-placeholder mb-2" />
              <p className="text-sm text-ink-tertiary">Available after graduation</p>
              <p className="text-xs text-ink-placeholder mt-1">Declare your employment status to support institutional accreditation reporting</p>
            </div>
          </Card>
        </div>
      )}
    </div>
  );
}
