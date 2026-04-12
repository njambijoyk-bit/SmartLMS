// Course Management API Service
const API_BASE = '/api/courses';

export interface Course {
  id: string;
  institution_id: string;
  code: string;
  title: string;
  description?: string;
  category?: string;
  status: 'draft' | 'published' | 'archived';
  created_at: string;
  updated_at: string;
}

export interface Module {
  id: string;
  course_id: string;
  title: string;
  description?: string;
  order: number;
  duration_minutes: number;
  is_preview: boolean;
  lessons: Lesson[];
}

export interface Lesson {
  id: string;
  module_id: string;
  title: string;
  lesson_type: 'video' | 'text' | 'quiz' | 'assignment' | 'document' | 'external' | 'scorm';
  content?: string;
  video_url?: string;
  duration_minutes: number;
  order: number;
  is_preview: boolean;
  is_free: boolean;
}

export interface CreateCourseRequest {
  code: string;
  title: string;
  description?: string;
  category?: string;
}

export interface UpdateCourseRequest {
  code?: string;
  title?: string;
  description?: string;
  category?: string;
}

export interface CreateModuleRequest {
  course_id: string;
  title: string;
  description?: string;
  order: number;
  duration_minutes?: number;
  is_preview?: boolean;
}

export interface UpdateModuleRequest {
  title?: string;
  description?: string;
  duration_minutes?: number;
  is_preview?: boolean;
}

export interface CreateLessonRequest {
  module_id: string;
  title: string;
  lesson_type: string;
  content?: string;
  video_url?: string;
  duration_minutes: number;
  order: number;
  is_preview?: boolean;
  is_free?: boolean;
}

export interface UpdateLessonRequest {
  title?: string;
  lesson_type?: string;
  content?: string;
  video_url?: string;
  duration_minutes?: number;
  is_preview?: boolean;
  is_free?: boolean;
}

export interface ReorderItem {
  id: string;
  order: number;
}

export interface ReorderItemsRequest {
  items: ReorderItem[];
}

export interface CourseListResponse {
  courses: Course[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface CourseDetailResponse {
  course: Course;
  modules: Module[];
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
  return response.json();
}

// Course operations
export async function listCourses(page = 1, perPage = 20, category?: string, search?: string): Promise<CourseListResponse> {
  const params = new URLSearchParams({
    page: page.toString(),
    per_page: perPage.toString(),
  });
  if (category) params.append('category', category);
  if (search) params.append('search', search);
  
  const response = await fetch(`${API_BASE}?${params}`);
  return handleResponse<CourseListResponse>(response);
}

export async function getInstructorCourses(page = 1, perPage = 20): Promise<CourseListResponse> {
  const params = new URLSearchParams({
    page: page.toString(),
    per_page: perPage.toString(),
  });
  
  const response = await fetch(`${API_BASE}/instructor?${params}`);
  return handleResponse<CourseListResponse>(response);
}

export async function getCourse(courseId: string): Promise<CourseDetailResponse> {
  const response = await fetch(`${API_BASE}/${courseId}`);
  return handleResponse<CourseDetailResponse>(response);
}

export async function createCourse(data: CreateCourseRequest): Promise<Course> {
  const response = await fetch(API_BASE, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Course>(response);
}

export async function updateCourse(courseId: string, data: UpdateCourseRequest): Promise<Course> {
  const response = await fetch(`${API_BASE}/${courseId}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Course>(response);
}

export async function deleteCourse(courseId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/${courseId}`, {
    method: 'DELETE',
  });
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
}

export async function publishCourse(courseId: string): Promise<Course> {
  const response = await fetch(`${API_BASE}/${courseId}/publish`, {
    method: 'POST',
  });
  return handleResponse<Course>(response);
}

export async function archiveCourse(courseId: string): Promise<Course> {
  const response = await fetch(`${API_BASE}/${courseId}/archive`, {
    method: 'POST',
  });
  return handleResponse<Course>(response);
}

// Module operations
export async function createModule(data: CreateModuleRequest): Promise<Module> {
  const response = await fetch(`${API_BASE}/modules`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Module>(response);
}

export async function updateModule(moduleId: string, data: UpdateModuleRequest): Promise<Module> {
  const response = await fetch(`${API_BASE}/modules/${moduleId}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Module>(response);
}

export async function deleteModule(moduleId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/modules/${moduleId}`, {
    method: 'DELETE',
  });
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
}

export async function reorderModules(items: ReorderItem[]): Promise<void> {
  const response = await fetch(`${API_BASE}/modules/reorder`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ items }),
  });
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
}

// Lesson operations
export async function createLesson(data: CreateLessonRequest): Promise<Lesson> {
  const response = await fetch(`${API_BASE}/lessons`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Lesson>(response);
}

export async function updateLesson(lessonId: string, data: UpdateLessonRequest): Promise<Lesson> {
  const response = await fetch(`${API_BASE}/lessons/${lessonId}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return handleResponse<Lesson>(response);
}

export async function deleteLesson(lessonId: string): Promise<void> {
  const response = await fetch(`${API_BASE}/lessons/${lessonId}`, {
    method: 'DELETE',
  });
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
}

export async function reorderLessons(items: ReorderItem[]): Promise<void> {
  const response = await fetch(`${API_BASE}/lessons/reorder`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ items }),
  });
  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }
}
