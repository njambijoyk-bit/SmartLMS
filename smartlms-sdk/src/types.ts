/**
 * SmartLMS SDK — Core Domain Types
 */

export interface Course {
  id: string;
  title: string;
  code?: string;
  description?: string;
  level?: string;
  status?: 'draft' | 'published' | 'archived';
  category?: string;
  instructorId?: string;
  institutionId?: string;
  startDate?: string;
  endDate?: string;
  credits?: number;
  maxEnrollments?: number;
  createdAt?: string;
  updatedAt?: string;
}

export interface User {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  role?: 'student' | 'instructor' | 'admin' | 'staff';
  institutionId?: string;
  studentNumber?: string;
  department?: string;
  active?: boolean;
  createdAt?: string;
  updatedAt?: string;
}

export interface Enrollment {
  id: string;
  userId: string;
  courseId: string;
  status?: 'active' | 'completed' | 'withdrawn' | 'deferred';
  enrolledAt?: string;
  completedAt?: string;
  grade?: string;
}

export interface Assignment {
  id: string;
  courseId: string;
  title: string;
  description?: string;
  type?: 'essay' | 'project' | 'practical' | 'presentation';
  dueDate?: string;
  maxScore?: number;
  weight?: number;
  status?: 'draft' | 'published' | 'closed';
  createdAt?: string;
}

export interface Quiz {
  id: string;
  courseId: string;
  title: string;
  description?: string;
  questionCount?: number;
  durationMinutes?: number;
  passMark?: number;
  maxAttempts?: number;
  status?: 'draft' | 'published' | 'closed';
  createdAt?: string;
}

export interface Grade {
  id: string;
  userId: string;
  courseId: string;
  score?: number;
  letter?: string;
  percentage?: number;
  assessmentId?: string;
  assessmentType?: string;
  gradedAt?: string;
  gradedBy?: string;
}

export interface AttendanceRecord {
  id: string;
  userId: string;
  courseId: string;
  sessionId?: string;
  date: string;
  status: 'present' | 'absent' | 'late' | 'excused';
  markedAt?: string;
  markedBy?: string;
}

export interface Announcement {
  id: string;
  courseId?: string;
  institutionId?: string;
  title: string;
  body: string;
  authorId?: string;
  priority?: 'normal' | 'high' | 'urgent';
  publishedAt?: string;
  expiresAt?: string;
  createdAt?: string;
}
