import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './context/AuthContext';
import { DashboardLayout } from './components/layout/DashboardLayout';
import { LoginPage } from './pages/auth/LoginPage';
import { SetupWizard } from './pages/onboarding/SetupWizard';
import { DashboardPage } from './pages/dashboard/DashboardPage';
import { CoursesPage } from './pages/courses/CoursesPage';
import { CourseDetailPage } from './pages/courses/CourseDetailPage';
import { GradebookPage } from './pages/gradebook/GradebookPage';
import { UsersPage } from './pages/users/UsersPage';
import { AssessmentsPage } from './pages/assessments/AssessmentsPage';
import { AnalyticsPage } from './pages/analytics/AnalyticsPage';
import { MessagesPage } from './pages/messages/MessagesPage';
import { TimetablePage } from './pages/timetable/TimetablePage';
import { LibraryPage } from './pages/library/LibraryPage';
import { LiveClassesPage } from './pages/live/LiveClassesPage';
import { AttendancePage } from './pages/attendance/AttendancePage';
import { NotificationsPage } from './pages/notifications/NotificationsPage';
import { RegistrationPage } from './pages/registration/RegistrationPage';
import { FeesPage } from './pages/fees/FeesPage';
import { ExamCardsPage } from './pages/examcards/ExamCardsPage';
import { CertificatesPage } from './pages/certificates/CertificatesPage';
import { SettingsPage } from './pages/settings/SettingsPage';
import { PlaceholderPage } from './pages/PlaceholderPage';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated } = useAuth();
  if (!isAuthenticated) return <Navigate to="/login" replace />;
  return <>{children}</>;
}

function AppRoutes() {
  const { isAuthenticated } = useAuth();

  return (
    <Routes>
      <Route path="/login" element={isAuthenticated ? <Navigate to="/dashboard" replace /> : <LoginPage />} />
      <Route path="/setup" element={<SetupWizard />} />
      <Route
        element={
          <ProtectedRoute>
            <DashboardLayout />
          </ProtectedRoute>
        }
      >
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="/courses" element={<CoursesPage />} />
        <Route path="/courses/:id" element={<CourseDetailPage />} />
        <Route path="/gradebook" element={<GradebookPage />} />
        <Route path="/users" element={<UsersPage />} />
        <Route path="/assessments" element={<AssessmentsPage />} />
        <Route path="/analytics" element={<AnalyticsPage />} />
        <Route path="/messages" element={<MessagesPage />} />
        <Route path="/timetable" element={<TimetablePage />} />
        <Route path="/library" element={<LibraryPage />} />
        <Route path="/live" element={<LiveClassesPage />} />
        <Route path="/attendance" element={<AttendancePage />} />
        <Route path="/notifications" element={<NotificationsPage />} />
        <Route path="/registration" element={<RegistrationPage />} />
        <Route path="/fees" element={<FeesPage />} />
        <Route path="/exam-cards" element={<ExamCardsPage />} />
        <Route path="/certificates" element={<CertificatesPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/wellbeing" element={<PlaceholderPage />} />
        <Route path="/automation" element={<PlaceholderPage />} />
        <Route path="/institution" element={<PlaceholderPage />} />
      </Route>
      <Route path="*" element={<Navigate to="/login" replace />} />
    </Routes>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <AppRoutes />
      </AuthProvider>
    </BrowserRouter>
  );
}
