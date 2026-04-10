import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
  GraduationCap, Building2, Palette, Users, CheckCircle2,
  ArrowRight, ArrowLeft, Globe, Upload, Sparkles,
} from 'lucide-react';
import { Button } from '../../components/ui/Button';
import { Input } from '../../components/ui/Input';
import { Card } from '../../components/ui/Card';
import { useAuth } from '../../context/AuthContext';

const STEPS = [
  { label: 'Institution', icon: Building2 },
  { label: 'Branding', icon: Palette },
  { label: 'Admin', icon: Users },
  { label: 'Complete', icon: CheckCircle2 },
];

export function SetupWizard() {
  const [step, setStep] = useState(0);
  const [plan, setPlan] = useState<'starter' | 'growth' | 'enterprise'>('growth');
  const { login } = useAuth();
  const navigate = useNavigate();

  const finish = () => {
    login('admin');
    navigate('/dashboard');
  };

  return (
    <div className="min-h-screen bg-surface pattern-geo flex flex-col">
      {/* Top bar */}
      <div className="border-pattern w-full" />
      <div className="flex items-center justify-between px-8 py-4 bg-surface-raised border-b border-sand-200">
        <div className="flex items-center gap-2.5">
          <div className="w-8 h-8 rounded-lg bg-brand-500 flex items-center justify-center">
            <GraduationCap size={18} className="text-white" />
          </div>
          <span className="font-bold text-sm font-[family-name:var(--font-display)]">SmartLMS Setup</span>
        </div>
        <div className="text-xs text-ink-tertiary">Step {step + 1} of {STEPS.length}</div>
      </div>

      <div className="flex-1 flex items-center justify-center p-8">
        <div className="w-full max-w-2xl">
          {/* Step indicators */}
          <div className="flex items-center justify-center gap-2 mb-8">
            {STEPS.map((s, i) => {
              const Icon = s.icon;
              return (
                <div key={i} className="flex items-center gap-2">
                  <div className={`w-9 h-9 rounded-full flex items-center justify-center transition-colors ${
                    i < step ? 'bg-success text-white' :
                    i === step ? 'bg-brand-500 text-white' :
                    'bg-sand-200 text-ink-tertiary'
                  }`}>
                    {i < step ? <CheckCircle2 size={16} /> : <Icon size={16} />}
                  </div>
                  {i < STEPS.length - 1 && (
                    <div className={`w-12 h-0.5 ${i < step ? 'bg-success' : 'bg-sand-200'}`} />
                  )}
                </div>
              );
            })}
          </div>

          <AnimatePresence mode="wait">
            <motion.div
              key={step}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3 }}
            >
              {step === 0 && (
                <Card padding="lg">
                  <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-1">
                    Set up your institution
                  </h2>
                  <p className="text-sm text-ink-tertiary mb-6">
                    Tell us about your institution so we can configure the engine.
                  </p>
                  <div className="space-y-4">
                    <Input label="Institution Name" placeholder="e.g. University of Nairobi" />
                    <Input label="Short Name / Slug" placeholder="e.g. uon" />
                    <div className="grid grid-cols-2 gap-4">
                      <Input label="Country" placeholder="e.g. Kenya" icon={<Globe size={14} />} />
                      <Input label="Institution Type" placeholder="e.g. University" />
                    </div>
                    <Input label="Custom Domain (optional)" placeholder="e.g. lms.uon.ac.ke" icon={<Globe size={14} />} />

                    <div>
                      <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)] mb-2">
                        Select Plan
                      </label>
                      <div className="grid grid-cols-3 gap-3">
                        {[
                          { id: 'starter' as const, name: 'Starter', desc: 'One-time licence', price: 'From $299' },
                          { id: 'growth' as const, name: 'Growth', desc: 'Monthly subscription', price: 'From $49/mo' },
                          { id: 'enterprise' as const, name: 'Enterprise', desc: 'Annual contract', price: 'Custom' },
                        ].map(p => (
                          <button
                            key={p.id}
                            onClick={() => setPlan(p.id)}
                            className={`p-3 rounded-lg border text-left transition-all cursor-pointer ${
                              plan === p.id
                                ? 'border-brand-400 bg-brand-50 ring-2 ring-brand-200'
                                : 'border-sand-300 hover:border-brand-200'
                            }`}
                          >
                            <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{p.name}</div>
                            <div className="text-xs text-ink-tertiary">{p.desc}</div>
                            <div className="text-sm font-bold text-brand-600 mt-1 font-[family-name:var(--font-display)]">{p.price}</div>
                          </button>
                        ))}
                      </div>
                    </div>
                  </div>
                </Card>
              )}

              {step === 1 && (
                <Card padding="lg">
                  <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-1">
                    Brand your LMS
                  </h2>
                  <p className="text-sm text-ink-tertiary mb-6">
                    Customize the look and feel. Students will see your brand, not ours.
                  </p>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)] mb-2">Institution Logo</label>
                      <div className="border-2 border-dashed border-sand-300 rounded-lg p-8 text-center hover:border-brand-300 transition-colors cursor-pointer">
                        <Upload size={24} className="mx-auto text-ink-tertiary mb-2" />
                        <p className="text-sm text-ink-secondary">Drop your logo here or click to browse</p>
                        <p className="text-xs text-ink-tertiary mt-1">PNG, SVG — Max 2MB</p>
                      </div>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)] mb-2">Primary Color</label>
                        <div className="flex items-center gap-2">
                          <div className="w-10 h-10 rounded-lg bg-brand-500 border border-sand-200 cursor-pointer" />
                          <Input placeholder="#0D5E6D" />
                        </div>
                      </div>
                      <div>
                        <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)] mb-2">Accent Color</label>
                        <div className="flex items-center gap-2">
                          <div className="w-10 h-10 rounded-lg bg-accent-400 border border-sand-200 cursor-pointer" />
                          <Input placeholder="#C75C2B" />
                        </div>
                      </div>
                    </div>
                    <Input label="Tagline (optional)" placeholder="e.g. Empowering minds, transforming futures" />

                    <div className="bg-sand-100 rounded-lg p-4 mt-4">
                      <div className="flex items-center gap-2 mb-2">
                        <Sparkles size={14} className="text-gold-500" />
                        <span className="text-xs font-semibold text-ink-secondary font-[family-name:var(--font-display)]">Preview</span>
                      </div>
                      <div className="bg-surface-raised rounded-lg p-3 border border-sand-200">
                        <div className="flex items-center gap-2 mb-2">
                          <div className="w-6 h-6 rounded bg-brand-500" />
                          <span className="text-sm font-bold font-[family-name:var(--font-display)]">University of Nairobi</span>
                        </div>
                        <div className="h-2 bg-brand-100 rounded w-3/4 mb-1.5" />
                        <div className="h-2 bg-sand-200 rounded w-1/2" />
                      </div>
                    </div>
                  </div>
                </Card>
              )}

              {step === 2 && (
                <Card padding="lg">
                  <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-1">
                    Create admin account
                  </h2>
                  <p className="text-sm text-ink-tertiary mb-6">
                    This will be the institution's primary administrator.
                  </p>
                  <div className="space-y-4">
                    <div className="grid grid-cols-2 gap-4">
                      <Input label="First Name" placeholder="Akinyi" />
                      <Input label="Last Name" placeholder="Odera" />
                    </div>
                    <Input label="Email" type="email" placeholder="admin@uon.ac.ke" />
                    <Input label="Password" type="password" placeholder="Create a strong password" />
                    <Input label="Confirm Password" type="password" placeholder="Confirm password" />
                    <label className="flex items-start gap-2 cursor-pointer mt-2">
                      <input type="checkbox" className="rounded border-sand-300 text-brand-500 mt-0.5" />
                      <span className="text-xs text-ink-secondary">Enable multi-factor authentication (recommended for admin accounts)</span>
                    </label>
                  </div>
                </Card>
              )}

              {step === 3 && (
                <Card padding="lg" className="text-center">
                  <div className="w-16 h-16 rounded-full bg-success-light flex items-center justify-center mx-auto mb-4">
                    <CheckCircle2 size={32} className="text-success" />
                  </div>
                  <h2 className="text-xl font-bold font-[family-name:var(--font-display)] text-ink mb-2">
                    Your SmartLMS engine is ready
                  </h2>
                  <p className="text-sm text-ink-tertiary mb-6 max-w-md mx-auto">
                    Your institution has been configured. The engine will now provision your database, apply your branding, and set up your admin account.
                  </p>
                  <div className="grid grid-cols-3 gap-4 text-left max-w-md mx-auto mb-6">
                    {[
                      { label: 'Database', status: 'Provisioned' },
                      { label: 'Branding', status: 'Applied' },
                      { label: 'Admin Account', status: 'Created' },
                    ].map(item => (
                      <div key={item.label} className="flex items-center gap-2">
                        <CheckCircle2 size={14} className="text-success" />
                        <div>
                          <div className="text-xs font-medium text-ink">{item.label}</div>
                          <div className="text-[10px] text-ink-tertiary">{item.status}</div>
                        </div>
                      </div>
                    ))}
                  </div>
                </Card>
              )}
            </motion.div>
          </AnimatePresence>

          {/* Nav buttons */}
          <div className="flex items-center justify-between mt-6">
            <Button
              variant="ghost"
              onClick={() => setStep(Math.max(0, step - 1))}
              disabled={step === 0}
            >
              <ArrowLeft size={16} /> Back
            </Button>
            {step < STEPS.length - 1 ? (
              <Button onClick={() => setStep(step + 1)}>
                Continue <ArrowRight size={16} />
              </Button>
            ) : (
              <Button onClick={finish} variant="accent">
                Launch Dashboard <ArrowRight size={16} />
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
