export type UserRole = 'admin' | 'instructor' | 'learner' | 'parent' | 'advisor' | 'counsellor' | 'alumni';
export type PlanTier = 'starter' | 'growth' | 'enterprise';

export interface User {
  id: string;
  name: string;
  email: string;
  role: UserRole;
  avatar?: string;
  institution?: string;
}

export interface Institution {
  id: string;
  slug: string;
  name: string;
  logo?: string;
  domain?: string;
  plan: PlanTier;
  primaryColor?: string;
  accentColor?: string;
}

export interface Course {
  id: string;
  title: string;
  code: string;
  description: string;
  instructor: string;
  thumbnail?: string;
  progress?: number;
  enrolledCount: number;
  status: 'draft' | 'published' | 'archived';
  category: string;
  units: number;
}

export interface Assessment {
  id: string;
  title: string;
  courseId: string;
  courseName: string;
  type: 'cat' | 'assignment' | 'exam';
  dueDate: string;
  totalMarks: number;
  status: 'upcoming' | 'in_progress' | 'submitted' | 'graded';
}

export interface GradeEntry {
  studentId: string;
  studentName: string;
  avatar?: string;
  cat1: number | null;
  cat2: number | null;
  assignment: number | null;
  exam: number | null;
  total: number | null;
  grade: string;
}

export interface Notification {
  id: string;
  title: string;
  message: string;
  type: 'info' | 'warning' | 'success' | 'danger';
  time: string;
  read: boolean;
}

export interface StatData {
  label: string;
  value: string | number;
  change?: number;
  trend?: 'up' | 'down' | 'flat';
  icon?: string;
}

// Export Phase 16 & 17 types
export * from './developer';
