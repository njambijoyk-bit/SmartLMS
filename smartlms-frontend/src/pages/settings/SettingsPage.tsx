import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  User, Bell, Shield, Palette, Globe, Key,
  Camera, Check, ChevronRight, Moon, Sun, Monitor,
  Smartphone, Mail,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../../context/AuthContext';

type SettingsTab = 'profile' | 'notifications' | 'security' | 'appearance' | 'language';

const TABS: { key: SettingsTab; label: string; icon: React.ReactNode }[] = [
  { key: 'profile', label: 'Profile', icon: <User size={17} /> },
  { key: 'notifications', label: 'Notifications', icon: <Bell size={17} /> },
  { key: 'security', label: 'Security', icon: <Shield size={17} /> },
  { key: 'appearance', label: 'Appearance', icon: <Palette size={17} /> },
  { key: 'language', label: 'Language & Region', icon: <Globe size={17} /> },
];

const fadeIn = { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

function Toggle({ value, onChange }: { value: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onChange(!value)}
      className={`relative w-10 h-5.5 rounded-full transition-colors cursor-pointer ${value ? 'bg-brand-500' : 'bg-sand-300'}`}
      style={{ height: '22px' }}
    >
      <span className={`absolute top-0.5 left-0.5 w-4.5 h-4.5 bg-white rounded-full shadow transition-transform ${value ? 'translate-x-[18px]' : 'translate-x-0'}`}
        style={{ width: '18px', height: '18px' }} />
    </button>
  );
}

