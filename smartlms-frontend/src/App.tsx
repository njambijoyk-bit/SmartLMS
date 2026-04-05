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
        <Route path="/assessments" element={<PlaceholderPage />} />
        <Route path="/live" element={<PlaceholderPage />} />
        <Route path="/messages" element={<PlaceholderPage />} />
        <Route path="/registration" element={<PlaceholderPage />} />
        <Route path="/attendance" element={<PlaceholderPage />} />
        <Route path="/fees" element={<PlaceholderPage />} />
        <Route path="/exam-cards" element={<PlaceholderPage />} />
        <Route path="/library" element={<PlaceholderPage />} />
        <Route path="/timetable" element={<PlaceholderPage />} />
        <Route path="/analytics" element={<PlaceholderPage />} />
        <Route path="/certificates" element={<PlaceholderPage />} />
        <Route path="/wellbeing" element={<PlaceholderPage />} />
        <Route path="/automation" element={<PlaceholderPage />} />
        <Route path="/institution" element={<PlaceholderPage />} />
        <Route path="/notifications" element={<PlaceholderPage />} />
        <Route path="/settings" element={<PlaceholderPage />} />
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
