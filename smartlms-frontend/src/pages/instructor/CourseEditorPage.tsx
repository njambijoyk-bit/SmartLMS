import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { motion } from 'framer-motion';
import { Plus, ArrowLeft, Save, Eye, Archive, Trash2 } from 'lucide-react';
import { Card } from '../../components/ui/Card';
import { Button } from '../../components/ui/Button';
import { Badge } from '../../components/ui/Badge';
import { CourseBuilder } from '../../components/courses/CourseBuilder';
import * as courseApi from '../../services/courseApi';
import { useCourseDetail } from '../../hooks/useCourses';

export function CourseEditorPage() {
  const { courseId } = useParams<{ courseId: string }>();
  const navigate = useNavigate();
  const { course, modules, loading, error, refresh } = useCourseDetail(courseId);
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);

  // If no courseId, we're creating a new course
  const isNewCourse = !courseId;

  const handleSave = async () => {
    setIsSaving(true);
    try {
      // In a real implementation, you would save the course structure here
      // For now, just refresh the data
      await refresh();
      setHasChanges(false);
    } catch (err) {
      console.error('Failed to save:', err);
    } finally {
      setIsSaving(false);
    }
  };

  const handlePublish = async () => {
    if (!courseId) return;
    
    try {
      await courseApi.publishCourse(courseId);
      await refresh();
    } catch (err) {
      console.error('Failed to publish:', err);
    }
  };

  const handleArchive = async () => {
    if (!courseId) return;
    
    try {
      await courseApi.archiveCourse(courseId);
      await refresh();
    } catch (err) {
      console.error('Failed to archive:', err);
    }
  };

  const handleDelete = async () => {
    if (!courseId) return;
    
    if (!confirm('Are you sure you want to delete this course? This action cannot be undone.')) {
      return;
    }
    
    try {
      await courseApi.deleteCourse(courseId);
      navigate('/instructor/courses');
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  };

  if (loading && !isNewCourse) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-brand-500 mx-auto"></div>
          <p className="mt-4 text-sm text-sand-500">Loading course...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-full">
        <Card padding="md" className="max-w-md">
          <h3 className="font-semibold text-lg mb-2">Error Loading Course</h3>
          <p className="text-sm text-sand-500 mb-4">{error}</p>
          <Button onClick={() => navigate('/instructor/courses')}>
            <ArrowLeft size={16} /> Back to Courses
          </Button>
        </Card>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <motion.div 
        initial={{ opacity: 0, y: 12 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ duration: 0.4 }}
        className="flex items-center justify-between mb-6"
      >
        <div className="flex items-center gap-4">
          <Button variant="secondary" size="sm" onClick={() => navigate('/instructor/courses')}>
            <ArrowLeft size={16} />
          </Button>
          <div>
            <h1 className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink">
              {isNewCourse ? 'Create New Course' : course?.title || 'Course Editor'}
            </h1>
            {course && (
              <div className="flex items-center gap-2 mt-1">
                <span className="text-xs text-sand-500">{course.code}</span>
                <Badge variant={course.status === 'published' ? 'success' : course.status === 'archived' ? 'secondary' : 'warning'}>
                  {course.status}
                </Badge>
              </div>
            )}
          </div>
        </div>
        
        <div className="flex items-center gap-2">
          {!isNewCourse && course?.status === 'draft' && (
            <>
              <Button variant="secondary" onClick={handlePublish}>
                <Eye size={16} /> Publish
              </Button>
              <Button variant="secondary" onClick={handleArchive}>
                <Archive size={16} /> Archive
              </Button>
            </>
          )}
          
          {course?.status === 'archived' && (
            <Button variant="secondary" onClick={handleDelete}>
              <Trash2 size={16} /> Delete
            </Button>
          )}
          
          <Button onClick={handleSave} disabled={isSaving}>
            <Save size={16} /> {isSaving ? 'Saving...' : 'Save Changes'}
          </Button>
        </div>
      </motion.div>

      {/* Course Builder */}
      <div className="flex-1 min-h-0">
        <CourseBuilder 
          courseId={courseId} 
          onSave={(data) => {
            console.log('Course data to save:', data);
            setHasChanges(true);
          }} 
        />
      </div>
    </div>
  );
}
