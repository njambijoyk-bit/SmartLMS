/**
 * SmartLMS TypeScript SDK
 * JavaScript/TypeScript client for SmartLMS API
 * 
 * @example
 * ```typescript
 * import { SmartLMS } from '@smartlms/sdk';
 * 
 * const client = new SmartLMS({
 *   baseUrl: 'https://lms.your-institution.com',
 *   apiKey: 'your-api-key'
 * });
 * 
 * // Get courses
 * const courses = await client.courses.list();
 * ```
 */

import type {
  Course,
  User,
  Enrollment,
  Assignment,
  Quiz,
  Grade,
  AttendanceRecord,
  Announcement,
} from './types';

// ============================================================================
// CONFIGURATION
// ============================================================================

export interface SmartLMSConfig {
  /** Base URL of the SmartLMS instance */
  baseUrl: string;
  /** API key or JWT token */
  apiKey?: string;
  /** Custom fetch function */
  fetch?: typeof fetch;
  /** Request timeout in ms */
  timeout?: number;
}

export interface ApiResponse<T> {
  data: T;
  meta?: {
    total: number;
    page: number;
    perPage: number;
  };
}

export interface ApiError {
  error: {
    code: string;
    message: string;
    details?: Record<string, unknown>;
  };
}

// ============================================================================
// MAIN CLIENT
// ============================================================================

export class SmartLMS {
  private config: Required<SmartLMSConfig>;
  
  public courses: CoursesClient;
  public users: UsersClient;
  public enrollments: EnrollmentsClient;
  public assignments: AssignmentsClient;
  public quizzes: QuizzesClient;
  public grades: GradesClient;
  public attendance: AttendanceClient;
  public announcements: AnnouncementsClient;
  public analytics: AnalyticsClient;

  constructor(config: SmartLMSConfig) {
    this.config = {
      baseUrl: config.baseUrl.replace(/\/$/, ''),
      apiKey: config.apiKey ?? '',
      fetch: config.fetch ?? fetch,
      timeout: config.timeout ?? 30000,
    };

    // Initialize sub-clients
    this.courses = new CoursesClient(this);
    this.users = new UsersClient(this);
    this.enrollments = new EnrollmentsClient(this);
    this.assignments = new AssignmentsClient(this);
    this.quizzes = new QuizzesClient(this);
    this.grades = new GradesClient(this);
    this.attendance = new AttendanceClient(this);
    this.announcements = new AnnouncementsClient(this);
    this.analytics = new AnalyticsClient(this);
  }

  /**
   * Set authentication token
   */
  setAuth(token: string): void {
    this.config.apiKey = token;
  }

  /**
   * Make authenticated request
   */
  async request<T>(
    method: string,
    path: string,
    body?: unknown,
    params?: Record<string, string>
  ): Promise<T> {
    const url = new URL(`${this.config.baseUrl}/api/v1${path}`);
    
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        url.searchParams.append(key, value);
      });
    }

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    };

    if (this.config.apiKey) {
      headers['Authorization'] = `Bearer ${this.config.apiKey}`;
    }

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.config.timeout);

    try {
      const response = await this.config.fetch(url.toString(), {
        method,
        headers,
        body: body ? JSON.stringify(body) : undefined,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        const error: ApiError = await response.json();
        throw new SmartLMSError(error.error.message, error.error.code, response.status);
      }

      return response.json();
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof SmartLMSError) throw error;
      throw new SmartLMSError('Network error', 'NETWORK_ERROR', 0);
    }
  }
}

// ============================================================================
// BASE CLIENT
// ============================================================================

abstract class BaseClient {
  constructor(protected client: SmartLMS) {}

  protected async get<T>(path: string, params?: Record<string, string>): Promise<T> {
    return this.client.request<T>('GET', path, undefined, params);
  }

  protected async post<T>(path: string, body?: unknown): Promise<T> {
    return this.client.request<T>('POST', path, body);
  }

  protected async put<T>(path: string, body?: unknown): Promise<T> {
    return this.client.request<T>('PUT', path, body);
  }

  protected async delete<T>(path: string): Promise<T> {
    return this.client.request<T>('DELETE', path);
  }
}

// ============================================================================
// COURSES CLIENT
// ============================================================================

export class CoursesClient extends BaseClient {
  /**
   * List courses
   */
  async list(params?: {
    page?: number;
    perPage?: number;
    search?: string;
    category?: string;
  }): Promise<ApiResponse<Course[]>> {
    return this.get<ApiResponse<Course[]>>('/courses', params as Record<string, string>);
  }

