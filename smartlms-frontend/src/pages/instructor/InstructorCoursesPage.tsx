import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Plus, Edit, Trash2, Eye, Archive, Search, Grid3X3, List } from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { useInstructorCourses } from '../../hooks/useCourses';
import * as courseApi from '../../services/courseApi';

export function InstructorCoursesPage() {
  const navigate = useNavigate();
  const { courses, loading, error, page, totalPages, refresh, setPage } = useInstructorCourses();
  const [view, setView] = useState<'grid' | 'list'>('grid');
  const [searchTerm, setSearchTerm] = useState('');
  const [filterStatus, setFilterStatus] = useState<'all' | 'draft' | 'published' | 'archived'>('all');

  const filteredCourses = courses.filter(course => {
    const matchesSearch = course.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         course.code.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesFilter = filterStatus === 'all' || course.status === filterStatus;
    return matchesSearch && matchesFilter;
  });

  const handleDelete = async (courseId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    
    if (!confirm('Are you sure you want to delete this course? This action cannot be undone.')) {
      return;
    }
    
    try {
      await courseApi.deleteCourse(courseId);
      await refresh();
    } catch (err) {
      console.error('Failed to delete course:', err);
      alert('Failed to delete course. Please try again.');
    }
  };

  const handlePublish = async (courseId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    
    try {
      await courseApi.publishCourse(courseId);
      await refresh();
    } catch (err) {
      console.error('Failed to publish course:', err);
      alert('Failed to publish course. Please try again.');
    }
  };

  const handleArchive = async (courseId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    
    try {
      await courseApi.archiveCourse(courseId);
      await refresh();
    } catch (err) {
      console.error('Failed to archive course:', err);
      alert('Failed to archive course. Please try again.');
    }
  };

  const COLORS = ['bg-brand-500', 'bg-accent-400', 'bg-gold-500', 'bg-brand-300', 'bg-accent-300', 'bg-brand-700', 'bg-gold-400', 'bg-accent-500'];

  if (loading && courses.length === 0) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
          <p className="mt-4 text-sm text-sand-500">Loading your courses...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div initial={{ opacity: 0, y: 12 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.4 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">My Courses</h1>
            <p className="text-sm text-ink-tertiary mt-1">Manage and organize your course content</p>
          </div>
          <Button onClick={() => navigate('/instructor/courses/new')}>
            <Plus size={16} /> Create Course
          </Button>
        </div>
      </motion.div>

      {/* Filters bar */}
      <div className="flex items-center justify-between gap-4">
        <div className="relative flex-1 max-w-sm">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-placeholder" />
          <input
            type="text"
            placeholder="Search courses..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full bg-surface-raised border border-sand-300 rounded-lg pl-9 pr-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400"
          />
        </div>
        <div className="flex items-center gap-2">
          <div className="flex bg-sand-100 rounded-lg p-0.5">
            {['all', 'draft', 'published', 'archived'].map(f => (
              <button
                key={f}
                onClick={() => setFilterStatus(f as any)}
                className={`px-3 py-1.5 text-xs font-medium rounded-md transition-colors capitalize cursor-pointer ${
                  filterStatus === f ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                {f}
              </button>
            ))}
          </div>
          <div className="flex bg-sand-100 rounded-lg p-0.5">
            <button onClick={() => setView('grid')} className={`p-1.5 rounded-md cursor-pointer ${view === 'grid' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}>
              <Grid3X3 size={16} />
            </button>
            <button onClick={() => setView('list')} className={`p-1.5 rounded-md cursor-pointer ${view === 'list' ? 'bg-surface-raised text-ink shadow-sm' : 'text-ink-tertiary'}`}>
              <List size={16} />
            </button>
          </div>
        </div>
      </div>

      {/* Error message */}
      {error && (
        <Card padding="normal" className="bg-red-50 border-red-200">
          <p className="text-sm text-red-600">{error}</p>
        </Card>
      )}

      {/* Course grid */}
      {view === 'grid' ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.3 }}
          className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
        >
          {filteredCourses.map((course, i) => (
            <motion.div
              key={course.id}
              initial={{ opacity: 0, y: 15 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, delay: i * 0.05 }}
            >
              <Card hover padding="none" onClick={() => navigate(`/instructor/courses/${course.id}`)}>
                <div className={`h-24 ${COLORS[i % COLORS.length]} rounded-t-xl relative overflow-hidden`}>
                  <div className="absolute inset-0 opacity-10">
                    <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
                      <defs>
                        <pattern id={`p${i}`} width="40" height="40" patternUnits="userSpaceOnUse">
                          <circle cx="20" cy="20" r="12" fill="none" stroke="white" strokeWidth="1" />
                          <path d="M0 20h40M20 0v40" stroke="white" strokeWidth="0.5" />
                        </pattern>
                      </defs>
                      <rect width="100%" height="100%" fill={`url(#p${i})`} />
                    </svg>
                  </div>
                  <div className="absolute bottom-3 left-4">
                    <span className="text-white text-lg font-bold font-[family-name:var(--font-display)]">{course.code}</span>
                  </div>
                  <div className="absolute top-3 right-3">
                    <Badge variant={course.status === 'published' ? 'success' : course.status === 'archived' ? 'secondary' : 'warning'}>
                      {course.status}
                    </Badge>
                  </div>
                </div>
                <div className="p-4">
                  <h4 className="text-sm font-semibold text-ink font-[family-name:var(--font-display)] line-clamp-1">{course.title}</h4>
                  {course.description && (
                    <p className="text-xs text-ink-tertiary mt-1 line-clamp-2">{course.description}</p>
                  )}
                  <div className="flex items-center gap-2 mt-3 text-xs text-ink-tertiary">
                    <span>Created: {new Date(course.created_at).toLocaleDateString()}</span>
                  </div>
                  
                  {/* Action buttons */}
                  <div className="flex items-center gap-1 mt-3 pt-3 border-t border-sand-200">
                    {course.status === 'draft' && (
                      <button
                        onClick={(e) => handlePublish(course.id, e)}
                        className="flex-1 px-2 py-1.5 text-xs bg-brand-50 text-brand-600 rounded hover:bg-brand-100 transition-colors flex items-center justify-center gap-1"
                      >
                        <Eye size={12} /> Publish
                      </button>
                    )}
                    {course.status === 'published' && (
                      <button
                        onClick={(e) => handleArchive(course.id, e)}
                        className="flex-1 px-2 py-1.5 text-xs bg-sand-100 text-ink-tertiary rounded hover:bg-sand-200 transition-colors flex items-center justify-center gap-1"
                      >
                        <Archive size={12} /> Archive
                      </button>
                    )}
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        navigate(`/instructor/courses/${course.id}`);
                      }}
                      className="px-2 py-1.5 text-xs bg-sand-100 text-ink-tertiary rounded hover:bg-sand-200 transition-colors"
                    >
                      <Edit size={12} />
                    </button>
                    <button
                      onClick={(e) => handleDelete(course.id, e)}
                      className="px-2 py-1.5 text-xs bg-red-50 text-red-600 rounded hover:bg-red-100 transition-colors"
                    >
                      <Trash2 size={12} />
                    </button>
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </motion.div>
      ) : (
        <Card padding="none">
          <div className="divide-y divide-sand-200">
            {filteredCourses.map(course => (
              <div key={course.id} className="flex items-center gap-4 p-4 hover:bg-sand-50 transition-colors">
                <div className={`w-12 h-12 rounded-lg ${COLORS[courses.indexOf(course) % COLORS.length]} flex items-center justify-center shrink-0`}>
                  <span className="text-xs font-bold text-white font-[family-name:var(--font-display)]">{course.code}</span>
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="text-sm font-semibold text-ink">{course.title}</span>
                    <Badge variant={course.status === 'published' ? 'success' : course.status === 'archived' ? 'secondary' : 'warning'}>
                      {course.status}
                    </Badge>
                  </div>
                  <div className="text-xs text-ink-tertiary mt-0.5">
                    Created: {new Date(course.created_at).toLocaleDateString()}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {course.status === 'draft' && (
                    <Button size="sm" variant="secondary" onClick={(e) => handlePublish(course.id, e)}>
                      <Eye size={14} /> Publish
                    </Button>
                  )}
                  {course.status === 'published' && (
                    <Button size="sm" variant="secondary" onClick={(e) => handleArchive(course.id, e)}>
                      <Archive size={14} /> Archive
                    </Button>
                  )}
                  <Button size="sm" variant="secondary" onClick={() => navigate(`/instructor/courses/${course.id}`)}>
                    <Edit size={14} /> Edit
                  </Button>
                  <Button size="sm" variant="secondary" onClick={(e) => handleDelete(course.id, e)}>
                    <Trash2 size={14} />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-center gap-2">
          <Button
            variant="secondary"
            size="sm"
            disabled={page === 1}
            onClick={() => setPage(page - 1)}
          >
            Previous
          </Button>
          <span className="text-sm text-ink-tertiary">
            Page {page} of {totalPages}
          </span>
          <Button
            variant="secondary"
            size="sm"
            disabled={page === totalPages}
            onClick={() => setPage(page + 1)}
          >
            Next
          </Button>
        </div>
      )}

      {/* Empty state */}
      {filteredCourses.length === 0 && !loading && (
        <div className="text-center py-12">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-sand-100 mb-4">
            <Search size={24} className="text-sand-400" />
          </div>
          <h3 className="text-lg font-semibold text-ink mb-2">No courses found</h3>
          <p className="text-sm text-ink-tertiary mb-4">
            {searchTerm || filterStatus !== 'all'
              ? 'Try adjusting your search or filters'
              : 'Get started by creating your first course'}
          </p>
          {!searchTerm && filterStatus === 'all' && (
            <Button onClick={() => navigate('/instructor/courses/new')}>
              <Plus size={16} /> Create Course
            </Button>
          )}
        </div>
      )}
    </div>
  );
}
