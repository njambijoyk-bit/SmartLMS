import { useParams, useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import {
  ArrowLeft, Users, Plus, MoreVertical, Edit2, Trash2, UserPlus, Upload,
  Video, FileText, Calendar, CheckCircle, XCircle, Clock, Search, Filter,
} from 'lucide-react';
import { useState } from 'react';
import { Card } from '../../components/ui/Card';
import { Badge } from '../../components/ui/Badge';
import { Button } from '../../components/ui/Button';
import { Avatar } from '../../components/ui/Avatar';
import { Input } from '../../components/ui/Input';

// Mock data - replace with API calls
const MOCK_GROUPS = [
  {
    id: '1',
    name: 'Section A - Morning Class',
    description: 'Monday/Wednesday morning sessions',
    instructor_name: 'Dr. Sarah Kimani',
    instructor_email: 's.kimani@university.ac.ke',
    student_count: 45,
    max_students: 50,
    is_active: true,
    created_at: '2025-01-15T08:00:00Z',
  },
  {
    id: '2',
    name: 'Section B - Afternoon Class',
    description: 'Tuesday/Thursday afternoon sessions',
    instructor_name: 'Prof. John Ochieng',
    instructor_email: 'j.ochieng@university.ac.ke',
    student_count: 38,
    max_students: 50,
    is_active: true,
    created_at: '2025-01-15T09:30:00Z',
  },
  {
    id: '3',
    name: 'Section C - Evening Class',
    description: 'Evening sessions for working students',
    instructor_name: 'Dr. Mary Wanjiku',
    instructor_email: 'm.wanjiku@university.ac.ke',
    student_count: 32,
    max_students: 40,
    is_active: true,
    created_at: '2025-01-16T14:00:00Z',
  },
];

const MOCK_STUDENTS = [
  { user_id: '1', first_name: 'James', last_name: 'Mutua', email: 'j.mutua@student.ac.ke', enrolled_at: '2025-01-20T10:00:00Z', enrollment_status: 'active' },
  { user_id: '2', first_name: 'Grace', last_name: 'Achieng', email: 'g.achieng@student.ac.ke', enrolled_at: '2025-01-20T10:05:00Z', enrollment_status: 'active' },
  { user_id: '3', first_name: 'Brian', last_name: 'Kamau', email: 'b.kamau@student.ac.ke', enrolled_at: '2025-01-20T10:10:00Z', enrollment_status: 'active' },
  { user_id: '4', first_name: 'Faith', last_name: 'Njeri', email: 'f.njeri@student.ac.ke', enrolled_at: '2025-01-20T10:15:00Z', enrollment_status: 'active' },
  { user_id: '5', first_name: 'Kevin', last_name: 'Omondi', email: 'k.omondi@student.ac.ke', enrolled_at: '2025-01-20T10:20:00Z', enrollment_status: 'active' },
];

const MOCK_SESSIONS = [
  { session_id: '1', title: 'Week 1: Introduction to Data Structures', scheduled_start: '2025-02-03T08:00:00Z', status: 'scheduled' },
  { session_id: '2', title: 'Week 2: Arrays and Complexity', scheduled_start: '2025-02-05T08:00:00Z', status: 'scheduled' },
];

const MOCK_ASSESSMENTS = [
  { assessment_id: '1', title: 'CAT 1: Arrays & Linked Lists', assessment_type: 'exam', is_group_only: true },
  { assessment_id: '2', title: 'Assignment 1: Stack Implementation', assessment_type: 'assignment', is_group_only: false },
];

export function CourseGroupsPage() {
  const { courseId } = useParams();
  const navigate = useNavigate();
  
  const [groups, setGroups] = useState(MOCK_GROUPS);
  const [selectedGroup, setSelectedGroup] = useState<any>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showAddStudentModal, setShowAddStudentModal] = useState(false);
  const [activeTab, setActiveTab] = useState<'overview' | 'students' | 'sessions' | 'assessments'>('overview');
  const [searchTerm, setSearchTerm] = useState('');
  
  const filteredGroups = groups.filter(g => 
    g.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    g.instructor_name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleCreateGroup = (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Call API to create group
    setShowCreateModal(false);
  };

  const handleAddStudent = (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Call API to add student
    setShowAddStudentModal(false);
  };

  const handleDeleteGroup = (groupId: string) => {
    if (confirm('Are you sure you want to delete this group? Students will be unenrolled.')) {
      setGroups(groups.filter(g => g.id !== groupId));
      if (selectedGroup?.id === groupId) setSelectedGroup(null);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <button onClick={() => navigate(`/courses/${courseId}`)} className="flex items-center gap-1.5 text-sm text-ink-tertiary hover:text-ink transition-colors cursor-pointer">
          <ArrowLeft size={16} /> Back to Course
        </button>
        <div className="flex items-center gap-3">
          <Button variant="outline" onClick={() => setShowAddStudentModal(true)}>
            <Upload size={16} className="mr-2" /> Bulk Import
          </Button>
          <Button onClick={() => setShowCreateModal(true)}>
            <Plus size={16} className="mr-2" /> Create Group
          </Button>
        </div>
      </div>

      {/* Page Title */}
      <div>
        <h1 className="text-2xl font-semibold text-ink">Course Groups</h1>
        <p className="text-sm text-ink-secondary mt-1">
          Manage lecturer-specific student groups within this course
        </p>
      </div>

      {!selectedGroup ? (
        /* Groups List View */
        <Card>
          {/* Search & Filter */}
          <div className="flex items-center justify-between mb-4">
            <div className="relative w-64">
              <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-tertiary" />
              <Input
                placeholder="Search groups..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-10"
              />
            </div>
            <div className="flex items-center gap-2 text-sm text-ink-tertiary">
              <Filter size={14} />
              <span>{filteredGroups.length} groups</span>
            </div>
          </div>

          {/* Groups Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filteredGroups.map((group) => (
              <motion.div
                key={group.id}
                initial={{ opacity: 0, y: 8 }}
                animate={{ opacity: 1, y: 0 }}
                className="cursor-pointer"
                onClick={() => setSelectedGroup(group)}
              >
                <Card className="hover:border-brand-300 transition-colors" padding="normal">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex-1">
                      <h3 className="font-medium text-ink">{group.name}</h3>
                      <p className="text-xs text-ink-tertiary mt-1">{group.description}</p>
                    </div>
                    <button className="text-ink-tertiary hover:text-ink">
                      <MoreVertical size={16} />
                    </button>
                  </div>

                  <div className="space-y-2 mb-4">
                    <div className="flex items-center gap-2 text-xs text-ink-secondary">
                      <Users size={12} />
                      <span>{group.student_count} / {group.max_students} students</span>
                    </div>
                    <div className="flex items-center gap-2 text-xs text-ink-secondary">
                      <Avatar name={group.instructor_name} size="xs" />
                      <span>{group.instructor_name}</span>
                    </div>
                  </div>

                  <div className="flex items-center justify-between pt-3 border-t border-line">
                    <Badge variant={group.is_active ? 'success' : 'secondary'} size="small">
                      {group.is_active ? 'Active' : 'Inactive'}
                    </Badge>
                    <span className="text-xs text-ink-tertiary">
                      {new Date(group.created_at).toLocaleDateString()}
                    </span>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>

          {filteredGroups.length === 0 && (
            <div className="text-center py-12">
              <Users size={48} className="mx-auto text-ink-tertiary mb-3" />
              <h3 className="text-lg font-medium text-ink mb-1">No groups found</h3>
              <p className="text-sm text-ink-tertiary mb-4">
                Create your first group to organize students by lecturer or schedule
              </p>
              <Button onClick={() => setShowCreateModal(true)}>
                <Plus size={16} className="mr-2" /> Create Group
              </Button>
            </div>
          )}
        </Card>
      ) : (
        /* Group Detail View */
        <div className="space-y-6">
          {/* Group Header */}
          <Card padding="normal">
            <div className="flex items-start justify-between">
              <div>
                <div className="flex items-center gap-3 mb-2">
                  <button onClick={() => setSelectedGroup(null)} className="text-ink-tertiary hover:text-ink">
                    <ArrowLeft size={18} />
                  </button>
                  <h2 className="text-xl font-semibold text-ink">{selectedGroup.name}</h2>
                  <Badge variant={selectedGroup.is_active ? 'success' : 'secondary'}>
                    {selectedGroup.is_active ? 'Active' : 'Inactive'}
                  </Badge>
                </div>
                <p className="text-sm text-ink-secondary ml-11">{selectedGroup.description}</p>
              </div>
              <div className="flex items-center gap-2">
                <Button variant="outline" size="small">
                  <Edit2 size={14} className="mr-2" /> Edit
                </Button>
                <Button variant="outline" size="small" className="text-danger hover:bg-danger/10" onClick={() => handleDeleteGroup(selectedGroup.id)}>
                  <Trash2 size={14} className="mr-2" /> Delete
                </Button>
              </div>
            </div>

            <div className="flex items-center gap-6 mt-4 ml-11">
              <div className="flex items-center gap-2 text-sm text-ink-secondary">
                <Users size={16} />
                <span>{selectedGroup.student_count} / {selectedGroup.max_students} students</span>
              </div>
              <div className="flex items-center gap-2 text-sm text-ink-secondary">
                <Avatar name={selectedGroup.instructor_name} size="sm" />
                <span>{selectedGroup.instructor_name}</span>
              </div>
              <div className="flex items-center gap-2 text-sm text-ink-tertiary">
                <Clock size={16} />
                <span>Created {new Date(selectedGroup.created_at).toLocaleDateString()}</span>
              </div>
            </div>
          </Card>

          {/* Tabs */}
          <div className="border-b border-line">
            <div className="flex items-center gap-6">
              <button
                onClick={() => setActiveTab('overview')}
                className={`pb-3 text-sm font-medium transition-colors ${
                  activeTab === 'overview'
                    ? 'text-brand-500 border-b-2 border-brand-500'
                    : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                Overview
              </button>
              <button
                onClick={() => setActiveTab('students')}
                className={`pb-3 text-sm font-medium transition-colors ${
                  activeTab === 'students'
                    ? 'text-brand-500 border-b-2 border-brand-500'
                    : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                Students ({selectedGroup.student_count})
              </button>
              <button
                onClick={() => setActiveTab('sessions')}
                className={`pb-3 text-sm font-medium transition-colors ${
                  activeTab === 'sessions'
                    ? 'text-brand-500 border-b-2 border-brand-500'
                    : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                Live Sessions
              </button>
              <button
                onClick={() => setActiveTab('assessments')}
                className={`pb-3 text-sm font-medium transition-colors ${
                  activeTab === 'assessments'
                    ? 'text-brand-500 border-b-2 border-brand-500'
                    : 'text-ink-tertiary hover:text-ink'
                }`}
              >
                Assessments
              </button>
            </div>
          </div>

          {/* Tab Content */}
          {activeTab === 'overview' && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Card padding="normal">
                <h3 className="font-medium text-ink mb-3">Quick Stats</h3>
                <div className="space-y-3">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-ink-tertiary">Enrollment Rate</span>
                    <span className="font-medium text-ink">
                      {Math.round((selectedGroup.student_count / selectedGroup.max_students!) * 100)}%
                    </span>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-ink-tertiary">Live Sessions</span>
                    <span className="font-medium text-ink">{MOCK_SESSIONS.length}</span>
                  </div>
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-ink-tertiary">Assessments</span>
                    <span className="font-medium text-ink">{MOCK_ASSESSMENTS.length}</span>
                  </div>
                </div>
              </Card>
              <Card padding="normal">
                <h3 className="font-medium text-ink mb-3">Recent Activity</h3>
                <div className="text-sm text-ink-tertiary">
                  No recent activity
                </div>
              </Card>
            </div>
          )}

          {activeTab === 'students' && (
            <Card padding="none">
              <div className="p-4 border-b border-line flex items-center justify-between">
                <div className="relative w-64">
                  <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-ink-tertiary" />
                  <Input placeholder="Search students..." className="pl-10" />
                </div>
                <Button size="small" onClick={() => setShowAddStudentModal(true)}>
                  <UserPlus size={16} className="mr-2" /> Add Student
                </Button>
              </div>
              <table className="w-full">
                <thead className="bg-surface-2 text-xs text-ink-tertiary">
                  <tr>
                    <th className="text-left font-medium p-3">Student</th>
                    <th className="text-left font-medium p-3">Email</th>
                    <th className="text-left font-medium p-3">Enrolled</th>
                    <th className="text-left font-medium p-3">Status</th>
                    <th className="text-right font-medium p-3">Actions</th>
                  </tr>
                </thead>
                <tbody className="text-sm">
                  {MOCK_STUDENTS.map((student) => (
                    <tr key={student.user_id} className="border-t border-line hover:bg-surface-2">
                      <td className="p-3">
                        <div className="flex items-center gap-2">
                          <Avatar name={`${student.first_name} ${student.last_name}`} size="sm" />
                          <span className="text-ink">{student.first_name} {student.last_name}</span>
                        </div>
                      </td>
                      <td className="p-3 text-ink-secondary">{student.email}</td>
                      <td className="p-3 text-ink-tertiary">{new Date(student.enrolled_at).toLocaleDateString()}</td>
                      <td className="p-3">
                        <Badge variant="success" size="small">{student.enrollment_status}</Badge>
                      </td>
                      <td className="p-3 text-right">
                        <button className="text-ink-tertiary hover:text-danger text-xs">Remove</button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </Card>
          )}

          {activeTab === 'sessions' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="font-medium text-ink">Scheduled Sessions</h3>
                <Button size="small" onClick={() => navigate(`/live?groupId=${selectedGroup.id}`)}>
                  <Video size={16} className="mr-2" /> Schedule Session
                </Button>
              </div>
              {MOCK_SESSIONS.map((session) => (
                <Card key={session.session_id} padding="normal">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="p-2 bg-brand-100 rounded-lg">
                        <Video size={20} className="text-brand-500" />
                      </div>
                      <div>
                        <h4 className="font-medium text-ink">{session.title}</h4>
                        <p className="text-sm text-ink-tertiary">
                          {new Date(session.scheduled_start).toLocaleString()}
                        </p>
                      </div>
                    </div>
                    <Badge variant={session.status === 'scheduled' ? 'info' : 'secondary'}>
                      {session.status}
                    </Badge>
                  </div>
                </Card>
              ))}
            </div>
          )}

          {activeTab === 'assessments' && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="font-medium text-ink">Linked Assessments</h3>
                <Button size="small" onClick={() => navigate(`/assessments?groupId=${selectedGroup.id}`)}>
                  <FileText size={16} className="mr-2" /> Link Assessment
                </Button>
              </div>
              {MOCK_ASSESSMENTS.map((assessment) => (
                <Card key={assessment.assessment_id} padding="normal">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="p-2 bg-accent-100 rounded-lg">
                        <FileText size={20} className="text-accent-500" />
                      </div>
                      <div>
                        <h4 className="font-medium text-ink">{assessment.title}</h4>
                        <div className="flex items-center gap-2 mt-1">
                          <Badge variant="secondary" size="small">{assessment.assessment_type}</Badge>
                          {assessment.is_group_only && (
                            <Badge variant="info" size="small">Group Only</Badge>
                          )}
                        </div>
                      </div>
                    </div>
                    <button className="text-ink-tertiary hover:text-ink text-sm">View Details</button>
                  </div>
                </Card>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Create Group Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <Card className="w-full max-w-md" padding="normal">
            <h3 className="text-lg font-semibold text-ink mb-4">Create New Group</h3>
            <form onSubmit={handleCreateGroup} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-ink mb-1">Group Name</label>
                <Input placeholder="e.g., Section A - Morning Class" required />
              </div>
              <div>
                <label className="block text-sm font-medium text-ink mb-1">Description</label>
                <textarea className="w-full px-3 py-2 bg-surface border border-line rounded-lg text-sm text-ink focus:outline-none focus:border-brand-300" rows={3} />
              </div>
              <div>
                <label className="block text-sm font-medium text-ink mb-1">Max Students</label>
                <Input type="number" placeholder="50" />
              </div>
              <div className="flex items-center justify-end gap-3 pt-4">
                <Button type="button" variant="outline" onClick={() => setShowCreateModal(false)}>
                  Cancel
                </Button>
                <Button type="submit">Create Group</Button>
              </div>
            </form>
          </Card>
        </div>
      )}

      {/* Add Student Modal */}
      {showAddStudentModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <Card className="w-full max-w-md" padding="normal">
            <h3 className="text-lg font-semibold text-ink mb-4">Add Student to Group</h3>
            <form onSubmit={handleAddStudent} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-ink mb-1">Student Email or ID</label>
                <Input placeholder="student@university.ac.ke" required />
              </div>
              <div>
                <label className="block text-sm font-medium text-ink mb-1">Notes (Optional)</label>
                <textarea className="w-full px-3 py-2 bg-surface border border-line rounded-lg text-sm text-ink focus:outline-none focus:border-brand-300" rows={2} />
              </div>
              <div className="flex items-center justify-end gap-3 pt-4">
                <Button type="button" variant="outline" onClick={() => setShowAddStudentModal(false)}>
                  Cancel
                </Button>
                <Button type="submit">Add Student</Button>
              </div>
            </form>
          </Card>
        </div>
      )}
    </div>
  );
}
