import { useState, useEffect, useCallback } from 'react';
import * as courseApi from '../services/courseApi';

export function useInstructorCourses() {
  const [courses, setCourses] = useState<courseApi.Course[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [totalPages, setTotalPages] = useState(1);

  const fetchCourses = useCallback(async (pageNum = 1) => {
    try {
      setLoading(true);
      const response = await courseApi.getInstructorCourses(pageNum, 20);
      setCourses(response.courses);
      setTotalPages(response.total_pages);
      setPage(pageNum);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load courses');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchCourses();
  }, [fetchCourses]);

  return {
    courses,
    loading,
    error,
    page,
    totalPages,
    refresh: fetchCourses,
    setPage: (newPage: number) => fetchCourses(newPage),
  };
}

export function useCourseDetail(courseId?: string) {
  const [course, setCourse] = useState<courseApi.Course | null>(null);
  const [modules, setModules] = useState<courseApi.Module[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchCourse = useCallback(async () => {
    if (!courseId) return;
    
    try {
      setLoading(true);
      const response = await courseApi.getCourse(courseId);
      setCourse(response.course);
      setModules(response.modules || []);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load course');
    } finally {
      setLoading(false);
    }
  }, [courseId]);

  useEffect(() => {
    fetchCourse();
  }, [fetchCourse]);

  return {
    course,
    modules,
    loading,
    error,
    refresh: fetchCourse,
  };
}
