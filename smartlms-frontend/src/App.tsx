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
import { WellbeingPage } from './pages/wellbeing/WellbeingPage';
import { AutomationPage } from './pages/automation/AutomationPage';
import { InstitutionPage } from './pages/institution/InstitutionPage';
import { DiscussionForumsPage } from './pages/forums/DiscussionForumsPage';
import { ParentsPortalPage } from './pages/parents/ParentsPortalPage';
import { ClearancePage } from './pages/clearance/ClearancePage';
import { AlumniPortalPage } from './pages/alumni/AlumniPortalPage';
import { PortfolioPage } from './pages/portfolio/PortfolioPage';
import { AdvisingPage } from './pages/advising/AdvisingPage';
import { CompetencyPage } from './pages/competency/CompetencyPage';
import { ExamBankPage } from './pages/exambank/ExamBankPage';
import { IDCardsPage } from './pages/idcards/IDCardsPage';
import { BadgesPage } from './pages/badges/BadgesPage';
import { ResearchPage } from './pages/research/ResearchPage';
import { PeerReviewPage } from './pages/peerreview/PeerReviewPage';
import { EmployerPortalPage } from './pages/employer/EmployerPortalPage';
import { RPLPage } from './pages/rpl/RPLPage';
import { ProctoringPage } from './pages/proctoring/ProctoringPage';
import DeveloperPage from './pages/developer';
import AccessibilityPage from './pages/accessibility';
import BlockchainPage from './pages/blockchain/BlockchainPage';
import IoTPage from './pages/iot/IoTPage';

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
        <Route path="/wellbeing" element={<WellbeingPage />} />
        <Route path="/automation" element={<AutomationPage />} />
        <Route path="/institution" element={<InstitutionPage />} />
        <Route path="/forums" element={<DiscussionForumsPage />} />
        <Route path="/parents" element={<ParentsPortalPage />} />
        <Route path="/clearance" element={<ClearancePage />} />
        <Route path="/alumni" element={<AlumniPortalPage />} />
        <Route path="/portfolio" element={<PortfolioPage />} />
        <Route path="/advising" element={<AdvisingPage />} />
        <Route path="/competency" element={<CompetencyPage />} />
        <Route path="/exam-bank" element={<ExamBankPage />} />
        <Route path="/id-cards" element={<IDCardsPage />} />
        <Route path="/badges" element={<BadgesPage />} />
        <Route path="/research" element={<ResearchPage />} />
        <Route path="/peer-review" element={<PeerReviewPage />} />
        <Route path="/employer" element={<EmployerPortalPage />} />
        <Route path="/rpl" element={<RPLPage />} />
        <Route path="/proctoring" element={<ProctoringPage />} />
        <Route path="/developer" element={<DeveloperPage />} />
        <Route path="/accessibility" element={<AccessibilityPage />} />
        <Route path="/blockchain" element={<BlockchainPage />} />
        <Route path="/iot" element={<IoTPage />} />
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
