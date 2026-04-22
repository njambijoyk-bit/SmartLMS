import { useState, useEffect, useCallback } from 'react';
import { api } from '../../lib/api';

export interface GradeEntry {
  id: string;
  student_id: string;
  student_name: string;
  assessment_id: string;
  assessment_name: string;
  score: number | null;
  max_score: number;
  percentage: number | null;
  grade_letter?: string;
  feedback?: string;
  submitted_at?: string;
  graded_at?: string;
  graded_by?: string;
  status: 'missing' | 'submitted' | 'graded' | 'late';
  is_late: boolean;
  override?: {
    previous_score: number;
    overridden_by: string;
    overridden_at: string;
    reason: string;
  };
}

export interface GradebookStats {
  average: number;
  median: number;
  highest: number;
  lowest: number;
  std_deviation: number;
  submission_rate: number;
  late_submission_rate: number;
  grade_distribution: {
    A: number;
    B: number;
    C: number;
    D: number;
    F: number;
  };
}

export interface GradebookFilters {
  assessment_id?: string;
  student_id?: string;
  status?: GradeEntry['status'];
  date_from?: string;
  date_to?: string;
  search_query?: string;
}

export function useGradebook(courseId: string) {
  const [grades, setGrades] = useState<GradeEntry[]>([]);
  const [stats, setStats] = useState<GradebookStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<GradebookFilters>({});

  const fetchGrades = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const params = new URLSearchParams();
      if (filters.assessment_id) params.append('assessment_id', filters.assessment_id);
      if (filters.student_id) params.append('student_id', filters.student_id);
      if (filters.status) params.append('status', filters.status);
      if (filters.date_from) params.append('date_from', filters.date_from);
      if (filters.date_to) params.append('date_to', filters.date_to);
      if (filters.search_query) params.append('search_query', filters.search_query);

      const response = await api.get(`/api/gradebook/courses/${courseId}?${params.toString()}`);
      setGrades(response.data.grades);
      setStats(response.data.stats);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to fetch grades');
    } finally {
      setLoading(false);
    }
  }, [courseId, filters]);

  const updateGrade = async (gradeId: string, updates: Partial<GradeEntry>) => {
    try {
      const response = await api.put(`/api/gradebook/grades/${gradeId}`, updates);
      setGrades(prev => prev.map(g => g.id === gradeId ? response.data : g));
      await fetchGrades(); // Refresh stats
      return response.data;
    } catch (err: any) {
      throw new Error(err.response?.data?.message || 'Failed to update grade');
    }
  };

  const bulkUpdateGrades = async (gradeIds: string[], updates: Partial<GradeEntry>) => {
    try {
      const response = await api.post('/api/gradebook/grades/bulk-update', {
        grade_ids: gradeIds,
        updates
      });
      await fetchGrades();
      return response.data;
    } catch (err: any) {
      throw new Error(err.response?.data?.message || 'Failed to bulk update grades');
    }
  };

  const exportGrades = async (format: 'csv' | 'xlsx') => {
    try {
      const response = await api.get(`/api/gradebook/courses/${courseId}/export?format=${format}`, {
        responseType: 'blob',
        params: filters
      });
      
      const blob = new Blob([response.data], { 
        type: format === 'csv' ? 'text/csv' : 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
      });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `gradebook-${courseId}-${new Date().toISOString().split('T')[0]}.${format}`;
      link.click();
      window.URL.revokeObjectURL(url);
    } catch (err: any) {
      throw new Error(err.response?.data?.message || 'Failed to export grades');
    }
  };

  const overrideGrade = async (gradeId: string, score: number, reason: string) => {
    try {
      const response = await api.post(`/api/gradebook/grades/${gradeId}/override`, {
        score,
        reason
      });
      setGrades(prev => prev.map(g => g.id === gradeId ? response.data : g));
      await fetchGrades();
      return response.data;
    } catch (err: any) {
      throw new Error(err.response?.data?.message || 'Failed to override grade');
    }
  };

  useEffect(() => {
    fetchGrades();
  }, [fetchGrades]);

  return {
    grades,
    stats,
    loading,
    error,
    filters,
    setFilters,
    updateGrade,
    bulkUpdateGrades,
    exportGrades,
    overrideGrade,
    refresh: fetchGrades
  };
}
