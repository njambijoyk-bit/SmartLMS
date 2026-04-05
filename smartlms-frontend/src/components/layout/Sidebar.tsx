import { NavLink, useLocation } from 'react-router-dom';
import {
  LayoutDashboard, BookOpen, ClipboardCheck, Users, MessageSquare, BarChart3,
  Calendar, Settings, GraduationCap, FileText, CreditCard, Shield,
  Video, Bell, ChevronLeft, ChevronRight, Zap, Library, Award,
  UserCheck, Heart, Building2,
} from 'lucide-react';
import { useState } from 'react';
import clsx from 'clsx';
import { useAuth } from '../../context/AuthContext';
import type { UserRole } from '../../types';

interface NavItem {
  label: string;
  path: string;
  icon: React.ReactNode;
  roles: UserRole[];
  badge?: string;
}

const NAV_ITEMS: NavItem[] = [
  { label: 'Dashboard', path: '/dashboard', icon: <LayoutDashboard size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Courses', path: '/courses', icon: <BookOpen size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Assessments', path: '/assessments', icon: <ClipboardCheck size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Gradebook', path: '/gradebook', icon: <FileText size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Live Classes', path: '/live', icon: <Video size={20} />, roles: ['instructor', 'learner'], badge: 'Live' },
  { label: 'Messages', path: '/messages', icon: <MessageSquare size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Users & Roles', path: '/users', icon: <Users size={20} />, roles: ['admin'] },
  { label: 'Registration', path: '/registration', icon: <UserCheck size={20} />, roles: ['admin'] },
  { label: 'Attendance', path: '/attendance', icon: <Calendar size={20} />, roles: ['admin', 'instructor'] },
  { label: 'Fee Management', path: '/fees', icon: <CreditCard size={20} />, roles: ['admin'] },
  { label: 'Exam Cards', path: '/exam-cards', icon: <Shield size={20} />, roles: ['admin', 'learner'] },
  { label: 'Library', path: '/library', icon: <Library size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Timetable', path: '/timetable', icon: <Calendar size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Analytics', path: '/analytics', icon: <BarChart3 size={20} />, roles: ['admin', 'instructor'] },
  { label: 'Certificates', path: '/certificates', icon: <Award size={20} />, roles: ['admin', 'learner'] },
  { label: 'Wellbeing', path: '/wellbeing', icon: <Heart size={20} />, roles: ['admin', 'learner', 'counsellor'] },
  { label: 'Automation', path: '/automation', icon: <Zap size={20} />, roles: ['admin'] },
  { label: 'Institution', path: '/institution', icon: <Building2 size={20} />, roles: ['admin'] },
  { label: 'Notifications', path: '/notifications', icon: <Bell size={20} />, roles: ['admin', 'instructor', 'learner'] },
  { label: 'Settings', path: '/settings', icon: <Settings size={20} />, roles: ['admin', 'instructor', 'learner'] },
];

export function Sidebar() {
  const [collapsed, setCollapsed] = useState(false);
  const { user } = useAuth();
  const location = useLocation();

  const visibleItems = NAV_ITEMS.filter(item => user && item.roles.includes(user.role));

  return (
    <aside
      className={clsx(
        'h-screen sticky top-0 flex flex-col border-r border-sand-200 bg-surface-raised transition-all duration-300 shrink-0',
        collapsed ? 'w-[68px]' : 'w-[250px]'
      )}
    >
      {/* Logo */}
      <div className="h-16 flex items-center px-4 border-b border-sand-200 shrink-0">
        <div className="flex items-center gap-2.5 overflow-hidden">
          <div className="w-8 h-8 rounded-lg bg-brand-500 flex items-center justify-center shrink-0">
            <GraduationCap size={18} className="text-white" />
          </div>
          {!collapsed && (
            <div className="flex flex-col">
              <span className="font-bold text-sm font-[family-name:var(--font-display)] text-ink tracking-tight">SmartLMS</span>
              <span className="text-[10px] text-ink-tertiary leading-none">Engine v1.0</span>
            </div>
          )}
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-3 px-2.5 space-y-0.5">
        {visibleItems.map(item => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) => clsx(
              'flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-sm font-medium transition-colors duration-150 relative group',
              isActive
                ? 'bg-brand-50 text-brand-600'
                : 'text-ink-secondary hover:bg-sand-100 hover:text-ink',
              collapsed && 'justify-center px-2'
            )}
          >
            <span className="shrink-0">{item.icon}</span>
            {!collapsed && (
              <>
                <span className="truncate">{item.label}</span>
                {item.badge && (
                  <span className="ml-auto px-1.5 py-0.5 rounded-full text-[10px] font-semibold bg-danger text-white">
                    {item.badge}
                  </span>
                )}
              </>
            )}
            {collapsed && (
              <div className="absolute left-full ml-2 px-2 py-1 bg-ink text-ink-inverse text-xs rounded-md whitespace-nowrap opacity-0 pointer-events-none group-hover:opacity-100 transition-opacity z-50">
                {item.label}
              </div>
            )}
          </NavLink>
        ))}
      </nav>

      {/* Collapse toggle */}
      <div className="p-2.5 border-t border-sand-200">
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="w-full flex items-center justify-center p-2 rounded-lg text-ink-tertiary hover:bg-sand-100 hover:text-ink transition-colors cursor-pointer"
        >
          {collapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
        </button>
      </div>
    </aside>
  );
}