export function SettingsPage() {
  const { user } = useAuth();
  const [tab, setTab] = useState<SettingsTab>('profile');
  const [theme, setTheme] = useState<'light' | 'dark' | 'system'>('light');
  const [notifs, setNotifs] = useState({
    emailAssessments: true,
    emailAnnouncements: true,
    emailMessages: false,
    pushAll: true,
    pushAssessments: true,
    pushLive: true,
    pushMessages: true,
    pushAchievements: true,
  });
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  return (
    <div className="space-y-5 max-w-4xl">
      <motion.div {...fadeIn} transition={{ duration: 0.4 }}>
        <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Settings</h1>
        <p className="text-sm text-ink-tertiary mt-1">Manage your account, preferences, and notifications</p>
      </motion.div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-5">
        {/* Sidebar */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.08 }}>
          <Card padding="sm">
            <nav className="space-y-0.5">
              {TABS.map(t => (
                <button key={t.key} onClick={() => setTab(t.key)}
                  className={`w-full flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors cursor-pointer text-left ${tab === t.key ? 'bg-brand-50 text-brand-600' : 'text-ink-secondary hover:bg-sand-100'}`}>
                  {t.icon}
                  {t.label}
                  <ChevronRight size={14} className="ml-auto text-ink-tertiary" />
                </button>
              ))}
            </nav>
          </Card>
        </motion.div>

        {/* Content */}
        <motion.div {...fadeIn} transition={{ duration: 0.4, delay: 0.12 }} className="md:col-span-3">

          {/* Profile */}
          {tab === 'profile' && (
            <Card>
              <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-5">Profile Information</h2>
              {/* Avatar */}
              <div className="flex items-center gap-4 mb-6 p-4 bg-sand-50 rounded-xl border border-sand-200">
                <div className="relative">
                  <div className="w-16 h-16 rounded-full bg-brand-500 flex items-center justify-center text-white text-2xl font-bold font-[family-name:var(--font-display)]">
                    {user?.name?.charAt(0) ?? 'U'}
                  </div>
                  <button className="absolute -bottom-1 -right-1 w-7 h-7 rounded-full bg-brand-500 border-2 border-white flex items-center justify-center cursor-pointer hover:bg-brand-600 transition-colors">
                    <Camera size={13} className="text-white" />
                  </button>
                </div>
                <div>
                  <div className="font-semibold text-ink">{user?.name}</div>
                  <div className="text-sm text-ink-tertiary capitalize">{user?.role}</div>
                  <button className="text-xs text-brand-500 hover:underline mt-1 cursor-pointer">Change photo</button>
                </div>
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                {[
                  { label: 'First Name', value: user?.name?.split(' ')[0] ?? '' },
                  { label: 'Last Name', value: user?.name?.split(' ').slice(1).join(' ') ?? '' },
                  { label: 'Email Address', value: user?.email ?? '', type: 'email' },
                  { label: 'Phone Number', value: '+254 712 345 678' },
                ].map(field => (
                  <div key={field.label}>
                    <label className="block text-xs font-semibold text-ink-secondary mb-1.5">{field.label}</label>
                    <input
                      type={field.type ?? 'text'}
                      defaultValue={field.value}
                      className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
                    />
                  </div>
                ))}
              </div>

              <div className="mt-4">
                <label className="block text-xs font-semibold text-ink-secondary mb-1.5">Bio</label>
                <textarea
                  rows={3}
                  defaultValue="Computer Science Year 3 student. Interested in algorithms and distributed systems."
                  className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400 resize-none"
                />
              </div>

              <div className="mt-5 flex justify-end">
                <Button onClick={handleSave}>
                  {saved ? <><Check size={14} /> Saved!</> : 'Save Changes'}
                </Button>
              </div>
            </Card>
          )}

          {/* Notifications */}
          {tab === 'notifications' && (
            <div className="space-y-4">
              <Card>
                <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4 flex items-center gap-2">
                  <Mail size={17} className="text-ink-tertiary" /> Email Notifications
                </h2>
                <div className="space-y-4">
                  {[
                    { key: 'emailAssessments', label: 'Assessment reminders', desc: 'CAT, assignment, and exam deadlines' },
                    { key: 'emailAnnouncements', label: 'Course announcements', desc: 'When instructors post new announcements' },
                    { key: 'emailMessages', label: 'New messages', desc: 'When you receive a direct message' },
                  ].map(item => (
                    <div key={item.key} className="flex items-center justify-between">
                      <div>
                        <div className="text-sm font-medium text-ink">{item.label}</div>
                        <div className="text-xs text-ink-tertiary">{item.desc}</div>
                      </div>
                      <Toggle value={notifs[item.key as keyof typeof notifs]} onChange={v => setNotifs(p => ({ ...p, [item.key]: v }))} />
                    </div>
                  ))}
                </div>
              </Card>

              <Card>
                <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4 flex items-center gap-2">
                  <Smartphone size={17} className="text-ink-tertiary" /> Push Notifications
                </h2>
                <div className="space-y-4">
                  {[
                    { key: 'pushAssessments', label: 'Assessment alerts', desc: 'Active CATs and approaching deadlines' },
                    { key: 'pushLive', label: 'Live class alerts', desc: 'When a live session starts' },
                    { key: 'pushMessages', label: 'Direct messages', desc: 'New messages from instructors and peers' },
                    { key: 'pushAchievements', label: 'Achievements', desc: 'Badges, XP milestones, leaderboard changes' },
                  ].map(item => (
                    <div key={item.key} className="flex items-center justify-between">
                      <div>
                        <div className="text-sm font-medium text-ink">{item.label}</div>
                        <div className="text-xs text-ink-tertiary">{item.desc}</div>
                      </div>
                      <Toggle value={notifs[item.key as keyof typeof notifs]} onChange={v => setNotifs(p => ({ ...p, [item.key]: v }))} />
                    </div>
                  ))}
                </div>
              </Card>
            </div>
          )}

          {/* Security */}
          {tab === 'security' && (
            <div className="space-y-4">
              <Card>
                <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-4 flex items-center gap-2">
                  <Key size={17} className="text-ink-tertiary" /> Change Password
                </h2>
                <div className="space-y-3">
                  {['Current Password', 'New Password', 'Confirm New Password'].map(f => (
                    <div key={f}>
                      <label className="block text-xs font-semibold text-ink-secondary mb-1.5">{f}</label>
                      <input type="password" placeholder="••••••••"
                        className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
                    </div>
                  ))}
                  <Button>Update Password</Button>
                </div>
              </Card>

              <Card>
                <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-1 flex items-center gap-2">
                  <Shield size={17} className="text-ink-tertiary" /> Two-Factor Authentication
                </h2>
                <p className="text-xs text-ink-tertiary mb-4">Add an extra layer of security to your account.</p>
                <div className="flex items-center justify-between p-3 bg-sand-50 rounded-xl border border-sand-200">
                  <div className="flex items-center gap-2 text-sm">
                    <Smartphone size={15} className="text-ink-tertiary" />
                    Authenticator App
                  </div>
                  <Button variant="outline" size="sm">Enable</Button>
                </div>
              </Card>

              <Card>
                <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-3">Active Sessions</h2>
                <div className="space-y-2">
                  {[
                    { device: 'Chrome on Windows', location: 'Nairobi, KE', time: 'Now', current: true },
                    { device: 'Safari on iPhone', location: 'Nairobi, KE', time: '2 hours ago', current: false },
                  ].map((session, i) => (
                    <div key={i} className="flex items-center justify-between p-3 bg-sand-50 rounded-xl border border-sand-200">
                      <div>
                        <div className="text-sm font-medium text-ink">{session.device}</div>
                        <div className="text-xs text-ink-tertiary">{session.location} · {session.time}</div>
                      </div>
                      {session.current ? (
                        <span className="text-xs text-success font-medium flex items-center gap-1"><Check size={11} /> Current</span>
                      ) : (
                        <button className="text-xs text-danger font-medium hover:underline cursor-pointer">Revoke</button>
                      )}
                    </div>
                  ))}
                </div>
              </Card>
            </div>
          )}

          {/* Appearance */}
          {tab === 'appearance' && (
            <Card>
              <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-5 flex items-center gap-2">
                <Palette size={17} className="text-ink-tertiary" /> Appearance
              </h2>
              <div className="space-y-5">
                <div>
                  <div className="text-sm font-semibold text-ink-secondary mb-3">Theme</div>
                  <div className="grid grid-cols-3 gap-3">
                    {[
                      { key: 'light', label: 'Light', icon: <Sun size={20} /> },
                      { key: 'dark', label: 'Dark', icon: <Moon size={20} /> },
                      { key: 'system', label: 'System', icon: <Monitor size={20} /> },
                    ].map(t => (
                      <button key={t.key} onClick={() => setTheme(t.key as typeof theme)}
                        className={`flex flex-col items-center gap-2 p-4 rounded-xl border-2 transition-all cursor-pointer ${theme === t.key ? 'border-brand-400 bg-brand-50' : 'border-sand-200 hover:border-brand-200'}`}>
                        <span className={theme === t.key ? 'text-brand-500' : 'text-ink-tertiary'}>{t.icon}</span>
                        <span className={`text-xs font-medium ${theme === t.key ? 'text-brand-600' : 'text-ink-secondary'}`}>{t.label}</span>
                        {theme === t.key && <div className="w-2 h-2 rounded-full bg-brand-500" />}
                      </button>
                    ))}
                  </div>
                </div>

                <div>
                  <div className="text-sm font-semibold text-ink-secondary mb-3">Sidebar style</div>
                  <div className="grid grid-cols-2 gap-3">
                    {['Compact', 'Expanded'].map(s => (
                      <button key={s} className="p-3 rounded-xl border-2 border-sand-200 hover:border-brand-200 text-sm text-ink-secondary transition-colors cursor-pointer">
                        {s}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
            </Card>
          )}

          {/* Language */}
          {tab === 'language' && (
            <Card>
              <h2 className="font-semibold font-[family-name:var(--font-display)] text-ink mb-5 flex items-center gap-2">
                <Globe size={17} className="text-ink-tertiary" /> Language & Region
              </h2>
              <div className="space-y-4">
                {[
                  { label: 'Language', options: ['English (US)', 'English (UK)', 'Kiswahili', 'French', 'Arabic', 'Spanish'], selected: 'English (US)' },
                  { label: 'Time Zone', options: ['Africa/Nairobi (EAT)', 'UTC', 'America/New_York (EST)', 'Europe/London (GMT)'], selected: 'Africa/Nairobi (EAT)' },
                  { label: 'Date Format', options: ['DD/MM/YYYY', 'MM/DD/YYYY', 'YYYY-MM-DD'], selected: 'DD/MM/YYYY' },
                ].map(field => (
                  <div key={field.label}>
                    <label className="block text-xs font-semibold text-ink-secondary mb-1.5">{field.label}</label>
                    <select defaultValue={field.selected} className="w-full bg-surface-raised border border-sand-300 rounded-xl px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 cursor-pointer">
                      {field.options.map(o => <option key={o}>{o}</option>)}
                    </select>
                  </div>
                ))}
                <div className="flex justify-end mt-2">
                  <Button onClick={handleSave}>{saved ? <><Check size={14} /> Saved!</> : 'Save Preferences'}</Button>
                </div>
              </div>
            </Card>
          )}
        </motion.div>
      </div>
    </div>
  );
}
