import React, { useState, useMemo } from 'react';
import { useGradebook, GradeEntry, GradebookStats } from '../../hooks/useGradebook';
import { Card, CardHeader, CardTitle, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { Input } from '../ui/Input';
import { Select } from '../ui/Select';
import { Badge } from '../ui/Badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '../ui/Dialog';
import { Textarea } from '../ui/Textarea';
import { Label } from '../ui/Label';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/Tabs';

interface GradebookDashboardProps {
  courseId: string;
  courseName: string;
}

export const GradebookDashboard: React.FC<GradebookDashboardProps> = ({ courseId, courseName }) => {
  const {
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
    refresh
  } = useGradebook(courseId);

  const [selectedGrades, setSelectedGrades] = useState<Set<string>>(new Set());
  const [editingGrade, setEditingGrade] = useState<GradeEntry | null>(null);
  const [overrideDialog, setOverrideDialog] = useState<{ open: boolean; gradeId?: string; score?: number }>({ open: false });
  const [overrideReason, setOverrideReason] = useState('');
  const [bulkFeedback, setBulkFeedback] = useState('');

  const handleSelectGrade = (gradeId: string) => {
    const newSelected = new Set(selectedGrades);
    if (newSelected.has(gradeId)) {
      newSelected.delete(gradeId);
    } else {
      newSelected.add(gradeId);
    }
    setSelectedGrades(newSelected);
  };

  const handleSelectAll = () => {
    if (selectedGrades.size === grades.length) {
      setSelectedGrades(new Set());
    } else {
      setSelectedGrades(new Set(grades.map(g => g.id)));
    }
  };

  const handleBulkUpdate = async (field: keyof GradeEntry, value: any) => {
    if (selectedGrades.size === 0) return;
    try {
      await bulkUpdateGrades(Array.from(selectedGrades), { [field]: value });
      setSelectedGrades(new Set());
    } catch (err: any) {
      alert(err.message);
    }
  };

  const handleExport = async (format: 'csv' | 'xlsx') => {
    try {
      await exportGrades(format);
    } catch (err: any) {
      alert(err.message);
    }
  };

  const handleOverride = async () => {
    if (!overrideDialog.gradeId || overrideDialog.score === undefined) return;
    try {
      await overrideGrade(overrideDialog.gradeId, overrideDialog.score, overrideReason);
      setOverrideDialog({ open: false });
      setOverrideReason('');
    } catch (err: any) {
      alert(err.message);
    }
  };

  const getStatusBadge = (status: GradeEntry['status']) => {
    const variants = {
      missing: 'destructive',
      submitted: 'default',
      graded: 'success',
      late: 'warning'
    } as const;
    return <Badge variant={variants[status]}>{status}</Badge>;
  };

  const getGradeLetter = (percentage: number | null): string => {
    if (percentage === null) return '-';
    if (percentage >= 90) return 'A';
    if (percentage >= 80) return 'B';
    if (percentage >= 70) return 'C';
    if (percentage >= 60) return 'D';
    return 'F';
  };

  if (loading) {
    return <div className="flex items-center justify-center p-8">Loading gradebook...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-600 bg-red-50 rounded-lg">{error}</div>;
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold">{courseName} - Gradebook</h1>
          <p className="text-gray-500">Manage grades and view analytics</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => handleExport('csv')}>
            Export CSV
          </Button>
          <Button variant="outline" onClick={() => handleExport('xlsx')}>
            Export Excel
          </Button>
          <Button onClick={() => refresh()}>Refresh</Button>
        </div>
      </div>

      {/* Stats Cards */}
      {stats && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium">Average Score</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.average.toFixed(1)}%</div>
              <p className="text-xs text-gray-500">Class average</p>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium">Submission Rate</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{(stats.submission_rate * 100).toFixed(1)}%</div>
              <p className="text-xs text-gray-500">{stats.late_submission_rate > 0 && `${(stats.late_submission_rate * 100).toFixed(1)}% late`}</p>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium">Highest Score</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.highest}%</div>
              <p className="text-xs text-gray-500">Top performer</p>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium">Lowest Score</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.lowest}%</div>
              <p className="text-xs text-gray-500">Needs attention</p>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Filters */}
      <Card>
        <CardContent className="pt-6">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div>
              <Label>Search</Label>
              <Input
                placeholder="Student or assessment name"
                value={filters.search_query || ''}
                onChange={(e) => setFilters({ ...filters, search_query: e.target.value })}
              />
            </div>
            <div>
              <Label>Status</Label>
              <Select
                value={filters.status || ''}
                onChange={(e) => setFilters({ ...filters, status: e.target.value as GradeEntry['status'] || undefined })}
              >
                <option value="">All Statuses</option>
                <option value="missing">Missing</option>
                <option value="submitted">Submitted</option>
                <option value="graded">Graded</option>
                <option value="late">Late</option>
              </Select>
            </div>
            <div>
              <Label>Date From</Label>
              <Input
                type="date"
                value={filters.date_from || ''}
                onChange={(e) => setFilters({ ...filters, date_from: e.target.value })}
              />
            </div>
            <div>
              <Label>Date To</Label>
              <Input
                type="date"
                value={filters.date_to || ''}
                onChange={(e) => setFilters({ ...filters, date_to: e.target.value })}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Bulk Actions */}
      {selectedGrades.size > 0 && (
        <Card className="bg-blue-50 border-blue-200">
          <CardContent className="pt-6">
            <div className="flex items-center gap-4">
              <span className="font-medium">{selectedGrades.size} selected</span>
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  const score = prompt('Enter score:');
                  if (score) handleBulkUpdate('score', parseFloat(score));
                }}
              >
                Set Score
              </Button>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="outline" size="sm">Add Feedback</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Add Bulk Feedback</DialogTitle>
                  </DialogHeader>
                  <Textarea
                    value={bulkFeedback}
                    onChange={(e) => setBulkFeedback(e.target.value)}
                    placeholder="Enter feedback for selected submissions"
                    className="mt-4"
                  />
                  <div className="flex justify-end gap-2 mt-4">
                    <Button variant="outline" onClick={() => setBulkFeedback('')}>Cancel</Button>
                    <Button onClick={() => handleBulkUpdate('feedback', bulkFeedback)}>Apply</Button>
                  </div>
                </DialogContent>
              </Dialog>
              <Button variant="outline" size="sm" onClick={() => setSelectedGrades(new Set())}>
                Clear Selection
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Grade Table */}
      <Card>
        <CardContent className="p-0">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-50 border-b">
                <tr>
                  <th className="p-4 text-left">
                    <input
                      type="checkbox"
                      checked={selectedGrades.size === grades.length && grades.length > 0}
                      onChange={handleSelectAll}
                    />
                  </th>
                  <th className="p-4 text-left font-medium">Student</th>
                  <th className="p-4 text-left font-medium">Assessment</th>
                  <th className="p-4 text-left font-medium">Status</th>
                  <th className="p-4 text-left font-medium">Score</th>
                  <th className="p-4 text-left font-medium">Grade</th>
                  <th className="p-4 text-left font-medium">Submitted</th>
                  <th className="p-4 text-left font-medium">Actions</th>
                </tr>
              </thead>
              <tbody>
                {grades.map((grade) => (
                  <tr key={grade.id} className="border-b hover:bg-gray-50">
                    <td className="p-4">
                      <input
                        type="checkbox"
                        checked={selectedGrades.has(grade.id)}
                        onChange={() => handleSelectGrade(grade.id)}
                      />
                    </td>
                    <td className="p-4">
                      <div className="font-medium">{grade.student_name}</div>
                      <div className="text-xs text-gray-500">{grade.student_id}</div>
                    </td>
                    <td className="p-4">{grade.assessment_name}</td>
                    <td className="p-4">{getStatusBadge(grade.status)}</td>
                    <td className="p-4">
                      {grade.score !== null ? (
                        <div className="flex items-center gap-2">
                          <span>{grade.score}/{grade.max_score}</span>
                          {grade.override && (
                            <Badge variant="warning" className="text-xs">Overridden</Badge>
                          )}
                        </div>
                      ) : (
                        <span className="text-gray-400">-</span>
                      )}
                    </td>
                    <td className="p-4">
                      <span className={`font-medium ${
                        grade.percentage !== null && grade.percentage >= 70 
                          ? 'text-green-600' 
                          : grade.percentage !== null && grade.percentage < 60 
                            ? 'text-red-600' 
                            : ''
                      }`}>
                        {getGradeLetter(grade.percentage)}
                      </span>
                    </td>
                    <td className="p-4 text-sm text-gray-500">
                      {grade.submitted_at ? new Date(grade.submitted_at).toLocaleDateString() : '-'}
                    </td>
                    <td className="p-4">
                      <div className="flex gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setEditingGrade(grade)}
                        >
                          Edit
                        </Button>
                        {grade.status === 'graded' && (
                          <Dialog>
                            <DialogTrigger asChild>
                              <Button
                                variant="outline"
                                size="sm"
                                onClick={() => setOverrideDialog({ open: true, gradeId: grade.id, score: grade.score || 0 })}
                              >
                                Override
                              </Button>
                            </DialogTrigger>
                            <DialogContent>
                              <DialogHeader>
                                <DialogTitle>Override Grade</DialogTitle>
                              </DialogHeader>
                              <div className="space-y-4 mt-4">
                                <div>
                                  <Label>New Score</Label>
                                  <Input
                                    type="number"
                                    value={overrideDialog.score}
                                    onChange={(e) => setOverrideDialog({ ...overrideDialog, score: parseFloat(e.target.value) })}
                                    max={grade.max_score}
                                    min={0}
                                  />
                                </div>
                                <div>
                                  <Label>Reason</Label>
                                  <Textarea
                                    value={overrideReason}
                                    onChange={(e) => setOverrideReason(e.target.value)}
                                    placeholder="Explain why this grade is being overridden"
                                  />
                                </div>
                                <div className="flex justify-end gap-2">
                                  <Button variant="outline" onClick={() => {
                                    setOverrideDialog({ open: false });
                                    setOverrideReason('');
                                  }}>
                                    Cancel
                                  </Button>
                                  <Button onClick={handleOverride}>Override</Button>
                                </div>
                              </div>
                            </DialogContent>
                          </Dialog>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      {/* Edit Grade Dialog */}
      {editingGrade && (
        <Dialog open={!!editingGrade} onOpenChange={() => setEditingGrade(null)}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Edit Grade - {editingGrade.student_name}</DialogTitle>
            </DialogHeader>
            <div className="space-y-4 mt-4">
              <div>
                <Label>Score</Label>
                <Input
                  type="number"
                  value={editingGrade.score || ''}
                  onChange={(e) => setEditingGrade({ ...editingGrade, score: parseFloat(e.target.value) || null })}
                  max={editingGrade.max_score}
                  min={0}
                  placeholder={`Max: ${editingGrade.max_score}`}
                />
              </div>
              <div>
                <Label>Feedback</Label>
                <Textarea
                  value={editingGrade.feedback || ''}
                  onChange={(e) => setEditingGrade({ ...editingGrade, feedback: e.target.value })}
                  placeholder="Enter feedback for the student"
                  rows={4}
                />
              </div>
              <div className="flex justify-end gap-2">
                <Button variant="outline" onClick={() => setEditingGrade(null)}>Cancel</Button>
                <Button
                  onClick={async () => {
                    try {
                      await updateGrade(editingGrade.id, {
                        score: editingGrade.score,
                        feedback: editingGrade.feedback
                      });
                      setEditingGrade(null);
                    } catch (err: any) {
                      alert(err.message);
                    }
                  }}
                >
                  Save
                </Button>
              </div>
            </div>
          </DialogContent>
        </Dialog>
      )}

      {/* Analytics Tab */}
      {stats && (
        <Card>
          <CardHeader>
            <CardTitle>Grade Distribution</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex gap-4 items-end h-48">
              {Object.entries(stats.grade_distribution).map(([grade, count]) => {
                const height = grades.length > 0 ? (count / grades.length) * 100 : 0;
                return (
                  <div key={grade} className="flex-1 flex flex-col items-center">
                    <div
                      className="w-full bg-blue-500 rounded-t"
                      style={{ height: `${height}%`, minHeight: count > 0 ? '8px' : '0' }}
                    />
                    <div className="mt-2 text-sm font-medium">{grade}</div>
                    <div className="text-xs text-gray-500">{count}</div>
                  </div>
                );
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
};
