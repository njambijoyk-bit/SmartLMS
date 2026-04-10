import { useAuth } from '../../context/AuthContext';
import { AdminDashboard } from './AdminDashboard';
import { InstructorDashboard } from './InstructorDashboard';
import { LearnerDashboard } from './LearnerDashboard';

export function DashboardPage() {
  const { user } = useAuth();

  if (!user) return null;

  switch (user.role) {
    case 'admin':
      return <AdminDashboard />;
    case 'instructor':
      return <InstructorDashboard />;
    case 'learner':
      return <LearnerDashboard />;
    default:
      return <AdminDashboard />;
  }
}
