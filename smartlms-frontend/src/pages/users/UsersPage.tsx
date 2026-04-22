import { useState } from 'react';
import { motion } from 'framer-motion';
import {
  Search, Plus, Download, Upload, MoreHorizontal,
  Shield, BookOpen, GraduationCap, Eye, UserCog, Users as UsersIcon,
} from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';

type Role = 'admin' | 'instructor' | 'learner' | 'parent' | 'advisor';

const USERS = [
  { id: '1', name: 'Dr. Akinyi Odera', email: 'admin@uon.ac.ke', role: 'admin' as Role, status: 'active', lastLogin: '2 min ago' },
  { id: '2', name: 'Prof. James Mwangi', email: 'j.mwangi@uon.ac.ke', role: 'instructor' as Role, status: 'active', lastLogin: '1h ago' },
  { id: '3', name: 'Dr. Achieng Odhiambo', email: 'a.odhiambo@uon.ac.ke', role: 'instructor' as Role, status: 'active', lastLogin: '3h ago' },
  { id: '4', name: 'Faith Wanjiku', email: 'f.wanjiku@students.uon.ac.ke', role: 'learner' as Role, status: 'active', lastLogin: '30 min ago' },
  { id: '5', name: 'Brian Otieno', email: 'b.otieno@students.uon.ac.ke', role: 'learner' as Role, status: 'suspended', lastLogin: '12 days ago' },
  { id: '6', name: 'Grace Nyambura', email: 'g.nyambura@students.uon.ac.ke', role: 'learner' as Role, status: 'active', lastLogin: '2h ago' },
  { id: '7', name: 'Peter Wanjiku', email: 'p.wanjiku@gmail.com', role: 'parent' as Role, status: 'active', lastLogin: '5h ago' },
  { id: '8', name: 'Dr. Sarah Otieno', email: 's.otieno@uon.ac.ke', role: 'advisor' as Role, status: 'active', lastLogin: '1d ago' },
  { id: '9', name: 'Patrick Wafula', email: 'p.wafula@students.uon.ac.ke', role: 'learner' as Role, status: 'active', lastLogin: '4h ago' },
  { id: '10', name: 'Amina Hassan', email: 'a.hassan@students.uon.ac.ke', role: 'learner' as Role, status: 'pending', lastLogin: 'Never' },
];

const ROLE_ICONS: Record<Role, React.ReactNode> = {
  admin: <Shield size={13} className="text-danger" />,
  instructor: <BookOpen size={13} className="text-brand-500" />,
  learner: <GraduationCap size={13} className="text-gold-500" />,
  parent: <Eye size={13} className="text-accent-400" />,
  advisor: <UserCog size={13} className="text-info" />,
};

const ROLE_BADGE: Record<Role, 'danger' | 'brand' | 'accent' | 'warning' | 'info'> = {
  admin: 'danger',
  instructor: 'brand',
  learner: 'accent',
  parent: 'warning',
  advisor: 'info',
};

export function UsersPage() {
  const [roleFilter, setRoleFilter] = useState<string>('all');
  const filtered = roleFilter === 'all' ? USERS : USERS.filter(u => u.role === roleFilter);

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">Users & Roles</h1>
            <p className="text-sm text-ink-tertiary mt-1">Manage institution users, roles, and permissions</p>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="secondary" size="sm"><Upload size={14} /> CSV Import</Button>
            <Button size="sm"><Plus size={14} /> Add User</Button>
          </div>
        </div>
      </motion.div>

      {/* Role stat cards */}
      <div className="grid grid-cols-2 sm:grid-cols-5 gap-3">
        {[
          { role: 'all', label: 'All Users', count: USERS.length, icon: <UsersIcon size={16} className="text-ink-tertiary" /> },
          { role: 'admin', label: 'Admins', count: USERS.filter(u => u.role === 'admin').length, icon: ROLE_ICONS.admin },
          { role: 'instructor', label: 'Instructors', count: USERS.filter(u => u.role === 'instructor').length, icon: ROLE_ICONS.instructor },
          { role: 'learner', label: 'Learners', count: USERS.filter(u => u.role === 'learner').length, icon: ROLE_ICONS.learner },
          { role: 'parent', label: 'Parents', count: USERS.filter(u => u.role === 'parent').length, icon: ROLE_ICONS.parent },
        ].map(stat => (
          <button
            key={stat.role}
            onClick={() => setRoleFilter(stat.role)}
            className={`p-3 rounded-xl border text-left transition-all cursor-pointer ${
              roleFilter === stat.role
                ? 'border-brand-300 bg-brand-50 shadow-sm'
                : 'border-sand-200 bg-surface-raised hover:border-brand-200'
            }`}
          >
            <div className="flex items-center gap-1.5 mb-1">{stat.icon}<span className="text-xs text-ink-tertiary">{stat.label}</span></div>
            <div className="text-lg font-bold font-[family-name:var(--font-display)] text-ink">{stat.count}</div>
          </button>
        ))}
      </div>

      {/* Search and filter */}
      <div className="flex items-center gap-3">
        <div className="relative flex-1 max-w-sm">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
          <input type="text" placeholder="Search users..." className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300" />
        </div>
        <Button variant="secondary" size="sm"><Download size={14} /> Export</Button>
      </div>

      {/* Users table */}
      <Card padding="none">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-sand-200 bg-sand-50">
                <th className="text-left px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">User</th>
                <th className="text-left px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Role</th>
                <th className="text-left px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Status</th>
                <th className="text-left px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Last Login</th>
                <th className="text-right px-4 py-3 text-xs font-semibold text-ink-tertiary uppercase tracking-wider font-[family-name:var(--font-display)]">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((u) => (
                <tr key={u.id} className="border-b border-sand-100 hover:bg-sand-50 transition-colors">
                  <td className="px-4 py-3">
                    <div className="flex items-center gap-3">
                      <Avatar name={u.name} size="md" />
                      <div>
                        <div className="text-sm font-medium text-ink">{u.name}</div>
                        <div className="text-xs text-ink-tertiary">{u.email}</div>
                      </div>
                    </div>
                  </td>
                  <td className="px-4 py-3">
                    <Badge variant={ROLE_BADGE[u.role]} size="md">
                      <span className="flex items-center gap-1">{ROLE_ICONS[u.role]} {u.role}</span>
                    </Badge>
                  </td>
                  <td className="px-4 py-3">
                    <Badge
                      variant={u.status === 'active' ? 'success' : u.status === 'suspended' ? 'danger' : 'warning'}
                      size="sm"
                    >
                      {u.status}
                    </Badge>
                  </td>
                  <td className="px-4 py-3 text-sm text-ink-tertiary">{u.lastLogin}</td>
                  <td className="px-4 py-3 text-right">
                    <button className="p-1.5 rounded-lg hover:bg-sand-100 text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
                      <MoreHorizontal size={16} />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
