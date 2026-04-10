import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { GraduationCap, Mail, Lock, ArrowRight, Globe } from 'lucide-react';
import { useAuth } from '../../context/AuthContext';
import { Button } from '../../components/ui/Button';
import { Input } from '../../components/ui/Input';
import type { UserRole } from '../../types';

export function LoginPage() {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  const handleLogin = (role: UserRole) => {
    setLoading(true);
    setTimeout(() => {
      login(role);
      navigate('/dashboard');
      setLoading(false);
    }, 600);
  };

  return (
    <div className="min-h-screen flex">
      {/* Left panel — branding */}
      <div className="hidden lg:flex lg:w-[55%] relative bg-brand-800 overflow-hidden">
        {/* Geometric pattern overlay */}
        <div className="absolute inset-0 opacity-[0.08]">
          <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
            <defs>
              <pattern id="geo" x="0" y="0" width="80" height="80" patternUnits="userSpaceOnUse">
                <path d="M0 0h40v40H0zM40 40h40v40H40z" fill="white" />
                <circle cx="40" cy="40" r="20" fill="none" stroke="white" strokeWidth="1" />
                <path d="M0 40L40 0M40 80L80 40" stroke="white" strokeWidth="0.5" />
              </pattern>
            </defs>
            <rect width="100%" height="100%" fill="url(#geo)" />
          </svg>
        </div>

        {/* Gradient orbs */}
        <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-brand-400/20 rounded-full blur-[100px]" />
        <div className="absolute bottom-1/4 right-1/4 w-72 h-72 bg-accent-400/15 rounded-full blur-[80px]" />
        <div className="absolute top-1/2 right-1/3 w-64 h-64 bg-gold-400/10 rounded-full blur-[60px]" />

        <div className="relative z-10 flex flex-col justify-between p-12 text-white">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <div className="w-10 h-10 rounded-xl bg-white/10 backdrop-blur flex items-center justify-center">
                <GraduationCap size={22} className="text-gold-300" />
              </div>
              <span className="text-lg font-bold font-[family-name:var(--font-display)] tracking-tight">SmartLMS</span>
            </div>
          </div>

          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8, delay: 0.2 }}
          >
            <h1 className="text-5xl font-bold font-[family-name:var(--font-display)] leading-[1.1] tracking-tight mb-6">
              The engine that<br />
              powers education<br />
              <span className="text-gold-300">worldwide.</span>
            </h1>
            <p className="text-lg text-brand-200 max-w-md leading-relaxed">
              A complete Learning Management System built once, packaged and deployed to institutions across the globe. Self-hosted. White-labelled. Yours.
            </p>
          </motion.div>

          <div className="flex items-center gap-6 text-sm text-brand-300">
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-success animate-pulse" />
              <span>37 modules</span>
            </div>
            <div className="flex items-center gap-2">
              <Globe size={14} />
              <span>Multi-tenant</span>
            </div>
            <div className="flex items-center gap-2">
              <Lock size={14} />
              <span>Self-hosted</span>
            </div>
          </div>
        </div>
      </div>

      {/* Right panel — login form */}
      <div className="flex-1 flex items-center justify-center p-8 bg-surface pattern-geo">
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.5 }}
          className="w-full max-w-md"
        >
          {/* Mobile logo */}
          <div className="lg:hidden flex items-center gap-2.5 mb-8">
            <div className="w-9 h-9 rounded-lg bg-brand-500 flex items-center justify-center">
              <GraduationCap size={20} className="text-white" />
            </div>
            <span className="font-bold text-lg font-[family-name:var(--font-display)]">SmartLMS</span>
          </div>

          <div className="mb-8">
            <h2 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">
              Sign in to your account
            </h2>
            <p className="text-sm text-ink-tertiary mt-1.5">
              University of Nairobi — LMS Portal
            </p>
          </div>

          <div className="space-y-4">
            <Input
              label="Email address"
              type="email"
              placeholder="you@institution.ac.ke"
              icon={<Mail size={16} />}
            />
            <Input
              label="Password"
              type="password"
              placeholder="Enter your password"
              icon={<Lock size={16} />}
            />

            <div className="flex items-center justify-between text-sm">
              <label className="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" className="rounded border-sand-300 text-brand-500 focus:ring-brand-300" />
                <span className="text-ink-secondary">Remember me</span>
              </label>
              <button className="text-brand-500 hover:text-brand-600 font-medium cursor-pointer">
                Forgot password?
              </button>
            </div>

            <Button size="lg" className="w-full" onClick={() => handleLogin('admin')} loading={loading}>
              Sign in
              <ArrowRight size={16} />
            </Button>

            <div className="relative my-6">
              <div className="absolute inset-0 flex items-center">
                <div className="w-full border-t border-sand-300" />
              </div>
              <div className="relative flex justify-center text-xs uppercase">
                <span className="bg-surface px-3 text-ink-tertiary font-medium">or continue with</span>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <button className="flex items-center justify-center gap-2 px-4 py-2.5 border border-sand-300 rounded-lg text-sm font-medium text-ink-secondary hover:border-brand-300 hover:text-ink transition-colors cursor-pointer">
                <svg width="18" height="18" viewBox="0 0 24 24"><path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92a5.06 5.06 0 01-2.2 3.32v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.1z" fill="#4285F4"/><path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/><path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/><path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/></svg>
                Google
              </button>
              <button className="flex items-center justify-center gap-2 px-4 py-2.5 border border-sand-300 rounded-lg text-sm font-medium text-ink-secondary hover:border-brand-300 hover:text-ink transition-colors cursor-pointer">
                <svg width="18" height="18" viewBox="0 0 23 23"><path fill="#f35325" d="M1 1h10v10H1z"/><path fill="#81bc06" d="M12 1h10v10H12z"/><path fill="#05a6f0" d="M1 12h10v10H1z"/><path fill="#ffba08" d="M12 12h10v10H12z"/></svg>
                Microsoft
              </button>
            </div>
          </div>

          {/* Demo role quick-access */}
          <div className="mt-8 pt-6 border-t border-sand-200">
            <p className="text-xs text-ink-tertiary mb-3 font-medium uppercase tracking-wider">Demo — Quick role access</p>
            <div className="flex gap-2">
              {(['admin', 'instructor', 'learner'] as UserRole[]).map(role => (
                <button
                  key={role}
                  onClick={() => handleLogin(role)}
                  className="flex-1 px-3 py-2 rounded-lg border border-dashed border-sand-300 text-xs font-medium text-ink-secondary hover:border-brand-400 hover:text-brand-600 hover:bg-brand-50 transition-all cursor-pointer capitalize"
                >
                  {role}
                </button>
              ))}
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  );
}
