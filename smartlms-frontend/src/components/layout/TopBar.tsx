import { Bell, Search, ChevronDown, LogOut } from 'lucide-react';
import { useState } from 'react';
import { useAuth } from '../../context/AuthContext';
import { Avatar } from '../ui/Avatar';
import { Badge } from '../ui/Badge';
import type { UserRole } from '../../types';

const ROLE_LABELS: Record<UserRole, string> = {
  admin: 'Administrator',
  instructor: 'Instructor',
  learner: 'Learner',
  parent: 'Parent',
  advisor: 'Academic Advisor',
  counsellor: 'Counsellor',
  alumni: 'Alumni',
};

export function TopBar() {
  const { user, logout, switchRole } = useAuth();
  const [showMenu, setShowMenu] = useState(false);
  const [showRoleSwitcher, setShowRoleSwitcher] = useState(false);

  if (!user) return null;

  return (
    <header className="h-16 border-b border-sand-200 bg-surface-raised/80 backdrop-blur-sm flex items-center justify-between px-6 sticky top-0 z-30">
      {/* Search */}
      <div className="relative w-full max-w-md">
        <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
        <input
          type="text"
          placeholder="Search courses, students, assessments..."
          className="w-full bg-sand-100 border-none rounded-lg pl-9 pr-4 py-2 text-sm text-ink placeholder:text-ink-placeholder focus:outline-none focus:ring-2 focus:ring-brand-300"
        />
        <kbd className="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] text-ink-tertiary bg-surface-raised border border-sand-300 px-1.5 py-0.5 rounded font-mono">
          /
        </kbd>
      </div>

      <div className="flex items-center gap-3 ml-4">
        {/* Role switcher (demo) */}
        <div className="relative">
          <button
            onClick={() => setShowRoleSwitcher(!showRoleSwitcher)}
            className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-medium border border-sand-300 text-ink-secondary hover:border-brand-300 hover:text-brand-600 transition-colors cursor-pointer"
          >
            <Badge variant="brand" size="sm">{ROLE_LABELS[user.role]}</Badge>
            <ChevronDown size={12} />
          </button>
          {showRoleSwitcher && (
            <>
              <div className="fixed inset-0 z-40" onClick={() => setShowRoleSwitcher(false)} />
              <div className="absolute right-0 mt-1 w-44 bg-surface-raised border border-sand-200 rounded-lg shadow-lg py-1 z-50">
                <div className="px-3 py-1.5 text-[10px] font-semibold text-ink-tertiary uppercase tracking-wider">Switch role (demo)</div>
                {(['admin', 'instructor', 'learner'] as UserRole[]).map(role => (
                  <button
                    key={role}
                    onClick={() => { switchRole(role); setShowRoleSwitcher(false); }}
                    className="w-full text-left px-3 py-2 text-sm text-ink-secondary hover:bg-sand-100 hover:text-ink transition-colors cursor-pointer"
                  >
                    {ROLE_LABELS[role]}
                  </button>
                ))}
              </div>
            </>
          )}
        </div>

        {/* Notifications */}
        <button className="relative p-2 rounded-lg text-ink-tertiary hover:bg-sand-100 hover:text-ink transition-colors cursor-pointer">
          <Bell size={20} />
          <span className="absolute top-1.5 right-1.5 w-2 h-2 bg-danger rounded-full" />
        </button>

        {/* User menu */}
        <div className="relative">
          <button
            onClick={() => setShowMenu(!showMenu)}
            className="flex items-center gap-2 cursor-pointer"
          >
            <Avatar name={user.name} size="sm" />
            <ChevronDown size={14} className="text-ink-tertiary" />
          </button>
          {showMenu && (
            <>
              <div className="fixed inset-0 z-40" onClick={() => setShowMenu(false)} />
              <div className="absolute right-0 mt-2 w-56 bg-surface-raised border border-sand-200 rounded-lg shadow-lg py-1 z-50">
                <div className="px-4 py-2.5 border-b border-sand-200">
                  <div className="text-sm font-semibold text-ink font-[family-name:var(--font-display)]">{user.name}</div>
                  <div className="text-xs text-ink-tertiary">{user.email}</div>
                </div>
                <button
                  onClick={() => { logout(); setShowMenu(false); }}
                  className="w-full flex items-center gap-2 px-4 py-2.5 text-sm text-danger hover:bg-danger-light transition-colors cursor-pointer"
                >
                  <LogOut size={16} />
                  Sign out
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </header>
  );
}
