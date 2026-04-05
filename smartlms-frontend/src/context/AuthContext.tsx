import { createContext, useContext, useState, useCallback, type ReactNode } from 'react';
import type { User, UserRole } from '../types';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  login: (role: UserRole) => void;
  logout: () => void;
  switchRole: (role: UserRole) => void;
}

const DEMO_USERS: Record<UserRole, User> = {
  admin: { id: '1', name: 'Dr. Akinyi Odera', email: 'admin@uon.ac.ke', role: 'admin', institution: 'University of Nairobi' },
  instructor: { id: '2', name: 'Prof. James Mwangi', email: 'j.mwangi@uon.ac.ke', role: 'instructor', institution: 'University of Nairobi' },
  learner: { id: '3', name: 'Faith Wanjiku', email: 'f.wanjiku@students.uon.ac.ke', role: 'learner', institution: 'University of Nairobi' },
  parent: { id: '4', name: 'Peter Wanjiku', email: 'p.wanjiku@gmail.com', role: 'parent', institution: 'University of Nairobi' },
  advisor: { id: '5', name: 'Dr. Sarah Otieno', email: 's.otieno@uon.ac.ke', role: 'advisor', institution: 'University of Nairobi' },
  counsellor: { id: '6', name: 'Grace Kamau', email: 'g.kamau@uon.ac.ke', role: 'counsellor', institution: 'University of Nairobi' },
  alumni: { id: '7', name: 'Kevin Njoroge', email: 'k.njoroge@alumni.uon.ac.ke', role: 'alumni', institution: 'University of Nairobi' },
};

const AuthContext = createContext<AuthState | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);

  const login = useCallback((role: UserRole) => {
    setUser(DEMO_USERS[role]);
  }, []);

  const logout = useCallback(() => {
    setUser(null);
  }, []);

  const switchRole = useCallback((role: UserRole) => {
    setUser(DEMO_USERS[role]);
  }, []);

  return (
    <AuthContext.Provider value={{ user, isAuthenticated: !!user, login, logout, switchRole }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within AuthProvider');
  return ctx;
}
