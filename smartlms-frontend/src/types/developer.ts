// Phase 16 & 17 API Types

export interface VPATReport {
  id: string;
  productName: string;
  productVersion: string;
  reportDate: string;
  contactName: string;
  contactEmail: string;
  wcagLevel: 'A' | 'AA' | 'AAA';
  overallScore: number;
  criteria: VPATCriterion[];
  status: 'draft' | 'completed' | 'published';
}

export interface VPATCriterion {
  id: string;
  criterion: string;
  standard: 'WCAG_2_1_A' | 'WCAG_2_1_AA' | 'WCAG_2_1_AAA' | 'SECTION_508' | 'EN_301_549';
  level: 'A' | 'AA' | 'AAA';
  conformance: 'supports' | 'partially_supports' | 'does_not_support' | 'not_applicable';
  notes: string;
  remediation?: string;
}

export interface OAuthApplication {
  id: string;
  name: string;
  clientId: string;
  clientSecret?: string;
  redirectUris: string[];
  grantTypes: string[];
  scopes: string[];
  logo?: string;
  description?: string;
  website?: string;
  status: 'active' | 'inactive' | 'revoked';
  createdAt: string;
  updatedAt: string;
}

export interface OAuthToken {
  id: string;
  applicationId: string;
  userId: string;
  scopes: string[];
  expiresAt: string;
  createdAt: string;
  revoked: boolean;
}

export interface MarketplaceApp {
  id: string;
  name: string;
  developerId: string;
  developerName: string;
  description: string;
  category: string;
  version: string;
  price: number;
  currency: string;
  logo?: string;
  screenshots: string[];
  rating: number;
  reviewCount: number;
  installCount: number;
  status: 'pending' | 'approved' | 'rejected' | 'suspended';
  createdAt: string;
  updatedAt: string;
}

export interface SDKConfig {
  language: 'typescript' | 'python' | 'java' | 'csharp' | 'go' | 'php';
  packageName: string;
  version: string;
  includeExamples: boolean;
  includeTests: boolean;
  apiVersion: string;
}

export interface SDKPackage {
  id: string;
  language: string;
  packageName: string;
  version: string;
  downloadUrl: string;
  size: number;
  checksum: string;
  createdAt: string;
}

export interface APIAnalytics {
  totalRequests: number;
  successfulRequests: number;
  failedRequests: number;
  averageLatency: number;
  p95Latency: number;
  p99Latency: number;
  requestsByEndpoint: EndpointStats[];
  requestsByUser: UserStats[];
  requestsByTime: TimeSeriesData[];
  errorRates: ErrorRateData[];
}

export interface EndpointStats {
  endpoint: string;
  method: string;
  count: number;
  averageLatency: number;
  errorRate: number;
}

export interface UserStats {
  userId: string;
  userName: string;
  requestCount: number;
  quotaUsed: number;
  quotaLimit: number;
}

export interface TimeSeriesData {
  timestamp: string;
  count: number;
  averageLatency: number;
}

export interface ErrorRateData {
  errorCode: number;
  count: number;
  percentage: number;
}

export interface RateLimitConfig {
  requestsPerMinute: number;
  requestsPerHour: number;
  requestsPerDay: number;
  burstLimit: number;
}

export interface APIQuota {
  userId: string;
  plan: string;
  dailyLimit: number;
  monthlyLimit: number;
  dailyUsed: number;
  monthlyUsed: number;
  resetDate: string;
}

export interface LMSMigration {
  id: string;
  sourceSystem: 'moodle' | 'canvas' | 'blackboard' | 'other';
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  coursesCount: number;
  usersCount: number;
  assessmentsCount: number;
  progress: number;
  errors: MigrationError[];
  startedAt: string;
  completedAt?: string;
}

export interface MigrationError {
  type: string;
  message: string;
  item: string;
  severity: 'warning' | 'error' | 'critical';
}

export interface SOC2Control {
  id: string;
  controlId: string;
  category: 'security' | 'availability' | 'confidentiality' | 'processing_integrity' | 'privacy';
  name: string;
  description: string;
  status: 'not_started' | 'in_progress' | 'implemented' | 'verified';
  evidence: string[];
  lastReviewed: string;
  nextReview: string;
  owner: string;
}

export interface ComplianceDashboard {
  overallScore: number;
  controlsByCategory: CategoryBreakdown[];
  recentAudits: AuditRecord[];
  upcomingReviews: ReviewItem[];
  riskAssessments: RiskAssessment[];
}

export interface CategoryBreakdown {
  category: string;
  implemented: number;
  total: number;
  percentage: number;
}

export interface AuditRecord {
  id: string;
  date: string;
  auditor: string;
  findings: number;
  status: 'passed' | 'conditional' | 'failed';
  reportUrl?: string;
}

export interface ReviewItem {
  controlId: string;
  controlName: string;
  dueDate: string;
  owner: string;
  priority: 'low' | 'medium' | 'high' | 'critical';
}

export interface RiskAssessment {
  id: string;
  title: string;
  riskLevel: 'low' | 'medium' | 'high' | 'critical';
  likelihood: number;
  impact: number;
  mitigation: string;
  status: 'open' | 'mitigated' | 'accepted';
}