  /**
   * Get course by ID
   */
  async get(courseId: string): Promise<Course> {
    return this.get<Course>(`/courses/${courseId}`);
  }

  /**
   * Create course
   */
  async create(data: Partial<Course>): Promise<Course> {
    return this.post<Course>('/courses', data);
  }

  /**
   * Update course
   */
  async update(courseId: string, data: Partial<Course>): Promise<Course> {
    return this.put<Course>(`/courses/${courseId}`, data);
  }

  /**
   * Delete course
   */
  async delete(courseId: string): Promise<void> {
    return this.delete<void>(`/courses/${courseId}`);
  }

  /**
   * Get course modules
   */
  async getModules(courseId: string): Promise<unknown> {
    return this.get<unknown>(`/courses/${courseId}/modules`);
  }

  /**
   * Publish course
   */
  async publish(courseId: string): Promise<Course> {
    return this.post<Course>(`/courses/${courseId}/publish`);
  }
}

// ============================================================================
// USERS CLIENT
// ============================================================================

export class UsersClient extends BaseClient {
  /**
   * List users
   */
  async list(params?: {
    page?: number;
    perPage?: number;
    role?: string;
    search?: string;
  }): Promise<ApiResponse<User[]>> {
    return this.get<ApiResponse<User[]>>('/users', params as Record<string, string>);
  }

  /**
   * Get user by ID
   */
  async get(userId: string): Promise<User> {
    return this.get<User>(`/users/${userId}`);
  }

  /**
   * Create user
   */
  async create(data: Partial<User>): Promise<User> {
    return this.post<User>('/users', data);
  }

  /**
   * Update user
   */
  async update(userId: string, data: Partial<User>): Promise<User> {
    return this.put<User>(`/users/${userId}`, data);
  }

  /**
   * Delete user
   */
  async delete(userId: string): Promise<void> {
    return this.delete<void>(`/users/${userId}`);
  }

