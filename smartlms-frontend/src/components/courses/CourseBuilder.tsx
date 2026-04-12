import { useState } from 'react';
import { motion, Reorder } from 'framer-motion';
import { GripVertical, Plus, Trash2, Edit2, Video, FileText, HelpCircle, FolderOpen, ExternalLink, Package } from 'lucide-react';
import { Card } from '../ui/Card';
import { Button } from '../ui/Button';
import { Input } from '../ui/Input';
import { Label } from '../ui/Label';
import { Textarea } from '../ui/Textarea';
import { Select } from '../ui/Select';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '../ui/Dialog';
import { Badge } from '../ui/Badge';

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

interface CourseBuilderProps {
  courseId?: string;
  onSave?: (data: any) => void;
}

const LESSON_TYPE_ICONS = {
  video: Video,
  text: FileText,
  quiz: HelpCircle,
  assignment: FileText,
  document: FileText,
  external: ExternalLink,
  scorm: Package,
};

export function CourseBuilder({ courseId, onSave }: CourseBuilderProps) {
  const [modules, setModules] = useState<Module[]>([]);
  const [editingModule, setEditingModule] = useState<Module | null>(null);
  const [editingLesson, setEditingLesson] = useState<Lesson | null>(null);
  const [selectedModuleId, setSelectedModuleId] = useState<string | null>(null);
  const [isModuleDialogOpen, setIsModuleDialogOpen] = useState(false);
  const [isLessonDialogOpen, setIsLessonDialogOpen] = useState(false);

  const addModule = () => {
    const newModule: Module = {
      id: crypto.randomUUID(),
      course_id: courseId || '',
      title: 'New Module',
      description: '',
      order: modules.length,
      duration_minutes: 0,
      is_preview: false,
      lessons: [],
    };
    setEditingModule(newModule);
    setIsModuleDialogOpen(true);
  };

  const saveModule = () => {
    if (!editingModule) return;
    
    if (modules.find(m => m.id === editingModule.id)) {
      setModules(modules.map(m => m.id === editingModule.id ? editingModule : m));
    } else {
      setModules([...modules, editingModule]);
    }
    setIsModuleDialogOpen(false);
    setEditingModule(null);
  };

  const deleteModule = (moduleId: string) => {
    setModules(modules.filter(m => m.id !== moduleId));
    if (selectedModuleId === moduleId) {
      setSelectedModuleId(null);
    }
  };

  const addLesson = () => {
    if (!selectedModuleId) return;
    
    const module = modules.find(m => m.id === selectedModuleId);
    if (!module) return;

    const newLesson: Lesson = {
      id: crypto.randomUUID(),
      module_id: selectedModuleId,
      title: 'New Lesson',
      lesson_type: 'text',
      content: '',
      duration_minutes: 10,
      order: module.lessons.length,
      is_preview: false,
      is_free: false,
    };
    setEditingLesson(newLesson);
    setIsLessonDialogOpen(true);
  };

  const saveLesson = () => {
    if (!editingLesson || !selectedModuleId) return;

    setModules(modules.map(m => {
      if (m.id !== selectedModuleId) return m;
      
      const existingLesson = m.lessons.find(l => l.id === editingLesson.id);
      if (existingLesson) {
        return {
          ...m,
          lessons: m.lessons.map(l => l.id === editingLesson.id ? editingLesson : l),
        };
      }
      return {
        ...m,
        lessons: [...m.lessons, editingLesson],
      };
    }));
    setIsLessonDialogOpen(false);
    setEditingLesson(null);
  };

  const deleteLesson = (lessonId: string) => {
    setModules(modules.map(m => ({
      ...m,
      lessons: m.lessons.filter(l => l.id !== lessonId),
    })));
  };

  const reorderModules = (newOrder: Module[]) => {
    setModules(newOrder.map((m, i) => ({ ...m, order: i })));
  };

  const reorderLessons = (moduleId: string, newLessons: Lesson[]) => {
    setModules(modules.map(m => {
      if (m.id !== moduleId) return m;
      return { ...m, lessons: newLessons.map((l, i) => ({ ...l, order: i })) };
    }));
  };

  const editModule = (module: Module) => {
    setEditingModule({ ...module });
    setIsModuleDialogOpen(true);
  };

  const editLesson = (lesson: Lesson) => {
    setEditingLesson({ ...lesson });
    setIsLessonDialogOpen(true);
  };

  const selectedModule = modules.find(m => m.id === selectedModuleId);

  return (
    <div className="flex gap-4 h-full">
      {/* Modules Panel */}
      <Card className="w-80 flex flex-col" padding="normal">
        <div className="flex items-center justify-between mb-4">
          <h3 className="font-semibold text-lg">Course Content</h3>
          <Button size="sm" onClick={addModule}>
            <Plus size={16} /> Add Module
          </Button>
        </div>

        <Reorder.Group axis="y" values={modules} onReorder={reorderModules} className="space-y-2 flex-1 overflow-auto">
          {modules.map((module) => (
            <Reorder.Item key={module.id} value={module} className="cursor-grab active:cursor-grabbing">
              <Card 
                hover 
                padding="small"
                className={`mb-2 ${selectedModuleId === module.id ? 'ring-2 ring-brand-500' : ''}`}
              >
                <div className="flex items-start gap-2">
                  <GripVertical size={16} className="mt-1 text-sand-400 shrink-0" />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <h4 className="font-medium text-sm truncate">{module.title}</h4>
                      <div className="flex items-center gap-1">
                        <button onClick={() => editModule(module)} className="p-1 hover:bg-sand-100 rounded">
                          <Edit2 size={12} />
                        </button>
                        <button onClick={() => deleteModule(module.id)} className="p-1 hover:bg-red-100 rounded text-red-600">
                          <Trash2 size={12} />
                        </button>
                      </div>
                    </div>
                    <p className="text-xs text-sand-500 mt-1">{module.lessons.length} lessons</p>
                  </div>
                </div>
              </Card>
            </Reorder.Item>
          ))}
        </Reorder.Group>

        {modules.length === 0 && (
          <div className="text-center py-8 text-sand-500">
            <FolderOpen size={32} className="mx-auto mb-2 opacity-50" />
            <p className="text-sm">No modules yet</p>
            <Button variant="secondary" size="sm" className="mt-2" onClick={addModule}>
              Create your first module
            </Button>
          </div>
        )}
      </Card>

      {/* Lessons Panel */}
      <Card className="flex-1 flex flex-col" padding="normal">
        {selectedModule ? (
          <>
            <div className="flex items-center justify-between mb-4">
              <div>
                <h3 className="font-semibold text-lg">{selectedModule.title}</h3>
                {selectedModule.description && (
                  <p className="text-sm text-sand-500">{selectedModule.description}</p>
                )}
              </div>
              <Button size="sm" onClick={addLesson}>
                <Plus size={16} /> Add Lesson
              </Button>
            </div>

            <Reorder.Group axis="y" values={selectedModule.lessons} onReorder={(newLessons) => reorderLessons(selectedModule.id, newLessons)} className="space-y-2 flex-1 overflow-auto">
              {selectedModule.lessons.map((lesson) => {
                const Icon = LESSON_TYPE_ICONS[lesson.lesson_type];
                return (
                  <Reorder.Item key={lesson.id} value={lesson} className="cursor-grab active:cursor-grabbing">
                    <Card hover padding="small" className="mb-2">
                      <div className="flex items-center gap-3">
                        <GripVertical size={16} className="text-sand-400 shrink-0" />
                        <Icon size={16} className="text-brand-500 shrink-0" />
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center justify-between">
                            <h4 className="font-medium text-sm truncate">{lesson.title}</h4>
                            <div className="flex items-center gap-2">
                              {lesson.is_preview && <Badge variant="info">Preview</Badge>}
                              {lesson.is_free && <Badge variant="success">Free</Badge>}
                              <span className="text-xs text-sand-500">{lesson.duration_minutes} min</span>
                              <div className="flex items-center gap-1">
                                <button onClick={() => editLesson(lesson)} className="p-1 hover:bg-sand-100 rounded">
                                  <Edit2 size={12} />
                                </button>
                                <button onClick={() => deleteLesson(lesson.id)} className="p-1 hover:bg-red-100 rounded text-red-600">
                                  <Trash2 size={12} />
                                </button>
                              </div>
                            </div>
                          </div>
                          <div className="flex items-center gap-2 mt-1">
                            <Badge variant="secondary" className="text-xs">{lesson.lesson_type}</Badge>
                          </div>
                        </div>
                      </div>
                    </Card>
                  </Reorder.Item>
                );
              })}
            </Reorder.Group>

            {selectedModule.lessons.length === 0 && (
              <div className="text-center py-8 text-sand-500">
                <FileText size={32} className="mx-auto mb-2 opacity-50" />
                <p className="text-sm">No lessons in this module</p>
                <Button variant="secondary" size="sm" className="mt-2" onClick={addLesson}>
                  Add your first lesson
                </Button>
              </div>
            )}
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-sand-500">
            <div className="text-center">
              <FolderOpen size={48} className="mx-auto mb-2 opacity-50" />
              <p>Select a module to view lessons</p>
            </div>
          </div>
        )}
      </Card>

      {/* Module Dialog */}
      <Dialog open={isModuleDialogOpen} onOpenChange={setIsModuleDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{editingModule?.id && modules.find(m => m.id === editingModule.id) ? 'Edit Module' : 'New Module'}</DialogTitle>
          </DialogHeader>
          {editingModule && (
            <div className="space-y-4">
              <div>
                <Label>Title</Label>
                <Input
                  value={editingModule.title}
                  onChange={(e) => setEditingModule({ ...editingModule, title: e.target.value })}
                />
              </div>
              <div>
                <Label>Description</Label>
                <Textarea
                  value={editingModule.description || ''}
                  onChange={(e) => setEditingModule({ ...editingModule, description: e.target.value })}
                  rows={3}
                />
              </div>
              <div className="flex items-center gap-4">
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={editingModule.is_preview}
                    onChange={(e) => setEditingModule({ ...editingModule, is_preview: e.target.checked })}
                  />
                  <span className="text-sm">Available as preview</span>
                </label>
              </div>
            </div>
          )}
          <DialogFooter>
            <Button variant="secondary" onClick={() => setIsModuleDialogOpen(false)}>Cancel</Button>
            <Button onClick={saveModule}>Save Module</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Lesson Dialog */}
      <Dialog open={isLessonDialogOpen} onOpenChange={setIsLessonDialogOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>{editingLesson?.id && selectedModule?.lessons.find(l => l.id === editingLesson.id) ? 'Edit Lesson' : 'New Lesson'}</DialogTitle>
          </DialogHeader>
          {editingLesson && (
            <div className="space-y-4">
              <div>
                <Label>Title</Label>
                <Input
                  value={editingLesson.title}
                  onChange={(e) => setEditingLesson({ ...editingLesson, title: e.target.value })}
                />
              </div>
              <div>
                <Label>Lesson Type</Label>
                <Select
                  value={editingLesson.lesson_type}
                  onChange={(value) => setEditingLesson({ ...editingLesson, lesson_type: value as Lesson['lesson_type'] })}
                  options={[
                    { value: 'video', label: 'Video' },
                    { value: 'text', label: 'Text Content' },
                    { value: 'quiz', label: 'Quiz' },
                    { value: 'assignment', label: 'Assignment' },
                    { value: 'document', label: 'Document' },
                    { value: 'external', label: 'External Link' },
                    { value: 'scorm', label: 'SCORM Package' },
                  ]}
                />
              </div>
              {(editingLesson.lesson_type === 'text' || editingLesson.lesson_type === 'document') && (
                <div>
                  <Label>Content</Label>
                  <Textarea
                    value={editingLesson.content || ''}
                    onChange={(e) => setEditingLesson({ ...editingLesson, content: e.target.value })}
                    rows={6}
                  />
                </div>
              )}
              {editingLesson.lesson_type === 'video' && (
                <div>
                  <Label>Video URL</Label>
                  <Input
                    value={editingLesson.video_url || ''}
                    onChange={(e) => setEditingLesson({ ...editingLesson, video_url: e.target.value })}
                    placeholder="https://..."
                  />
                </div>
              )}
              <div>
                <Label>Duration (minutes)</Label>
                <Input
                  type="number"
                  value={editingLesson.duration_minutes}
                  onChange={(e) => setEditingLesson({ ...editingLesson, duration_minutes: parseInt(e.target.value) || 0 })}
                />
              </div>
              <div className="flex items-center gap-4">
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={editingLesson.is_preview}
                    onChange={(e) => setEditingLesson({ ...editingLesson, is_preview: e.target.checked })}
                  />
                  <span className="text-sm">Available as preview</span>
                </label>
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={editingLesson.is_free}
                    onChange={(e) => setEditingLesson({ ...editingLesson, is_free: e.target.checked })}
                  />
                  <span className="text-sm">Free lesson</span>
                </label>
              </div>
            </div>
          )}
          <DialogFooter>
            <Button variant="secondary" onClick={() => setIsLessonDialogOpen(false)}>Cancel</Button>
            <Button onClick={saveLesson}>Save Lesson</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
