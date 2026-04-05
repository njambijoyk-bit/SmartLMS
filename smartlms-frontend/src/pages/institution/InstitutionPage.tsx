import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Building2, Globe, Palette, Shield, Database,
  Server, Key, Users, Upload, CheckCircle2,
  AlertTriangle, Settings, Mail, Clock,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';

const TABS = ['general', 'branding', 'security', 'integrations', 'billing'] as const;
type Tab = typeof TABS[number];

const TAB_META: Record<Tab, { label: string; icon: React.ReactNode }> = {
  general: { label: 'General', icon: <Building2 size={15} /> },
  branding: { label: 'Branding', icon: <Palette size={15} /> },
  security: { label: 'Security', icon: <Shield size={15} /> },
  integrations: { label: 'Integrations', icon: <Database size={15} /> },
  billing: { label: 'Plan & Billing', icon: <Key size={15} /> },
};

const INTEGRATIONS = [
  { name: 'Google Workspace', desc: 'SSO via Google accounts', icon: '🔵', connected: true },
  { name: 'Microsoft 365', desc: 'SSO and calendar sync', icon: '🟦', connected: true },
  { name: 'M-Pesa', desc: 'Mobile payments via Safaricom', icon: '🟢', connected: true },
  { name: 'Stripe', desc: 'Card and international payments', icon: '🟣', connected: false },
  { name: 'Zoom', desc: 'Live class video conferencing', icon: '🔵', connected: true },
  { name: 'Jitsi', desc: 'Self-hosted video conferencing', icon: '🟠', connected: false },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

export function InstitutionPage() {
  const [tab, setTab] = useState<Tab>('general');

  return (
    <div className="space-y-5">
      {/* Header */}
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Institution Settings</h1>
            <p className="text-sm text-ink-tertiary mt-1">Manage your institution's configuration, branding, and integrations</p>
          </div>
          <Button size="sm"><Settings size={14} /> Save Changes</Button>
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div {...fadeIn} transition={{ duration: 0.3, delay: 0.08 }}>
        <div className="flex gap-1 bg-sand-100 rounded-xl p-1 w-fit">
          {TABS.map(t => (
            <button key={t} onClick={() => setTab(t)}
              className={`flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-lg transition-all cursor-pointer ${
                tab === t ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
              }`}>
              {TAB_META[t].icon} {TAB_META[t].label}
            </button>
          ))}
        </div>
      </motion.div>

      {/* General */}
      {tab === 'general' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }} className="grid grid-cols-1 lg:grid-cols-3 gap-4">
          <div className="lg:col-span-2 space-y-4">
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Institution Details</h3>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Institution Name</label>
                  <input type="text" defaultValue="University of Nairobi" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Slug</label>
                  <input type="text" defaultValue="uon" className="w-full bg-sand-50 border border-sand-300 rounded-xl px-3 py-2.5 text-sm text-ink-tertiary" readOnly />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Custom Domain</label>
                  <input type="text" defaultValue="lms.uon.ac.ke" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Country</label>
                  <input type="text" defaultValue="Kenya" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Timezone</label>
                  <input type="text" defaultValue="Africa/Nairobi (EAT +3)" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Default Language</label>
                  <input type="text" defaultValue="English" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
              </div>
              <div className="mt-4">
                <label className="text-xs font-medium text-ink-secondary mb-1.5 block">About</label>
                <textarea defaultValue="Premier institution of higher learning in East Africa, committed to quality education, research, and community service." rows={3}
                  className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 resize-none" />
              </div>
            </Card>

            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Academic Configuration</h3>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Academic Year</label>
                  <input type="text" defaultValue="2025/2026" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Current Semester</label>
                  <input type="text" defaultValue="Semester 1" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Grade Scale</label>
                  <select className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300">
                    <option>Letter Grades (A–F)</option>
                    <option>GPA 4.0</option>
                    <option>Percentage Only</option>
                  </select>
                </div>
                <div>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Student ID Format</label>
                  <input type="text" defaultValue="{YEAR}{PROGRAMME_CODE}{SEQUENCE}" className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-brand-300" />
                </div>
              </div>
            </Card>
          </div>

          {/* Sidebar */}
          <div className="space-y-4">
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">System Status</h3>
              <div className="space-y-3">
                {[
                  { label: 'Engine', status: 'healthy', icon: <Server size={14} /> },
                  { label: 'Database', status: 'healthy', icon: <Database size={14} /> },
                  { label: 'Email', status: 'healthy', icon: <Mail size={14} /> },
                  { label: 'SSL Certificate', status: 'valid', icon: <Shield size={14} /> },
                ].map(item => (
                  <div key={item.label} className="flex items-center gap-2.5">
                    <span className="text-ink-tertiary">{item.icon}</span>
                    <span className="text-xs text-ink flex-1">{item.label}</span>
                    <Badge variant="success" size="sm">{item.status}</Badge>
                  </div>
                ))}
              </div>
            </Card>
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Quick Stats</h3>
              <div className="space-y-2.5">
                <div className="flex justify-between text-xs"><span className="text-ink-tertiary">Active Users</span><span className="font-semibold text-ink">1,247</span></div>
                <div className="flex justify-between text-xs"><span className="text-ink-tertiary">Courses</span><span className="font-semibold text-ink">86</span></div>
                <div className="flex justify-between text-xs"><span className="text-ink-tertiary">Storage Used</span><span className="font-semibold text-ink">23.4 GB</span></div>
                <div className="flex justify-between text-xs"><span className="text-ink-tertiary">API Requests (today)</span><span className="font-semibold text-ink">45,210</span></div>
                <div className="flex justify-between text-xs"><span className="text-ink-tertiary">Uptime</span><span className="font-semibold text-success">99.98%</span></div>
              </div>
            </Card>
            <Card>
              <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Licence</h3>
              <div className="flex items-center gap-2 p-3 rounded-lg bg-brand-50 border border-brand-200">
                <Key size={16} className="text-brand-500" />
                <div>
                  <div className="text-xs font-semibold text-brand-700">Growth Plan</div>
                  <div className="text-[10px] text-brand-600">Valid until Dec 31, 2026</div>
                </div>
              </div>
            </Card>
          </div>
        </motion.div>
      )}

      {/* Branding */}
      {tab === 'branding' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }} className="space-y-4 max-w-3xl">
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Visual Identity</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
              <div>
                <label className="text-xs font-medium text-ink-secondary mb-2 block">Institution Logo</label>
                <div className="border-2 border-dashed border-sand-300 rounded-xl p-8 text-center hover:border-brand-300 transition-colors cursor-pointer">
                  <Upload size={24} className="mx-auto text-ink-placeholder mb-2" />
                  <p className="text-xs text-ink-tertiary">Drop logo here or click to upload</p>
                  <p className="text-[10px] text-ink-placeholder mt-1">SVG or PNG, max 2 MB</p>
                </div>
              </div>
              <div>
                <label className="text-xs font-medium text-ink-secondary mb-2 block">Favicon</label>
                <div className="border-2 border-dashed border-sand-300 rounded-xl p-8 text-center hover:border-brand-300 transition-colors cursor-pointer">
                  <Upload size={24} className="mx-auto text-ink-placeholder mb-2" />
                  <p className="text-xs text-ink-tertiary">Drop favicon here or click to upload</p>
                  <p className="text-[10px] text-ink-placeholder mt-1">32x32 or 64x64 PNG</p>
                </div>
              </div>
            </div>
          </Card>

          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Theme Colors</h3>
            <p className="text-xs text-ink-tertiary mb-4">These colors are injected as CSS variables across the entire LMS UI</p>
            <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
              {[
                { label: 'Primary', value: '#0D5E6D', css: '--brand-500' },
                { label: 'Accent', value: '#C75C2B', css: '--accent-400' },
                { label: 'Surface', value: '#FDFCFA', css: '--surface' },
                { label: 'Text', value: '#1A1D23', css: '--ink' },
              ].map(color => (
                <div key={color.label}>
                  <label className="text-xs font-medium text-ink-secondary mb-1.5 block">{color.label}</label>
                  <div className="flex items-center gap-2 p-2 rounded-xl border border-sand-300 bg-surface-raised">
                    <div className="w-8 h-8 rounded-lg shrink-0" style={{ background: color.value }} />
                    <input type="text" defaultValue={color.value} className="text-xs font-mono text-ink flex-1 bg-transparent focus:outline-none" />
                  </div>
                  <div className="text-[10px] text-ink-placeholder mt-1">{color.css}</div>
                </div>
              ))}
            </div>
          </Card>

          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Email Templates</h3>
            <p className="text-xs text-ink-tertiary mb-3">White-labelled email templates use your institution's branding</p>
            <div className="space-y-2">
              {['Welcome Email', 'Password Reset', 'Grade Notification', 'Fee Reminder', 'Exam Card Issued'].map(template => (
                <div key={template} className="flex items-center justify-between p-3 rounded-lg border border-sand-200 bg-surface-raised hover:border-brand-200 transition-colors cursor-pointer">
                  <div className="flex items-center gap-2">
                    <Mail size={14} className="text-brand-500" />
                    <span className="text-sm font-medium text-ink">{template}</span>
                  </div>
                  <Badge variant="success" size="sm">Active</Badge>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      )}

      {/* Security */}
      {tab === 'security' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }} className="space-y-4 max-w-3xl">
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Authentication</h3>
            <div className="space-y-4">
              {[
                { label: 'Password authentication', desc: 'Students and staff sign in with email and password', enabled: true },
                { label: 'Google SSO', desc: 'Sign in with institutional Google Workspace accounts', enabled: true },
                { label: 'Microsoft SSO', desc: 'Sign in with Microsoft 365 accounts', enabled: true },
                { label: 'SAML 2.0', desc: 'Enterprise SSO via SAML identity provider', enabled: false },
                { label: 'Two-factor authentication', desc: 'Require 2FA for admin and instructor accounts', enabled: false },
              ].map(item => (
                <div key={item.label} className="flex items-center justify-between p-3 rounded-lg border border-sand-200 bg-surface-raised">
                  <div>
                    <div className="text-sm font-medium text-ink">{item.label}</div>
                    <div className="text-xs text-ink-tertiary mt-0.5">{item.desc}</div>
                  </div>
                  <div className={`w-10 h-5.5 rounded-full relative cursor-pointer ${item.enabled ? 'bg-success' : 'bg-sand-300'}`}>
                    <div className={`absolute top-0.5 w-4.5 h-4.5 rounded-full bg-white shadow-sm transition-transform ${item.enabled ? 'left-[18px]' : 'left-0.5'}`} />
                  </div>
                </div>
              ))}
            </div>
          </Card>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Session & Rate Limiting</h3>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label className="text-xs font-medium text-ink-secondary mb-1.5 block">Session Timeout</label>
                <select className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300">
                  <option>30 minutes</option>
                  <option>1 hour</option>
                  <option>4 hours</option>
                  <option>8 hours</option>
                </select>
              </div>
              <div>
                <label className="text-xs font-medium text-ink-secondary mb-1.5 block">API Rate Limit</label>
                <select className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300">
                  <option>100 req/min (Growth)</option>
                  <option>500 req/min (Enterprise)</option>
                </select>
              </div>
            </div>
          </Card>
        </motion.div>
      )}

      {/* Integrations */}
      {tab === 'integrations' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }} className="space-y-4 max-w-3xl">
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Connected Services</h3>
            <div className="space-y-3">
              {INTEGRATIONS.map(int => (
                <div key={int.name} className="flex items-center gap-3 p-3.5 rounded-xl border border-sand-200 bg-surface-raised">
                  <span className="text-xl">{int.icon}</span>
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-semibold text-ink">{int.name}</div>
                    <div className="text-xs text-ink-tertiary">{int.desc}</div>
                  </div>
                  {int.connected ? (
                    <Badge variant="success" size="sm"><CheckCircle2 size={10} /> Connected</Badge>
                  ) : (
                    <Button variant="outline" size="sm">Connect</Button>
                  )}
                </div>
              ))}
            </div>
          </Card>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Webhooks</h3>
            <p className="text-xs text-ink-tertiary mb-3">Send event notifications to external services</p>
            <Button variant="outline" size="sm"><Plus size={14} /> Add Webhook</Button>
          </Card>
        </motion.div>
      )}

      {/* Billing */}
      {tab === 'billing' && (
        <motion.div {...fadeIn} transition={{ duration: 0.3 }} className="space-y-4 max-w-3xl">
          <div className="rounded-xl border-2 border-brand-200 bg-brand-50 p-6">
            <div className="flex items-center justify-between">
              <div>
                <Badge variant="brand" size="md">Growth Plan</Badge>
                <div className="text-3xl font-bold font-[family-name:var(--font-display)] text-brand-700 mt-2">$299<span className="text-sm font-normal text-brand-500">/month</span></div>
                <p className="text-xs text-brand-600 mt-1">Billed annually · Renews Dec 31, 2026</p>
              </div>
              <Button variant="outline">Upgrade to Enterprise</Button>
            </div>
          </div>
          <Card>
            <h3 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4">Plan Features</h3>
            <div className="grid grid-cols-2 gap-3">
              {[
                { feat: 'Active students', limit: '10,000', used: '1,247' },
                { feat: 'Storage', limit: '100 GB', used: '23.4 GB' },
                { feat: 'Courses', limit: 'Unlimited', used: '86' },
                { feat: 'API requests/day', limit: '100,000', used: '45,210' },
                { feat: 'Custom domain', limit: 'Yes', used: 'Active' },
                { feat: 'Camera proctoring', limit: 'Included', used: 'Active' },
              ].map(item => (
                <div key={item.feat} className="p-3 rounded-lg border border-sand-200 bg-surface-raised">
                  <div className="text-xs text-ink-tertiary">{item.feat}</div>
                  <div className="flex items-baseline gap-1.5 mt-1">
                    <span className="text-sm font-bold font-[family-name:var(--font-display)] text-ink">{item.used}</span>
                    <span className="text-[10px] text-ink-placeholder">/ {item.limit}</span>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </motion.div>
      )}
    </div>
  );
}