  /**
   * Bulk import users from CSV
   */
  async bulkImport(file: File): Promise<{ imported: number; errors: string[] }> {
    const formData = new FormData();
    formData.append('file', file);
    
    // Use fetch directly for file upload
    const response = await this.client.config.fetch(`${this.client.config.baseUrl}/api/v1/users/import`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${this.client.config.apiKey}`,
      },
      body: formData,
    });
    
    return response.json();
  }
}

// ============================================================================
// ENROLLMENTS CLIENT
// ============================================================================

export class EnrollmentsClient extends BaseClient {
  /**
   * List enrollments
   */
  async list(params?: {
    courseId?: string;
    userId?: string;
    status?: string;
  }): Promise<ApiResponse<Enrollment[]>> {
    return this.get<ApiResponse<Enrollment[]>>('/enrollments', params as Record<string, string>);
  }

  /**
   * Enroll user in course
   */
  async enroll(data: { userId: string; courseId: string }): Promise<Enrollment> {
    return this.post<Enrollment>('/enrollments', data);
  }

  /**
   * Unenroll user
   */
  async unenroll(enrollmentId: string): Promise<void> {
    return this.delete<void>(`/enrollments/${enrollmentId}`);
  }

  /**
   * Bulk enroll
   */
  async bulkEnroll(courseId: string, userIds: string[]): Promise<{ enrolled: number }> {
    return this.post<{ enrolled: number }>('/enrollments/bulk', { courseId, userIds });
  }
}

// ============================================================================
// ASSIGNMENTS CLIENT
// ============================================================================

export class AssignmentsClient extends BaseClient {
  /**
   * List assignments
   */
  async list(params?: {
    courseId?: string;
    status?: string;
  }): Promise<ApiResponse<Assignment[]>> {
    return this.get<ApiResponse<Assignment[]>>('/assignments', params as Record<string, string>);
  }

  /**
   * Get assignment
   */
  async get(assignmentId: string): Promise<Assignment> {
    return this.get<Assignment>(`/assignments/${assignmentId}`);
  }

  /**
   * Create assignment
   */
  async create(data: Partial<Assignment>): Promise<Assignment> {
    return this.post<Assignment>('/assignments', data);
  }

  /**
   * Submit assignment
   */
  async submit(assignmentId: string, submission: { content: string; attachments?: File[] }): Promise<void> {
    return this.post<void>(`/assignments/${assignmentId}/submit`, submission);
  }

  /**
   * Grade submission
   */
  async grade(submissionId: string, grade: { score: number; feedback?: string }): Promise<void> {
    return this.put<void>(`/assignments/submissions/${submissionId}/grade`, grade);
  }
}

// ============================================================================
// QUIZZES CLIENT
// ============================================================================

export class QuizzesClient extends BaseClient {
  /**
   * List quizzes
   */
  async list(params?: { courseId?: string }): Promise<ApiResponse<Quiz[]>> {
    return this.get<ApiResponse<Quiz[]>>('/quizzes', params as Record<string, string>);
  }

  /**
   * Get quiz
   */
  async get(quizId: string): Promise<Quiz> {
    return this.get<Quiz>(`/quizzes/${quizId}`);
  }

  /**
   * Start quiz attempt
   */
  async startAttempt(quizId: string): Promise<{ attemptId: string; questions: unknown[] }> {
    return this.post<{ attemptId: string; questions: unknown[] }>(`/quizzes/${quizId}/start`);
  }

  /**
   * Submit quiz answers
   */
  async submitAnswers(attemptId: string, answers: Record<string, unknown>): Promise<{ score: number; correct: number }> {
    return this.post<{ score: number; correct: number }>(`/quizzes/attempts/${attemptId}/submit`, { answers });
  }
}

// ============================================================================
// GRADES CLIENT
// ============================================================================

export class GradesClient extends BaseClient {
  /**
   * Get gradebook for course
   */
  async getGradebook(courseId: string): Promise<Grade[]> {
    return this.get<Grade[]>(`/courses/${courseId}/grades`);
  }

  /**
   * Update grade
   */
  async update(userId: string, courseId: string, grade: { score: number; letter?: string }): Promise<Grade> {
    return this.put<Grade>(`/grades/${userId}/${courseId}`, grade);
  }

  /**
   * Export gradebook
   */
  async export(courseId: string, format: 'csv' | 'excel' | 'pdf'): Promise<Blob> {
    const response = await this.client.config.fetch(
      `${this.client.config.baseUrl}/api/v1/courses/${courseId}/grades/export?format=${format}`,
      {
        headers: { 'Authorization': `Bearer ${this.client.config.apiKey}` },
      }
    );
    return response.blob();
  }
}

// ============================================================================
// ATTENDANCE CLIENT
// ============================================================================

export class AttendanceClient extends BaseClient {
  /**
   * Get attendance records
   */
  async list(params?: { courseId?: string; date?: string }): Promise<ApiResponse<AttendanceRecord[]>> {
    return this.get<ApiResponse<AttendanceRecord[]>>('/attendance', params as Record<string, string>);
  }

  /**
   * Mark attendance
   */
  async mark(data: { userId: string; courseId: string; status: string; date: string }): Promise<AttendanceRecord> {
    return this.post<AttendanceRecord>('/attendance', data);
  }

  /**
   * Generate QR code for session
   */
  async generateQR(courseId: string): Promise<{ qrCode: string; expiresAt: string }> {
    return this.post<{ qrCode: string; expiresAt: string }>(`/attendance/qr`, { courseId });
  }
}

// ============================================================================
// ANNOUNCEMENTS CLIENT
// ============================================================================

export class AnnouncementsClient extends BaseClient {
  /**
   * List announcements
   */
  async list(params?: { courseId?: string }): Promise<ApiResponse<Announcement[]>> {
    return this.get<ApiResponse<Announcement[]>>('/announcements', params as Record<string, string>);
  }

  /**
   * Create announcement
   */
  async create(data: Partial<Announcement>): Promise<Announcement> {
    return this.post<Announcement>('/announcements', data);
  }

  /**
   * Delete announcement
   */
  async delete(announcementId: string): Promise<void> {
    return this.delete<void>(`/announcements/${announcementId}`);
  }
}

// ============================================================================
// ANALYTICS CLIENT
// ============================================================================

export class AnalyticsClient extends BaseClient {
  /**
   * Get learner dashboard
   */
  async getDashboard(userId: string): Promise<unknown> {
    return this.get<unknown>(`/analytics/dashboard/${userId}`);
  }

  /**
   * Get course analytics
   */
  async getCourseAnalytics(courseId: string): Promise<unknown> {
    return this.get<unknown>(`/analytics/courses/${courseId}`);
  }

  /**
   * Get cohort comparison
   */
  async getCohortComparison(cohortId: string): Promise<unknown> {
    return this.get<unknown>(`/analytics/cohorts/${cohortId}`);
  }
}

// ============================================================================
// ERROR CLASS
// ============================================================================

export class SmartLMSError extends Error {
  constructor(
    message: string,
    public code: string,
    public status: number
  ) {
    super(message);
    this.name = 'SmartLMSError';
  }
}

// ============================================================================
// EXPORTS
// ============================================================================

export default SmartLMS;
export type { Course, User, Enrollment, Assignment, Quiz, Grade, AttendanceRecord, Announcement };