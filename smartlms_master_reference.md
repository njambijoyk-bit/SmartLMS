# SmartLMS Engine — Master Reference
> v0.2 — Planning & Design Phase  
> Last updated: April 2026  
> Status: Pre-build. All decisions subject to revision during implementation.

---

## Table of Contents

1. [Product Vision](#1-product-vision)
2. [Architecture](#2-architecture)
3. [Tech Stack](#3-tech-stack)
4. [Core Modules (1–13)](#4-core-modules-1-13)
5. [Academic Features](#5-academic-features)
6. [Extended Modules (14–24)](#6-extended-modules-14-24)
7. [Julia AI/ML Engine — Full Architecture](#7-julia-aiml-engine--full-architecture)
8. [Security Architecture](#8-security-architecture)
9. [Backup System](#9-backup-system)
10. [Packaging & Distribution](#10-packaging--distribution)
11. [Pricing Model](#11-pricing-model)
12. [Competitive Positioning](#12-competitive-positioning)
13. [Community & Ecosystem](#13-community--ecosystem)
14. [Migration Tooling](#14-migration-tooling)
15. [Enterprise Compliance & Trust](#15-enterprise-compliance--trust)
16. [Partner Programme](#16-partner-programme)
17. [Business Continuity Policy](#17-business-continuity-policy)
18. [Build Sequence](#18-build-sequence)
19. [Full Module Map](#19-full-module-map)

---

## 1. Product Vision

### In one sentence
SmartLMS is a complete Learning Management System engine built once, packaged and sold to institutions worldwide. Each institution installs it on their own server, runs it fully branded as their own product, and manages their own students and data. You never host their data. You just keep improving the engine.

### The model in one analogy
You are building the engine that powers other people's LMS products. Like how Shopify powers thousands of online stores — each store looks completely different, the store owner manages their own products and customers — but Shopify is the engine underneath. Except in your model, the store owner hosts it themselves on their own server. Your costs stay near zero as you scale to any number of institutions.

### Core model — locked

| Decision | Choice |
|---|---|
| Deployment model | Hybrid — institutions self-host, you distribute and upgrade centrally |
| Your role | Engine publisher, licence holder, update distributor — not a hosting company |
| Student data | Never leaves the institution's server — permanently |
| Student logins | Managed by each institution's running engine instance |
| Your infra cost | Near zero — 3–4 small VMs total, regardless of institution count |
| Target market | Global — all institution types, all sizes. Universities, K-12, corporate, EdTech, bootcamps |

---

## 2. Architecture

### Ownership split

**You own and pay for:**
- Update server — distributes new engine versions to all institutions
- Licence server — validates keys, gates features by plan tier
- CDN — hosts JS SDK, theme configs, frontend assets, institution logos
- Julia ML service — optional central AI tier (premium institutions only)
- Telemetry receiver — anonymised signals from opted-in institutions
- Developer portal / Community Hub — docs, SDK, marketplace, changelog, forum

**Institution owns and pays for:**
- Engine instance — the Rust binary running on their server
- Their database — all student, course, and grade data
- Their domain — lms.uon.ac.ke, learn.strathmore.edu
- Their file storage — videos, PDFs, assignment uploads
- Their server bill — VPS, cloud, or on-premise hardware

### Multi-tenant router

Every inbound HTTP request carries the institution's identity in the `Host` header. The Rust router resolves this into a full institution context before any business logic runs.

| Component | Detail |
|---|---|
| Slug extraction | Host header read on every request. Custom domain `lms.uon.ac.ke` → lookup in domain_map → slug. Subdomain `univabc.smartlms.io` → extracted directly. |
| Hot cache | `DashMap<String, InstitutionCtx>` — concurrent hashmap, no mutex on reads. 60 second TTL. Event-driven invalidation on config change. |
| Cache miss path | DashMap miss → Redis lookup → master DB query → write back to both. Rare after warmup. |
| DB isolation | Each institution gets its own `PgPool`. Handlers never share a pool. Cross-institution data access is architecturally impossible. |
| Handler injection | `InstitutionCtx` injected as Axum Extension on every request. Handlers extract it — never query config themselves. |
| Context contains | Institution ID, slug, db_pool, theme config, feature flags, plan tier, quota limits |
| Not found | Unknown slug or domain returns 404 immediately — no DB query attempted |

```rust
#[derive(Clone)]
pub struct InstitutionCtx {
    pub id: Uuid,
    pub slug: String,
    pub db_pool: PgPool,
    pub config: InstitutionConfig,
    pub plan: PlanTier,
}

pub struct RouterState {
    pub cache: DashMap<String, InstitutionCtx>,
    pub domain_map: DashMap<String, String>,
    pub redis: ConnectionManager,
    pub master_pool: PgPool,
}
```

### Feature flag and plugin system

| Rule | Detail |
|---|---|
| How features gate | Every module lives behind a feature flag. Licence key determines which flags are on. Plan upgrade = new key = new flags = features appear on next restart. |
| New module rollout | Built and tested → migration scripts written → packaged into new engine version → pushed to update server → institutions receive automatically. |
| Migration safety | Migrations only ADD new tables and columns — never rename or delete without a deprecation window. Rollback is always safe. |
| Optional modules | MongoDB and Julia ML are off by default. Institution adds connection string to config, engine detects and activates. |

### Institution server scaling

| Stage | Setup |
|---|---|
| Small school (< 1k students) | Single $10–20/month VPS. Engine + Redis + DB all on one machine. |
| Growing institution (1k–10k) | Separate DB server. Engine on its own VM. Redis alongside engine. Separate file storage bucket. |
| Large university (10k–50k) | 2–3 engine instances behind load balancer. Dedicated DB with read replica. Dedicated Redis. CDN for file delivery. |
| Your servers | Do not change. 3–4 VMs whether you have 5 institutions or 5,000. |

### Telemetry — opt-in, zero PII

**Usage counts:** Active learners per day, total logins, courses created, assignments submitted, live sessions held, storage used.

**Behaviour patterns:** Where learners drop off in courses, unused features, average session length, quiz retry rates. Feeds Julia model improvement across the network — anonymised and aggregated only.

**Rule:** Institution admin sees exactly what is being sent — full transparency panel in their dashboard. They can pause telemetry at any time. No student name, email, or identifier ever leaves their server.

---

## 3. Tech Stack

### Languages

| Language | Role | Frameworks |
|---|---|---|
| **Rust** | Engine core — HTTP server, routing, business logic, auth, DB queries, all APIs | Axum, sqlx |
| **Julia** | AI and ML tier — adaptive learning, dropout prediction, content recommendation, NLP auto-grading, voice analysis, agentic actions. Runs as a separate HTTP microservice. | HTTP microservice, Flux.jl, MLJ.jl, Turing.jl |
| **TypeScript** | JS SDK, React frontend components, developer SDK npm package, API client types | Strict mode, no `any` |

### Frontend

| Library | Role |
|---|---|
| React 18 | The LMS UI served by the engine. Built once, white-labelled per institution via CSS variable injection. Ships as a PWA by default. |
| Vite | Build tooling and dev server |
| PWA | Built-in via service worker — students install from browser, works offline for cached content |

### Databases — three-layer strategy

| Database | Type | Role | Required? |
|---|---|---|---|
| MySQL 8 or PostgreSQL | Relational | Core data — students, courses, grades, enrolments, transactions, audit logs. Institution picks one on install. Engine abstracts over both via sqlx. | Required |
| MongoDB | Document store | Course builder blocks, discussion threads, notification logs, activity feeds. Engine falls back to JSON columns in relational DB if absent. | Optional |
| Redis | In-memory | DashMap cache backing store, session storage, real-time pub/sub, rate limiting, leaderboard counters, telemetry buffer | Required |

### APIs exposed by the engine

| API | Purpose |
|---|---|
| REST | Primary API. Used by mobile apps, JS SDK, third-party integrations. JSON responses. JWT auth. Rate limited per plan. |
| GraphQL | Developer tier. Full schema — courses, learners, grades, events. Cursor pagination. Introspectable playground. |
| WebSocket | Real-time — live class events, messaging notifications, leaderboard updates, live CAT timer sync, forum posts. |

---

## 4. Core Modules (1–13)

| # | Module | What it does | Stack | Phase |
|---|---|---|---|---|
| 1 | Engine foundation | Multi-tenant router, per-institution DB provisioning, upgrade service, dynamic config routing | Rust, React | 1 |
| 2 | JavaScript SDK | 3-line embed snippet, Web Components, React library, iframe mode with postMessage bridge, event hooks | TypeScript, React | 2 |
| 3 | White-label system | CSS variable injection, logo/favicon hosting, custom domain with auto-TLS, white-labelled emails, locale per institution | React, TypeScript, CDN | 3 |
| 4 | Institution onboarding | Self-serve signup, guided setup wizard, plan and quota management, 14-day sandbox with sample data, admin invite flow | Rust, React | 4 |
| 5 | Users & roles | Role system (Admin/Instructor/Learner/Observer/Parent/Advisor/Counsellor/Alumni), SSO via Google/Microsoft/SAML, custom profile fields, groups and cohorts, bulk CSV import, soft delete with audit trail | Rust, React | 5 |
| 6 | Courses & content | Drag-drop course builder, rich block editor, video upload and HLS transcoding, SCORM 1.2/2004 import, prerequisite gates, draft/published versioning, guided instructional design mode | Rust, React, MongoDB | 6 |
| 7 | Assessments | Question bank, randomised CATs, deadline assignments, rubric builder, weighted gradebook, plagiarism detection, transcript generation, Exam Bank | Rust, React | 7 |
| 8 | Live classes | Session scheduling, Zoom/Meet/Jitsi integration, auto-recording and transcript, attendance tracking, breakout rooms, collaborative whiteboard | Rust, React, WebSocket | 8 |
| 9 | Communication | Announcements, direct messaging, discussion forums, notification centre, push notifications via Web Push, white-labelled email templates | Rust, React, WebSocket | 9 |
| 10 | Julia AI/ML engine | Full intelligence layer — see Section 7 | Julia, Rust bridge | 10 — Premium |
| 11 | Automation engine | Visual rule builder (IF/THEN, no code), 30+ triggers, action library, certificate issuer with QR verification, gamification (XP, badges, leaderboards), outbound webhooks | Rust, React | 11 |
| 12 | Analytics & reporting | Learner dashboard, course analytics, cohort comparison, instructor performance metrics, drag-drop custom report builder, xAPI/Tin Can export to LRS | Rust, React | 12 |
| 13 | Billing & finance | Institution subscription billing, learner payment gateway (Stripe + M-Pesa + bank transfer), promo codes, auto PDF invoices, instructor revenue share, finance dashboard | Rust, React, Stripe, M-Pesa | 13 |

---

## 5. Academic Features

### Courses

| Feature | Detail |
|---|---|
| Course structure | Units → Modules → Lessons. Each lesson is video, reading, PDF, quiz, or assignment. Instructor configures free navigation or strict sequencing. |
| Prerequisite gates | Lesson or module locked until student completes or scores above a threshold on a prior one. |
| Enrolment modes | Self-enrol, instructor-enrol, admin-enrol, payment-gated, cohort enrolment. |
| Versioning | Draft/published states. Full version history per lesson. Roll back to any prior version. |
| Instructional design mode | Guided wizard: define learning outcomes → map assessments → build content → choose structure template. Julia checks alignment throughout. |

### CATs — Continuous Assessment Tests

| Feature | Detail |
|---|---|
| Question bank | Instructor builds a large bank tagged by topic and difficulty. Engine draws from it per student automatically. |
| Unique paper per student | Different random subset for each student. Sharing answers is useless. |
| Shuffled answer options | MCQ answer choices randomised per student per attempt. |
| Server-side timer | Countdown enforced on the server. Auto-submits when time expires. |
| Scheduled windows | CAT is invisible to students until the configured open time. |
| Tab / focus tracking | Every tab switch and window minimise logged with timestamp. |
| Copy-paste disabled | Right-click and keyboard copy shortcuts disabled on the CAT page. |
| IP and device logging | Every submission records IP address and device fingerprint. |
| Essay similarity detection | Flags high-similarity pairs for instructor review. |
| Full audit trail | Per student: start time, every answer change, submission time, focus-loss events, time spent per question. |

### Grade Management

| Feature | Detail |
|---|---|
| Weighted categories | CATs 30%, Assignments 20%, Final Exam 50% — fully configurable. |
| Grade scales | Letter grades A–F, GPA 4.0, percentage only, or custom. |
| Transcripts | Student downloads a branded PDF transcript. Can require admin approval. |
| Grade override + audit | Full audit log — who changed it, from what to what, when, and why. |

### Deadline-Based Assignments

| Feature | Detail |
|---|---|
| Submission windows | Open date and due date per assignment. Form closes at deadline automatically. |
| Late submission rules | No late / late with % penalty per day / late until hard cutoff. |
| Extension requests | Student requests extension. Instructor approves or denies. Affects only that student. |
| Submission types | File upload, text entry, URL, video recording, code submission with syntax highlighting. |

---

## 6. Extended Modules (14–24)

### Module 14 — Library

A centralised, searchable, institution-scoped digital resource repository.

**Data model:**
```
Collection (e.g. "Engineering Resources", "Past Papers")
  └── Resource
        ├── title, description, tags[]
        ├── resource_type (PDF, video, link, EPUB, dataset)
        ├── file_ref → institution's own file storage
        ├── access_level (public, enrolled, role-gated)
        ├── linked_courses[]
        └── metadata { author, publisher, year, ISBN, DOI }
```

**Key features:**
- Collections with role-based access control (enforced at API level)
- Course linking — resources appear in course sidebar
- Citation export (APA, MLA, Chicago) auto-generated from metadata
- Bulk upload (ZIP → auto-unpack → batch metadata editor)
- Full-text search via PostgreSQL `tsvector`
- Physical borrow/reservation queue for institutions with physical items
- Usage analytics per resource per course
- OPDS feed (Enterprise tier — standard library catalog protocol)

---

### Module 15 — Exam Bank

A management layer above the CAT engine for full past papers, structured exam templates, and institutional exam archiving.

**Data model:**
```rust
pub struct ExamPaper {
    pub id: Uuid,
    pub title: String,
    pub course_id: Uuid,
    pub academic_year: String,
    pub semester: u8,
    pub paper_type: PaperType,  // CAT | MidSem | EndSem | Supplementary
    pub status: PaperStatus,    // Draft | Approved | Archived | PublishedToStudents
    pub sections: Vec<ExamSection>,
    pub marking_scheme: Option<Document>,
    pub approval_flow: ApprovalFlow,
    pub access_log: Vec<AccessRecord>,
}
```

**Key features:**
- Multi-stage approval workflow (created → reviewed → approved → locked)
- Version control — year-on-year paper history
- Security classification: Restricted / Confidential / Public
- Student practice mode — past papers published as self-paced timed practice
- QR verification for invigilators (offline-capable, works without network in exam hall)
- Exam timetabling — link paper to scheduled slot, students see on dashboard

**Relationship to CAT engine:**
The Exam Bank is the management/approval layer. The CAT engine is the delivery layer underneath. A formal end-of-semester sit activates through the CAT engine with all integrity features applied.

---

### Module 16 — Student Registration System

A fully configurable intake pipeline that sits before account creation.

**Registration modes:**

| Mode | How it works | Best for |
|---|---|---|
| Open self-registration | Student fills form, verifies email, account active immediately | Bootcamps, MOOCs |
| Admin-approval | Student submits form, sits in pending queue, admin approves/rejects | Most universities |
| Application-based | Multi-step form — personal info, documents, references, statement of purpose | Postgrad, professional certs |
| Invite-only | Admin generates single-use invite links. No public registration form exists. | Corporate, government |
| Bulk CSV import | Admin uploads spreadsheet — engine creates accounts and sends welcome emails | Migrations |
| Payment-gated | Student pays registration fee first. Payment confirmed → account created | Private institutions |
| Parent-initiated | Parent creates account, registers their child. Accounts linked automatically. | K-12 |
| SSO-only | No password registration. Account created on first SSO login. | Corporate, enterprise |

**Registration form builder:**
- Built-in fields: name, email, phone, DOB, gender, nationality, photo
- Optional standard fields: National ID, KRA PIN (Kenya), next of kin, previous institution, programme, year of study
- Custom fields: text, number, date, dropdown, checkbox, file upload — each configurable as required/optional/admin-only

**Approval workflow:**
```
Student submits → [Status: Pending] → Approve/Reject
                                          │
                              Multi-stage option:
                    Admissions → Department Head → Finance → Active
```

**Student ID generation:**
```toml
[registration.student_id]
format = "{YEAR}{PROGRAMME_CODE}{SEQUENCE}"
sequence_padding = 4
sequence_reset = "yearly"
```

**Registration period config:**
```toml
[registration]
mode = "admin_approval"
open_date = "2024-09-01T00:00:00"
close_date = "2024-09-30T23:59:59"
allow_late_registration = true
late_fee_enabled = true
late_fee_amount = 500
waitlist_enabled = true
max_enrolments = 5000
```

---

### Module 17 — Exam Cards

A formal document authorising a student to sit an examination, issued only when all pre-conditions are met.

**Pre-conditions engine (configurable per institution):**
```toml
[exam_card]
enabled = true

[[exam_card.conditions]]
type = "fee_balance"
operator = "less_than_or_equal"
value = 0

[[exam_card.conditions]]
type = "attendance_percentage"
operator = "greater_than_or_equal"
value = 75

[[exam_card.conditions]]
type = "coursework_submission"
operator = "greater_than_or_equal"
value = 80

[[exam_card.conditions]]
type = "registration_status"
value = "approved"
```

**Issuance modes:**
- Auto-issue — engine checks conditions daily and issues automatically when met
- Manual issue — finance/admin office reviews and issues each card
- Self-service with gate — student requests, engine checks conditions in real-time

**Card PDF content:**
- Institution logo and branding (white-label)
- Student photo, name, registration number, programme, year
- Registered units with room assignments
- Card number, validity period
- QR code (cryptographically signed payload)
- Institution seal

**QR verification (invigilator):**
```json
{
  "card_id": "EC-2024-0042",
  "student_id": "uuid...",
  "valid_until": "2024-11-29",
  "sig": "RS256 signature by institution private key"
}
```
Verification works **offline** — invigilator's device caches the day's valid cards at the start of the exam period.

**Partial clearance / grace allowance:**
```toml
[[exam_card.conditions]]
type = "fee_balance"
operator = "less_than_or_equal"
value = 5000
allow_undertaking = true
undertaking_expires_days = 30
```

---

### Module 18 — Parents Portal

A separate portal for parents/guardians with its own authentication and configurable data access.

**Linkage model:**
```
Parent account → linked to → Student account(s)
                               └── institution enforces visibility
```

**Linkage methods:**
- Admin-managed CSV (K-12 default)
- Self-service with student approval (university — student can revoke anytime)
- Registration-time capture (parent details captured during student registration)

**Configurable visibility per data category:**

| Data Category | Configurable | Default |
|---|---|---|
| Enrolled courses | Yes | On |
| Grades and results | Yes | On |
| Attendance records | Yes | On |
| Exam timetable and exam card | Yes | On |
| Fee balance and payment history | Yes | On |
| Disciplinary records | Yes | Off |
| Detailed coursework submissions | Yes | Off |
| Direct messaging with instructors | Yes | Off |

**All toggles enforced at API level — not just hidden in UI.**

**Parent notifications (tiered):**
- Critical (always on): exam card issued/blocked, fee payment received, emergency absence
- Important (on by default, mutable): end of semester results, attendance warning, missed deadline
- Optional (off by default): every grade posted, every attendance marked

**Fee payment from parent portal:**
- M-Pesa STK push (Kenya — first-class, not an afterthought)
- Stripe (card payments, international)
- Bank transfer reference generation
- Downloadable receipts and payment history

**Privacy (adult students):**
- Student grants parental access — it is a privilege, not a right
- Student can hide individual courses from parent view
- Revocation takes effect immediately
- All parent data access logged in audit trail
- For minors (K-12): institution manages access, student cannot revoke

---

### Module 19 — Attendance System

**QR-based attendance (recommended):**
- Instructor opens a session, engine generates a time-limited QR (valid 10 minutes)
- Students scan with phone → attendance marked
- QR expires → late students marked late, absent marked absent
- Duplicate scan detection (proxy scanning prevention)

**Manual roster:**
- Instructor opens class, sees student list, taps present/absent/late

**NFC/biometric hook:**
- Engine exposes a webhook endpoint — third-party biometric/NFC hardware posts attendance events
- Engine records them as authoritative attendance marks
- Engine is the receiver, not the hardware

**Attendance data feeds into:**
- Exam card eligibility check (module 17)
- Parent portal panel (module 18)
- Julia at-risk detection (module 10)
- Analytics and reporting (module 12)

---

### Module 20 — Fee Management System

**Fee structure builder:**
```
Per programme per year:
  ├── Tuition Fee: KES 45,000 / semester
  ├── Registration Fee: KES 2,000 / year (once)
  ├── Library Levy: KES 1,500 / semester
  ├── ICT Levy: KES 1,000 / semester
  ├── Caution Money: KES 3,000 (refundable)
  └── Hostel Fee: KES 18,000 / semester (optional)
```

**Instalment plans:** Pay in full / 2 instalments / 3 instalments. Each instalment has a due date and late payment penalty. Engine tracks balance per instalment.

**Payment channels:**
- M-Pesa (STK Push + Paybill auto-reconciliation) — primary for Kenyan market
- Stripe (card payments, international students)
- Bank transfer (engine generates unique reference per student)
- Cash (finance office marks manually with receipt number)

**Bursaries and scholarships:** Admin creates bursary with coverage percentage, applied automatically to student accounts, reported separately in finance reports.

**Reports:**
- Outstanding debtors list
- Revenue by fee type by semester
- Scholarship/bursary expenditure
- Daily collection summary
- Aged debt analysis (30/60/90/180 days)

---

### Module 21 — Clearance System

Critical for East African universities. Student must get clearance from multiple departments before receiving their certificate/transcript.

**Configurable departments:**
```
├── Library — "No outstanding books or fines"
├── Finance — "Zero fee balance"
├── Hostel — "Room vacated and deposit refunded"
├── Examination office — "All results processed"
├── Department — "Project submitted and approved"
└── Student welfare — "No disciplinary matters pending"
```

**Clearance Officer role:** Sees all students pending clearance in their department. Marks cleared/blocked with reason. Generates department clearance certificate.

**Student view:** Dashboard checklist — red/green per department. All green → download full clearance certificate (PDF with digital signature). Required to collect degree certificate.

---

### Module 22 — Timetable & Scheduling System

**Physical timetable:**
- Admin creates rooms with capacity and equipment tags
- Instructor requests a time slot for a course
- Engine detects conflicts: room double-booked, instructor clash, student cohort clash
- Admin approves and publishes — students see only their enrolled courses
- Export as iCal for Google Calendar/Apple Calendar sync

**Exam timetabling:**
- Admin slots exam papers from the Exam Bank into time slots and rooms
- Engine checks: room capacity ≥ students sitting this paper
- Engine detects student clashes (same student, two exams same time)
- Published timetable auto-populates exam card
- Changes trigger push notifications to affected students and parents

---

### Module 23 — Student & Alumni ID Cards

**Student ID card (permanent):**
- Generated at registration approval
- QR code links to real-time verified profile (confirms enrolment is still active)
- Digital version in student portal and parent portal
- Print-ready PDF for institution's card printer
- Auto-invalidates when student is deactivated/graduated

**Card transitions:**
```
Active Student → [Graduation] → Alumni
Active Student → [Withdrawal] → Former Student (limited access)
Active Student → [Suspension] → Suspended (read-only, no submissions)
```

---

### Module 24 — Alumni Portal

**Alumni account capabilities:**
- Download transcripts and certificates permanently
- Access alumni-only CPD and professional development courses
- Network directory — find graduates by programme/graduation year
- Job board — institution posts opportunities, employers post listings
- Optional: donate to institution (connects to billing module)
- Update employment status (feeds Julia's graduate outcome tracking)

**Institution benefits:**
- Graduate outcome data for accreditation reports
- Continuing education revenue stream
- Alumni engagement metric for rankings submissions

---

### Module 25 — Discussion Forums (Expanded from Module 9)

**Data model** (MongoDB primary, JSON columns fallback):
```
Course → Forum (1:1 or 1:many per unit)
Forum → Thread (many)
Thread → Post (many, tree structure via parent_post_id — max 2 levels deep)
Post → { author, body_blocks[], attachments[], reactions{}, flags[] }
```

**Thread types:** Question (mark answer as accepted), Discussion (open-ended), Announcement (instructor-only post, reply-disabled).

**Key features:**
- Anonymous posting (optional, instructor enables — instructor always sees real identity)
- Instructor badge on every instructor post
- Pin and lock threads
- Real-time via Redis Pub/Sub → WebSocket fan-out
- Full-text search scoped to course forum (PostgreSQL `tsvector`)
- Email digest (daily, opt-in, white-labelled)
- Moderation queue — configurable flag threshold for auto-hiding
- Forum analytics — unanswered questions, instructor response time, confusion signals feeding Julia

---

### Module 26 — Competency-Based Education (CBE)

**Competency framework:**
```
Domain (e.g. "Software Engineering")
  └── Competency (e.g. "Write unit tests")
        ├── Level 1 — Novice
        ├── Level 2 — Practitioner
        └── Level 3 — Expert
              └── Mastery threshold per level (configurable %)
```

**Student view:** Visual competency map — nodes light up as mastered. Advance at your own pace. Mastery unlocks next node automatically.

**Instructor view:** Per-student gap analysis. Cohort heatmap — which competencies the class struggles with most. Map any assessment to one or more competency nodes.

**Institution config:**
- Pure CBE mode (no traditional grades)
- Hybrid mode (traditional grades + competency overlay)
- Map competencies to national qualifications frameworks (KNQF for Kenya, NQF for South Africa, etc.)

---

### Module 27 — Micro-Credentials & Digital Badges

**Credential types:**
```
├── Course completion certificate (existing — module 11)
├── Digital badge — specific skill demonstrated
├── Micro-credential — short programme completed
└── Stackable credential — accumulate badges → earn a full credential
```

**Badge anatomy:**
- Title + issuing institution (white-labelled)
- Evidence: which assessments, what scores, what date
- Competency alignment: which CBE nodes this badge covers
- Expiry (optional — for time-sensitive certifications)
- Public QR verification URL (no login required)

**Open Badges 3.0 compliance:** Industry-standard format — LinkedIn, Credly, and HR systems can import natively. Student accumulates badges in their SmartLMS wallet. One-click export to LinkedIn.

**Optional blockchain anchoring (Enterprise):** Badge hash recorded on a public blockchain (hash only — not the data). Tamper-proof even if the institution's server goes down. Particularly powerful for professional certifications and government training.

---

### Module 28 — Student Wellbeing

**Student-facing (weekly check-in, optional):**
```
"How are you doing this week?" [1–5]
"How manageable is your workload?" [1–5]
"Are you sleeping enough?" [yes/no]
```
Responses are private by default. Student sees their own trend. Self-referral to counselling available in one tap.

**Counsellor role (new):**
- Aggregate anonymised wellbeing trends per cohort
- Individual student escalation (student consents first)
- Case management: log interactions, set follow-up reminders, close cases

**Instructor-facing (aggregate only, no individual data):**
```
"Your CS301 cohort wellbeing score dropped this week"
Correlated with: assignment deadlines, exam proximity
Suggested actions: "Consider extending the deadline"
```

**Julia integration:** Dropout prediction model includes wellbeing signals. Automatic escalation when: student stops submitting + attendance drops + wellbeing score low.

```toml
[wellbeing]
enabled = true
check_in_frequency = "weekly"
mandatory = false
counsellor_role_enabled = true
```

---

### Module 29 — Academic Advising

**Advisor role capabilities:**
- Assigned caseload of students
- Dashboard: grades, attendance, wellbeing, exam card status per student
- Appointment booking system — student books from portal
- Notes per student: private, searchable, date-stamped
- Academic plan builder: maps courses for next semester
- Referral system: route to counselling, financial aid, disability services

**Student view:**
- "Your advisor is Dr. Otieno — Book an appointment"
- Academic plan showing course sequence and progress toward graduation
- Graduation audit: "You need 12 more credit hours in electives and CS401 to graduate"

---

### Module 30 — Research & Postgraduate Supervision

**Supervision relationship:**
- Supervisor role assigned to postgraduate students
- Primary supervisor + co-supervisors supported
- Research profile: title, abstract, proposal, milestone timeline, chapter submissions, ethics clearance

**Milestone tracking:**
- Each milestone has due date and supervisor sign-off requirement
- Overdue milestones trigger alerts to supervisor, student, and coordinator
- Automated escalation if no interaction for N days

**Chapter review workflow:**
- Student uploads draft → supervisor notified
- Supervisor returns with tracked-changes document + comments
- Full version history maintained — every draft saved
- Native in the engine (no dependency on Microsoft Word)

**Quarterly progress reports:**
- Student fills structured form: progress made, plan for next quarter, challenges
- Supervisor endorses with comments
- Coordinator archives — feeds department research reporting

**Viva/defence scheduling:**
- External examiner gets limited portal access (read thesis only, submit examiner report)
- Outcome recorded: Pass / Pass with minor corrections / Major corrections / Fail
- Corrections deadline tracked and enforced

---

### Module 31 — Offline-First Architecture

**What works offline (service worker + IndexedDB):**
- Watch pre-downloaded video lessons
- Read course content and PDFs
- Take quizzes (answers queued locally, sync when reconnected)
- Write assignment drafts (auto-saved locally, uploaded on reconnect)
- View grades and progress
- Read pre-cached forum threads

**What requires connection:**
- Live CAT (server-side timer cannot be faked)
- Live classes
- New forum posts
- Payment actions

**Sync engine:**
```
- Student marks content for offline download
- Background sync when on WiFi
- Conflict resolution: server wins for grades; local wins for drafts
- Storage quota management visible to student
- Instructor can mark content as offline-priority
```

**Low-bandwidth mode:**
- Auto-detects connection speed on load
- Serves text-only version of course pages
- Video replaced with transcript + audio-only track
- Images replaced with alt-text
- Student can force low-bandwidth mode manually

---

### Module 32 — Peer Learning & Structured Peer Review

**Peer review assignment type:**
1. Student submits their work
2. Engine anonymously assigns each student N peers' work to review (avoids assigning friends)
3. Structured rubric guides the review — not free text
4. Grade calculation: Student's work (70%) + quality of peer reviews given (30%)

**Calibration system:**
- Instructor grades a sample with a gold-standard rubric
- Engine compares peer ratings against gold standard
- "Generous rater" and "harsh rater" corrections applied automatically
- Students who rate randomly flagged for instructor review

**Peer tutoring marketplace (Growth/Enterprise):**
- High-performing students register as peer tutors
- Other students book sessions
- Institution sets: paid (tutors earn credits/cash) or volunteer
- Sessions conducted via the live classes module
- Tutor effectiveness tracked: did tutees improve?

---

### Module 33 — Employer & Industry Portal

**Employer account (institution-managed):**
- Job/internship posting board visible to enrolled students
- Student talent pool search by programme/graduation year/skills/GPA (with consent)
- Certificate and credential verification (no login required)
- Sponsored course creation: employer co-creates, institution delivers
- Graduate outcome reporting

**Student career features:**
- Career profile: skills, CV, portfolio link
- Apply for jobs/internships directly through SmartLMS
- Internship tracking: log hours, supervisor evaluates
- Graduate employment outcome declaration

**Institution benefits:**
- Graduate employment rate metric for accreditation
- Curriculum feedback from employers (skill gaps they see in graduates)
- Revenue from employer partnerships

---

### Module 34 — Recognition of Prior Learning (RPL)

**RPL application (student-initiated):**
- Student selects courses/competencies they believe they already meet
- Uploads evidence: work portfolios, certificates, employer letters, prior transcripts
- Writes reflective statement mapping experience to learning outcomes

**Assessment workflow:**
- Assessor role reviews evidence against course learning outcomes
- May schedule a challenge assessment (oral exam or practical demonstration)
- Decision: Full credit / Partial credit / Not granted
- Full audit trail of decision and evidence

**On grant:**
- Course marked as "Credit via RPL" in gradebook
- Counts toward graduation requirements
- Transcript shows RPL credit separately
- Student not charged for credited course

---

### Module 35 — Student Portfolio System

**Portfolio types:**
- Academic — curated best work across all courses
- Competency — evidence mapped to each CBE node
- Project — showcase of major projects with context
- Career — public-facing, employer-ready version

**Portfolio items:**
- Any submission from any course can be "added to portfolio"
- Student adds context: what they learned, what they'd do differently
- Instructor can endorse specific items
- External evidence: personal projects, GitHub links, volunteer work

**Sharing modes:** Private (default) / Share with advisor / Share with specific employer (time-limited link) / Public URL (student opt-in only)

**Portfolio as credential:** SmartLMS generates a verified portfolio PDF with institution seal and digital signature. Each item timestamped — employer can verify it's genuine. Contains competency map.

---

### Module 36 — Built-in Proctoring (4 Tiers)

**Tier 1 — Honour-based:** Student confirms they are working alone. No technical enforcement.

**Tier 2 — Behavioural (existing + expanded):** Tab/focus tracking, copy-paste disabled, IP + device fingerprinting, full answer audit trail.

**Tier 3 — Camera monitoring (Growth+):**
- Student enables camera before CAT
- Engine captures periodic screenshots (configurable interval)
- Screenshots reviewed by instructor post-exam, not live-monitored
- Julia ML flags anomalies: multiple faces detected, student absent from frame
- Processed on institution's own server — no third-party service
- Student sees exactly what data is collected before they begin
- Data deleted after configurable retention period (default 30 days)

**Tier 4 — Live invigilation (Enterprise):**
- Invigilator watches live camera feed from N students simultaneously
- Grid view of 16 students
- Flag a student → send warning notification
- Terminate a student's exam session with reason

**Browser lockdown (optional):**
- Open-source SmartLMS browser extension (Chrome/Firefox)
- Engine verifies extension is active before allowing CAT start
- Not a proprietary browser — institution-hosted

**Business case:** Eliminates $5–20 per student per exam cost of third-party proctoring tools (Respondus, ProctorU, Honorlock). Immediate, quantifiable saving.

---

### Module 37 — Accessibility (First-Class)

**WCAG 2.2 AA compliance — built into every component:**
- All images require alt text (enforced in course builder — cannot publish without it)
- All videos get auto-captions via speech recognition on upload
- Full keyboard navigation — no mouse required anywhere
- Screen reader tested (NVDA + VoiceOver + JAWS)
- Colour contrast enforced in white-label system
- Dyslexia-friendly font option (OpenDyslexic) toggleable by student
- Text size controls per student
- Motion reduction mode (no animations)

**Language and localisation:**
- Institution sets default language
- Student overrides to preferred language
- UI fully translated including error messages, notifications, and PDFs
- RTL layout for Arabic/Hebrew
- Julia AI responses in institution's configured language

**Device accessibility:**
- Tested on 2GB RAM Android phone from 2018
- Tested on 2G/EDGE connections — core functionality loads under 5 seconds
- SMS fallback for grade notifications
- USSD interface (experimental, Enterprise): basic grade/balance check via feature phone
  ```
  *213*STUDENTID*PIN# → "Your current GPA is 3.2. Next exam: Nov 15."
  ```

---

## 7. Julia AI/ML Engine — Full Architecture

### Overview

Julia runs as a Julia-lang HTTP microservice, called by Rust for inference. It is a **model registry** — a collection of specialised models, each independently deployed, versioned, and callable.

**Why Julia-lang:**
- Native GPU support via CUDA.jl
- Flux.jl for deep learning (DKT, essay grading neural models)
- MLJ.jl for classical ML (dropout prediction, grade forecasting)
- Turing.jl for probabilistic models (Bayesian knowledge tracing)
- Mathematical performance rivals Python for optimisation problems
- Designed for this exact domain

### Deployment Split

```
┌─────────────────────────────────────────────────────┐
│              JULIA INTELLIGENCE LAYER               │
├─────────────────────┬───────────────────────────────┤
│   CENTRAL SERVICE   │    LOCAL ENGINE               │
│  (your servers)     │    (institution's server)     │
├─────────────────────┼───────────────────────────────┤
│ Heavy models        │ Lightweight models             │
│ Cross-institution   │ Single-institution data        │
│ Premium tier only   │ All tiers                      │
│ GPU-backed          │ CPU only                       │
│ Retrained nightly   │ Updated with engine releases   │
└─────────────────────┴───────────────────────────────┘
```

### Julia Model Registry

```
julia-service/
  ├── models/
  │     ├── dkt/              # Deep Knowledge Tracing
  │     ├── dropout/          # Dropout prediction
  │     ├── essay_grader/     # Multi-dimensional AEG
  │     ├── question_gen/     # Question generation
  │     ├── code_assessor/    # Code grading
  │     ├── recommender/      # Content recommendation
  │     ├── sentiment/        # Forum sentiment analysis
  │     ├── speech_to_text/   # Voice assessment
  │     ├── similarity/       # Plagiarism detection
  │     └── scheduler/        # Study schedule optimisation
  │
  ├── registry.jl             # Routes requests to correct model
  ├── inference.jl            # Inference pipeline
  ├── training/
  │     ├── pipeline.jl       # Nightly retraining orchestration
  │     └── privacy.jl        # PII scrubbing before training
  └── api/
        ├── routes.jl         # HTTP endpoints called by Rust
        └── auth.jl           # Validates institution licence key
```

---

### J1 — Deep Knowledge Tracing (DKT+)

Tracks how a student's knowledge of each concept evolves over time using a recurrent neural network. Goes beyond IRT by modelling temporal knowledge evolution and concept interdependencies.

```julia
struct DKTModel
    embedding::Embedding      # question concept embeddings
    lstm::LSTM                # temporal knowledge state
    output::Dense             # probability per concept
end
# k_t = f(k_{t-1}, interaction_t)
# "Given everything this student has done, what's the probability
#  they get question q right?"
```

**Output — Student Knowledge State (real-time):**
```
Concept: Chain Rule          ████░░░░░░  43% mastery ← flagged
Concept: Product Rule        ███████░░░  71% mastery
Concept: Integration         ██████░░░░  61% mastery

Predicted CAT performance: 67%
Weakest prerequisite before next topic: Chain Rule (43%)
Recommended: 2 practice questions on Chain Rule before proceeding
```

**Forgetting curve model (Ebbinghaus + DKT):**
```
current_mastery = initial_mastery × e^(-decay_rate × days_since_last_interaction)
```
Surfaces "memory refresh" micro-sessions proactively before exams on forgotten content.

---

### J2 — Content Generation Engine

**J2A — Course skeleton generator:** Instructor inputs course title, level, and prerequisites. Julia generates a full course structure with units, lessons, lesson types, CATs, and a learning outcomes map — including Bloom's taxonomy alignment.

**J2B — Question generation from content:**
- Instructor uploads PDF, video transcript, or webpage
- Julia generates MCQs at multiple Bloom's levels, short answer questions, and essay prompts
- Each question tagged by topic, difficulty, and Bloom's level
- Model answers and marking criteria generated automatically
- Instructor reviews and approves before adding to question bank

**J2C — Question bank analysis:**
```
"Question Q047 answered correctly by 94% of students — may be too easy"
"Questions Q089 and Q091 test the same concept at same difficulty"
"Chapter 7 has only 2 questions in your bank — recommend 8-10 more"
```

**J2D — Auto-summarisation and study guides:**
- Chapter summaries (1 page per chapter)
- Key concept glossary with page references
- Visual concept map
- "Most likely exam topics" based on question bank weighting and learning outcomes
- Personalised study checklist based on individual knowledge state

**J2E — Multilingual content translation:**
- Extract all text, preserve formatting and block structure
- Neural MT translation with per-course technical glossary
- Instructor reviews diff view (original | translation side by side)
- Post-translation TTS audio generation
- Auto-subtitles on all videos in all published languages

---

### J3 — Advanced Assessment Intelligence

**J3A — Automated Essay Grading (AEG):**

```julia
struct EssayGrader
    content_scorer::ContentRelevanceModel
    argument_scorer::ArgumentStructureModel
    evidence_scorer::EvidenceQualityModel
    language_scorer::LanguageQualityModel
    rubric_aligner::RubricAlignmentModel
end
```

Output: dimension-by-dimension breakdown with highlighted problem areas. Instructor always sees Julia's reasoning, can override any dimension. **Julia grades, instructor decides — always.**

**J3B — Oral assessment and voice analysis:**
- Speech-to-text → NLP content grading
- Pace analysis (words per minute)
- Filler word detection (um, like, you know)
- Clarity and enunciation scoring
- Confidence from pitch variance analysis
- Grammar errors flagged with timestamps
- Full transcript generated for instructor review

**J3C — Code assessment:**
- Test case execution (pass/fail per test)
- Time and space complexity analysis
- Code quality: naming conventions, documentation, style compliance
- Similarity check against classmate submissions
- Constructive feedback with hints (not solutions)

**J3D — Computer Adaptive Testing (true CAT):**
```
After each question:
  current_ability = IRT_estimate(all_answers_so_far)
  next_question = argmax(information_gain(question, current_ability))

Result: 90% confidence ability estimate in 15 questions vs 30 random
        Shorter, more precise, less exhausting for students
```

---

### J4 — Predictive Intelligence Suite

**J4A — Dropout prediction (full feature set):**
```
Academic signals:    grade trajectory, submission rate, CAT performance
Engagement signals:  login frequency, time-on-platform, video completion, forum activity
Contextual signals:  wellbeing score, fee payment status, attendance rate
Historical signals:  cross-cohort anonymised patterns, intervention success rates

Output:
  Risk score: 0–100
  Top contributing factors (ranked)
  Recommended interventions ordered by historical effectiveness:
    1. Instructor personal message (78% success for this profile)
    2. Peer mentor assignment (65%)
    3. Extension on upcoming assignment (51%)
    4. Counsellor referral
```

**J4B — Grade forecasting:**
```
Student dashboard (opt-in):
  Current average: 64%
  Predicted final grade: 71% (current trajectory)
  "If you submit outstanding Assignment 3: 74%"
  "If you improve CAT scores by 10%: 78% (B+)"
  What this requires: [specific actions]

Instructor dashboard:
  Grade distribution forecast for cohort at end of semester
  "12 students predicted to fail — [View list]"
  "23 students on grade boundary — intervention could move them up"
```

**J4C — Time-to-mastery prediction:**
Predicts when a student will achieve mastery of a competency based on current knowledge state, learning rate, concept difficulty, and predicted forgetting. Used by advisors for academic planning.

**J4D — Instructor effectiveness model:**
Measures student grade improvement controlling for prior ability, engagement rates, feedback turnaround. Used for self-improvement insights — **not for performance management by default.** Configurable admin visibility.

---

### J5 — Natural Language Processing Suite

**J5A — Semantic search across the entire LMS:**
- Queries match by intent, not just keyword
- "I don't understand IRR" matches content tagged "Internal Rate of Return"
- Searches: course content (videos → transcripts, PDFs → text), forum threads, library, past CAT questions
- Results ranked by relevance to student's current knowledge state

**J5B — Forum sentiment analysis:**
- Real-time monitoring of forum posts
- Flags for instructor: emerging confusion on a topic
- Flags for counsellor (private): language suggesting high stress
- Flags for admin: escalating negative sentiment threads
- Aggregate course sentiment trend visible to instructor

**J5C — Automated discussion facilitation:**
- When thread goes 48h without instructor response, Julia can:
  - Draft a response for instructor approval
  - Post a curated resource link as "SmartLMS Assistant"
  - Escalate to instructor
- Julia never posts as if it is the instructor
- Full transparency: posts clearly labelled as AI-assisted

**J5D — Real-time writing feedback (student-facing, optional):**
```
As student types their assignment:
  Clarity: "This sentence is 67 words long — consider splitting it"
  Structure: "You've introduced a claim but provided no evidence"
  Academic language: "Consider 'significant' instead of 'really important'"
  Rubric alignment: "Your submission addresses 2 of 4 rubric criteria.
                     Missing: discussion of counterarguments (Criterion 3)"
  Word count pacing: "You've used 450 words on point 1 of 4"
```
This is AI coaching the student to write better — not AI writing the assignment. The distinction is academically and ethically non-negotiable.

---

### J6 — Personalisation Engine

**J6A — Learning style modelling (inference-based):**
Julia infers content preferences from behaviour (video completion vs reading completion, performance correlation by content type). Never labels students as "visual learners" (scientifically contested). Silently drives content ordering and recommendation priority.

**J6B — Personalised study schedule:**
```
Input: "My exam is in 14 days. I can study 2 hours per day."

Output: Day-by-day schedule
  - Prioritises weak areas from knowledge state
  - Schedules memory refresh for decaying concepts
  - Includes mixed practice exam at midpoint
  - Adapts in real-time based on daily practice performance
```

**J6C — Peer matching and study group formation:**
```
Julia matches based on:
  - Complementary knowledge states (students can teach each other)
  - Compatible schedules (most active at same times)
  - Compatible learning pace
  - Positive social signals from forum interactions

Instructor action: "Assign recommended groups with one click"
```

---

### J7 — Instructor Intelligence Tools

**J7A — Pedagogical coach:**
```
"Your course has 6 consecutive video lessons with no assessment.
 Consider adding a quiz or discussion prompt between Lesson 3 and 4."

"Assignment 3 rubric has 5 criteria but grading shows criteria 4-5
 are rarely used. Consider simplifying the rubric."

"Your CAT bank has 80% recall-level questions.
 Higher-order questions better predict long-term learning.
 Consider adding Bloom's levels 4–6 questions."
```

**J7B — Assessment workload balancing:**
```
Instructor view: "Week 8 has 3 major deadlines across your courses.
 Students in CS301 and CS302 have assignments due the same day."

Admin view: "Week 11 has the highest assessment density
 across all departments (23 deadlines in 5 days).
 This historically correlates with lowest wellbeing scores."
```

**J7C — Automated marking assistant:**
```
For short-answer questions before instructor grades:
  1. Groups similar answers for batch grading
  2. Flags outliers (possibly misunderstood the question)
  3. Suggests grades with confidence and reasoning
  4. Instructor makes all final decisions
  Result: 60–70% reduction in grading time
```

---

### J8 — Agentic Julia (Enterprise)

Julia can be configured to **take actions autonomously** within strict institution-defined limits.

```toml
[julia.agent]
enabled = true
autonomy_level = "supervised"    # supervised | semi-autonomous | autonomous
human_review_queue = true        # all actions queued for human review

[julia.agent.permitted_actions]
send_encouragement_message = true
extend_deadline_max_days = 2
flag_for_counsellor = true
create_study_group = true
post_resource_to_forum = true
send_parent_notification = true  # if parent portal + consent enabled
enrol_in_support_module = true

[julia.agent.requires_human]
grade_override = true
student_suspension = true
exam_card_revocation = true
any_financial_action = true
```

**Scenario: Early intervention at 11pm:**
```
Julia detects: 9 days no login, assignment due in 48 hours,
               wellbeing score 2/5, risk score 78

Julia (autonomous): sends supportive message offering extension/support
                    logs action for instructor review in the morning
                    flags for counsellor if no response in 24 hours

Instructor sees: "Julia sent 3 check-in messages overnight. [Review]"
```

---

### J9 — Multimodal Intelligence

**J9A — Video content intelligence (on institution's server):**
```
On upload, Julia automatically:
  1. Transcribes (English, Swahili, French — priority languages)
  2. Generates chapter markers with timestamps
  3. Extracts key concepts mentioned
  4. Flags issues: inaudible sections, small slide text
  5. Generates searchable transcript embedded in the player
     (student Ctrl+F → click any word → video jumps to that moment)
```

**J9B — Document intelligence:**
- Research paper → summary, key arguments, course topic linkage, APA citation
- Marking criteria document → auto-converted to machine-readable rubric

**J9C — Diagram and figure analysis:**
- Recognises diagram type (circuit, biology, flowchart)
- Identifies components and labels
- Checks against expected answer
- Annotates student's image with visual feedback
- Produces correctness score for instructor review

---

### J10 — Ethics and Transparency Layer

**Explanation for every Julia output:**
```
"Why did Julia flag this student as at-risk?"
→ Risk score: 74
→ Top factors:
     1. Login frequency dropped 80% in last 2 weeks (weight: 35%)
     2. Last 2 assignments submitted late (weight: 28%)
     3. Wellbeing score: 2/5 (weight: 22%)
     4. Forum activity: zero posts this month (weight: 15%)
→ Historical: 67% of students with this profile who received
  no intervention did not complete the course
```

**Bias monitoring:**
- Julia monitors its own outputs for demographic disparities
- If bias detected: flag to admin + suspend that model feature pending investigation
- Regular bias audit reports available to admin

**Student opt-out controls:**
```
Student can opt out of:
  ├── Personalised recommendations
  ├── Predictive grade display
  └── Proactive check-in messages

Student cannot opt out of:
  ├── Plagiarism detection (institution requirement)
  └── CAT integrity monitoring (academic integrity)
```

---

### Julia Capability Map by Tier

| Capability | Starter | Growth | Enterprise |
|---|---|---|---|
| Basic dropout risk (rule-based) | ✓ | ✓ | ✓ |
| Plagiarism/similarity detection | ✓ | ✓ | ✓ |
| Auto-quiz from uploaded text | ✓ | ✓ | ✓ |
| Essay grammar/structure check | ✓ | ✓ | ✓ |
| Deep Knowledge Tracing (DKT+) | — | ✓ | ✓ |
| Full dropout prediction (ML) | — | ✓ | ✓ |
| Grade forecasting | — | ✓ | ✓ |
| Content recommendation | — | ✓ | ✓ |
| Question generation from content | — | ✓ | ✓ |
| Semantic search | — | ✓ | ✓ |
| Video transcription + chapters | — | ✓ | ✓ |
| Essay auto-grading (AEG) | — | ✓ | ✓ |
| Personalised study schedules | — | ✓ | ✓ |
| Peer matching | — | ✓ | ✓ |
| Forum sentiment analysis | — | ✓ | ✓ |
| Instructor pedagogical coaching | — | ✓ | ✓ |
| Real-time writing feedback | — | ✓ | ✓ |
| Agentic Julia | — | — | ✓ |
| Oral assessment / voice analysis | — | — | ✓ |
| Code assessment | — | — | ✓ |
| Computer Adaptive Testing | — | — | ✓ |
| Diagram / figure analysis | — | — | ✓ |
| Multilingual translation + TTS | — | — | ✓ |
| Private model training | — | — | ✓ |
| Julia API for institution's developers | — | — | ✓ |

---

## 8. Security Architecture

### Layer 1 — Rust advantage (compiler-level)

| Vulnerability class | Rust's position |
|---|---|
| Buffer overflow | Impossible — bounds checking enforced at compile time |
| Use-after-free | Impossible — ownership system prevents it |
| Data races | Impossible — borrow checker catches at compile time |
| Null pointer dereference | Eliminated — `Option<T>` forces explicit handling |
| SQL injection | Impossible — sqlx uses compile-time verified prepared statements |

### Layer 2 — Authentication

| Control | Implementation |
|---|---|
| Password hashing | Argon2id — memory-hard, GPU-resistant |
| JWT signing | RS256 asymmetric — institution's private key signs tokens |
| JWT expiry | Short-lived access tokens (15 min) + long-lived refresh tokens (7 days, HttpOnly cookie) |
| Refresh token rotation | Each use issues a new token. Old one immediately invalidated. |
| Session revocation | Admin force-logouts any user instantly via Redis invalidation list |
| Rate limiting | 5 login attempts/minute per IP. Lockout after 10 failures with exponential backoff. |
| MFA | TOTP (Google Authenticator compatible). Admin can require for instructor/admin roles. |
| SSO | Google OAuth, Microsoft OAuth, SAML 2.0. Token validation server-side only. |
| CSRF protection | Double-submit cookie pattern. SameSite=Strict on session cookies. |

### Layer 3 — Data protection

| Control | Detail |
|---|---|
| In-transit | TLS 1.3 enforced. TLS 1.0/1.1 rejected. HSTS with long max-age. Auto-TLS via Let's Encrypt. |
| Field-level encryption | High-sensitivity columns (government IDs, payment tokens) encrypted at application layer |
| PII minimisation | Engine collects only what is functionally needed |
| Audit log integrity | Append-only table. No UPDATE or DELETE permitted — enforced via database trigger |
| GDPR/DPA compliance | Student data export (JSON). Right-to-erasure anonymises identifying fields while preserving aggregate academic record integrity. |

### Layer 4 — Operational security

| Area | Practice |
|---|---|
| Secrets management | All secrets in environment variables — engine refuses to start if secrets are in config files in plain text |
| Dependency auditing | `cargo audit` in CI before every release. Known-vulnerable crates blocked. |
| Update security | Engine binary signed with your private key. Institution engine validates signature before applying any update. |
| Content Security Policy | Strict CSP headers. `script-src 'self'` — no inline scripts. |
| Upload validation | MIME type verified (not just extension), file size enforced, ClamAV antivirus hook (configurable) |
| XSS | React DOM rendering escapes by default. All user-generated content sanitised with DOMPurify on save. |
| Security headers | `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin` |
| Penetration testing | Run before each major version release. Results and remediations published in changelog (responsible disclosure). |
| Incident response | Security event stream. Critical events (brute force, admin password change, bulk export) raise immediate alerts to admin dashboard + configured webhook. |

---

## 9. Backup System

### What gets backed up

| Layer | Content | Risk if lost |
|---|---|---|
| Relational DB | Students, grades, enrolments, transactions, audit logs | Catastrophic — irreplaceable |
| MongoDB | Forum threads, course blocks, activity feeds | High |
| File storage | Videos, PDFs, assignment submissions, certificates | High |
| Config files | `smartlms.toml`, env vars, TLS certs | Medium — recoverable but time-consuming |
| Redis | Sessions, cache | Low — regenerates on restart. **Do NOT back up** |

### Backup agent config

```toml
[backup]
enabled = true
schedule = "0 2 * * *"          # 2am daily
retention_days = 30
destination = "s3"              # s3 | r2 | local | sftp
encrypt = true                  # AES-256-GCM before upload
encryption_key_env = "BACKUP_KEY"

[backup.destinations.s3]
bucket = "institution-backups"
region = "af-south-1"
prefix = "smartlms/"
```

### Backup pipeline

```
1. DB dump (pg_dump / mysqldump / mongodump — atomic snapshots)
2. File manifest (SHA-256 of every file)
3. AES-256-GCM encryption (key never leaves institution's server)
4. Chunked resumable upload to destination
5. Checksum verification after upload
6. Monthly restore dry-run (decrypt, check integrity, discard)
7. Retention rotation — older backups deleted, pinned backups preserved
```

### Restore CLI

```bash
smartlms restore \
  --from s3://institution-backups/smartlms/2024-11-15_02-00.backup \
  --key $BACKUP_KEY \
  --target postgres://localhost/smartlms
```

### Admin backup dashboard

- Last successful backup: timestamp + size
- Next scheduled backup: countdown
- Last verify run: result
- All backup history: downloadable, deletable, pinnable
- Destination health indicator
- One-click manual backup trigger
- Persistent warning until backup destination is configured (shown in onboarding wizard)

---

## 10. Packaging & Distribution

Five formats — same engine binary packaged differently.

| Format | Best for | Update method |
|---|---|---|
| **Docker image** | Institutions with a technical team | Pull new image, restart |
| **Linux binary** | Bare VPS, minimum footprint | Replace binary, restart service |
| **Debian/Ubuntu package** | Sysadmin-managed institutions using apt | `apt upgrade smartlms` |
| **One-click cloud** (DO/AWS Marketplace) | Non-technical institutions — live in 10 minutes | Auto-update or admin-approval |
| **JS SDK** | Developers embedding LMS in existing site | `npm update @smartlms/sdk` |

### JS SDK embed

```html
<script src="https://cdn.smartlms.io/sdk.js"
        data-key="inst_uon_xxxx"
        data-container="#lms-portal">
</script>
```

### How institution engines update

```
1. Engine phones home → sees new version available
2. Pulls new binary + migration scripts from update server
3. Validates binary signature (your private key)
4. Applies on configured schedule: auto-install or admin-approval-first
5. Migration runs locally on their DB (additive only — never destructive)
6. Rollback always safe — old binary ignores new columns it doesn't know about
```

---

## 11. Pricing Model

### Starter — one-time licence
- Own your software — one payment, yours to keep
- 1 year of updates included
- Pay per major version upgrade after year one — opt in when ready
- Community support via forum and docs only
- Local Julia models (lightweight)
- "Powered by SmartLMS" in footer
- **Targets:** small schools, tight budgets, institutions that want software ownership

### Growth — affordable monthly subscription
- All updates automatic — always on latest engine version
- Full central Julia ML service included up to learner quota
- White-label — full branding, no SmartLMS mention
- Email support
- Telemetry opt-in unlocks bonus Julia features
- Overage billing above learner seat threshold
- **Targets:** growing schools, corporate training, EdTech startups

### Enterprise — annual contract, custom pricing
- Unlimited learners, storage, and API calls
- Full Julia ML — no quota, private model option (your data trains only your model)
- Full white-label, 100% branding removed
- Dedicated support and SLA guarantee
- Custom onboarding and white-glove migration assistance
- Private telemetry
- Source code escrow included
- **Targets:** large universities, government bodies, large corporates

### Positioning vs Moodle
> *"Moodle is free but costs you a sysadmin, server management time, and the risk of manual upgrades breaking your installation. SmartLMS Starter is a one-time fee for software that installs in minutes, upgrades itself, and never puts your data at risk."*

### Positioning vs Canvas/Blackboard/Brightspace
> *"Canvas tells you what happened. Blackboard shows you a grade. Julia tells you what's about to happen — and does something about it."*

---

## 12. Competitive Positioning

### Competitive gap analysis

| Feature | Canvas | Moodle | Blackboard | Brightspace | **SmartLMS** |
|---|---|---|---|---|---|
| Competency-Based Education | Weak add-on | Plugin | Limited | ✓ Enterprise | **Native, all tiers** |
| Micro-credentials / Badges | Separate product | Plugin | No | Partial | **Native + blockchain** |
| Student Wellbeing | No | No | No | No | **Native module** |
| Academic Advising | No | No | No | No | **Native module** |
| Research Supervision | No | No | No | No | **Native module** |
| Real Offline Mode | No | No | No | No | **True offline-first** |
| Structured Peer Review | Basic | Basic | Basic | Basic | **Calibrated, graded** |
| Employer Portal | No | No | No | No | **Native module** |
| RPL | No | No | No | Partial | **Native workflow** |
| Portfolio System | No | Mahara (separate) | No | No | **Native, verifiable** |
| Built-in Proctoring | No (3rd party) | No | No | No | **4 tiers, native** |
| M-Pesa / Local Payments | No | No | No | No | **Native** |
| USSD Interface | No | No | No | No | **Native (Enterprise)** |
| Parent Portal | No | No | No | No | **Native module** |
| Exam Cards | No | No | No | No | **Native module** |
| Clearance System | No | No | No | No | **Native module** |
| True White-Label (self-hosted) | No | Partial | No | No | **Complete** |
| Built-in AI content generation | No (2025) | No | No | No | **Native (Julia)** |

### Core strategic moats

1. **Self-hosted** — student data never leaves the institution's server. GDPR, Kenya DPA, and any future data localisation law compliance by architecture, not by policy.
2. **Julia AI** — predictive and proactive, not just descriptive. The only LMS with built-in agentic AI.
3. **African market fit** — M-Pesa, USSD, offline-first, exam cards, clearance system, KRA PIN fields — features built for the real operational context of institutions in Kenya and across Africa.
4. **Full-stack integration** — exam card connects to fee management connects to attendance connects to analytics. No silos, no extra cost to integrate modules that are all the same product.
5. **Rust performance** — a fundamentally smaller attack surface and higher throughput than any PHP/Python/Node.js competitor.

---

## 13. Community & Ecosystem

### Community Hub Architecture

**Zone 1 — Public Discovery (no login required)**
```
smartlms.io/community

├── Showcase gallery (case studies of institutions using SmartLMS)
├── Public roadmap (Planned / In Progress / Shipped — with your comments)
├── Plugin/theme marketplace (revenue share: you 20%, author 80%)
└── SmartLMS Academy (free courses running on a live SmartLMS instance)
```

**Zone 2 — Practitioner Forum (free account)**
```
forum.smartlms.io (Discourse — self-hosted, matches self-hosted ethos)

Categories:
  ├── #announcements (read-only)
  ├── #help
  ├── #show-and-tell
  ├── #feature-requests (linked to public roadmap)
  ├── #integrations
  ├── #regional (Africa, LATAM, SEA, Europe)
  └── #developers
```
1 paid community manager in year 1. Every unanswered question answered within 24 hours.

**Zone 3 — Champion Programme**

| Tier | Criteria | Benefits |
|---|---|---|
| Contributor | 1+ accepted tutorial or 25+ questions answered | Early beta access |
| Champion | Active monthly, runs local user group | Free Growth licence, direct Slack with your team, quarterly product input session |
| Partner Champion | Officially resells or implements SmartLMS | Revenue share on referred customers (5% year 1), listed on partner directory |

### SmartLMS Academy

Free, public-facing courses running on a live SmartLMS instance. Serves as a live demo, training tool, and marketing asset simultaneously.

| Course | Duration |
|---|---|
| Getting Started | 2 hours |
| Designing Engaging Online Courses | 4 hours |
| Assessment That Actually Measures Learning | 3 hours |
| Reading Your Analytics | 2 hours |
| Advanced — Building with the SDK | Self-paced |

Every course completion earns a verifiable SmartLMS Academy badge — the first micro-credentials issued on your own platform, before a single paying customer exists.

### Course Templates (in course builder)

| Template | Design Philosophy | Best for |
|---|---|---|
| Flipped Classroom | Video lectures before class, synchronous time for problems | Maths, sciences, coding |
| 5-Stage Challenge-Based | Challenge → Research → Share → Reflect → Act | Business, social sciences |
| Weekly Cohort | Same deadline for whole cohort, strong social learning | Professional development |
| Self-Paced Mastery | Unlock next unit only after passing this one | Skills training, languages |
| Project-Based | Semester-long project with milestone check-ins | Final year, capstone |
| Seminar-Style | Reading → Discussion → Synthesis | Postgraduate, humanities |
| Compliance Training | Short modules, mandatory completion, evidence trail | Corporate, health, legal |

---

## 14. Migration Tooling

### `smartlms-migrate` — standalone Rust binary, open-sourced

Making the migrator open source removes a procurement objection and signals confidence.

### Moodle → SmartLMS (Priority 1)

Moodle backup files are `.mbz` archives (`.tar.gz` containing XML, well-documented format).

**Migration scope:**

| Content | Coverage |
|---|---|
| Users (name, email, roles) | ✓ Complete |
| Course structure (sections/topics) | ✓ Complete |
| Course content (pages, files) | ✓ Complete |
| Assignments (settings, deadlines) | ✓ Complete |
| Quiz questions (MCQ, essay) | ✓ via QTI conversion |
| Historical grades | ✓ Complete |
| Forum threads and posts | ✓ Complete |
| Enrolments | ✓ Complete |
| SCORM packages | ✓ Pass-through |
| Badges earned | ~ Partial |
| H5P content | Exported as files (manual re-import) |
| Custom Moodle plugins with no equivalent | Flagged in report |

**Migration report (generated before any data is touched):**
```
SmartLMS Migration Report — [Institution Name] Moodle 4.1

SUMMARY
  Users:          4,847  ✓ Ready
  Courses:          312  ✓ Ready
  Grade records: 421,889 ✓ Ready
  Forum posts:   156,221 ✓ Ready

WARNINGS
  ⚠  14 courses use H5P — content exported as files (manual re-upload)
  ⚠  3 courses use Moodle-specific blocks with no SmartLMS equivalent

ERRORS
  ✗  None

ESTIMATED TIME: 4–6 hours (run overnight)
Proceed? [yes/no]:
```

### Canvas QTI importer (Priority 2)

Canvas exports quizzes as QTI 2.1 XML. SmartLMS imports QTI natively — any Canvas institution migrates their question bank without rebuilding it. Also works with any IMS Global-standard exam content.

### Blackboard partial importer (Priority 3)

Best-effort importer covering course structure, content files, and grade history (~80% of what institutions care about). Blackboard-specific tool integrations documented as manual steps.

### White-Glove Migration Service (Enterprise)

```
Paid service:
  ├── Pre-migration audit of current LMS setup
  ├── Custom migration scripting for specific plugins/customisations
  ├── Parallel running period: both systems live for 30 days
  ├── Staff training during transition
  ├── Post-migration dedicated support: 90 days
  └── Pricing: $5,000–$50,000 depending on institution size and complexity
```

---

## 15. Enterprise Compliance & Trust

### Trust Center (build from day 1, before any certifications)

**Location:** `smartlms.io/trust`

```
Security:
  ├── Architecture overview (Rust + self-hosted story)
  ├── Encryption in transit and at rest
  ├── Authentication controls
  ├── Penetration test summary (published after every major release)
  └── Vulnerability disclosure policy

Privacy:
  ├── What SmartLMS Ltd collects (licence server, telemetry only)
  ├── What institutions control (everything else)
  ├── Data Processing Agreement template
  └── Right-to-erasure process

Compliance:
  ├── SOC 2 Type I report (when achieved) — on request with NDA
  ├── WCAG 2.2 AA conformance statement + VPAT
  ├── Kenya Data Protection Act 2019 registration
  └── GDPR controller/processor clarification

Uptime:
  ├── Status page (your central servers only)
  ├── Historical uptime
  └── Incident history and post-mortems
```

### Certification roadmap

**Year 1 — Foundation**
- Months 1–3: Write and implement security policies (acceptable use, incident response, change management, access control)
- Months 4–6: SOC 2 readiness gap assessment with a consultant
- Months 7–12: SOC 2 Type I audit ($15,000–$30,000)

**Year 2 — Certification**
- SOC 2 Type II (6–12 month observation period)
- ISO/IEC 27001 (run in parallel where possible)
- VPAT for the frontend (required for government and higher ed deals)
- Kenya Data Protection Commissioner registration (home market first)

### VPAT (Voluntary Product Accessibility Template)

Formal documentation of WCAG 2.2 AA conformance for the entire frontend. Required for:
- Government procurement in many countries
- US higher education (Section 508)
- EU public sector institutions (EN 301 549)

---

## 16. Partner Programme

| Tier | Qualification | Model | Benefits |
|---|---|---|---|
| **Referral** | Anyone | Share link, earn 10% of first year licence fee | No minimum commitment |
| **Reseller** | Business registration + 1 deployed customer | Buy at 30% discount, sell at list price | Partner portal, co-branded sales materials, priority support |
| **Implementation** | Completed SmartLMS certification ($500 course) | Offer migration/deployment services under own brand | Partner directory listing, referral fee from engine deals |
| **Technology** | Product integration with SmartLMS | Listed in marketplace | Co-marketing, co-documentation |

**Why the partner programme is critical for distribution:**
The first 20 paying customers in Africa will come from 3–4 well-chosen Reseller Partners who already have trust relationships with institutions in their country. Local partners know the procurement contacts, budget cycles, and decision makers. This network cannot be built by cold outreach — it must be local.

---

## 17. Business Continuity Policy

Institutions are being asked to trust SmartLMS with student records that legally must be retained for 30+ years in many jurisdictions. The continuity policy provides structural answers, not just reassurances.

### Source Code Escrow
- Engine source code deposited with an independent escrow agent (e.g. Iron Mountain)
- Updated with every major release
- Release conditions: company dissolution, cessation of updates for 18+ months, bankruptcy filing
- Enterprise licence agreements include escrow access rights
- Cost: ~$2,000/year

### Licence Continuity Clause
- Starter licence holders keep their purchased version forever
- If SmartLMS ceases update distribution, Starter licences convert to source-available status for self-maintenance

### Community Edition Clause
- If SmartLMS Ltd ever ceases to operate, the last stable engine version is released under an open-source licence
- Written into company articles of association — not just marketing copy

### The Runbook (Enterprise clients)
"If SmartLMS disappeared tomorrow, here is how your team maintains the engine you already have, for the next 5 years." Provided to Enterprise clients as part of onboarding. Almost no software company does this. Closes deals that promises don't.

---

## 18. Build Sequence

### Phase 0 — Before writing engine code (parallel tracks)
1. Trust Center skeleton — policies and architecture documentation (2 weeks)
2. Community Hub structure — register domain, set up Discourse instance
3. Public roadmap — every feature from this document goes on it
4. Sign first 2–3 Reseller Partners in target markets — give early access
5. SOC 2 readiness gap assessment

### Phase 1 — Engine Foundation
6. Multi-tenant router
7. Per-institution DB provisioning
8. Upgrade service
9. Licence server integration

### Phase 2 — Institution Onboarding
10. Self-serve signup
11. Guided setup wizard
12. Plan and quota management
13. 14-day sandbox with sample data

### Phase 3 — White-label & SDK
14. CSS variable injection + logo/favicon hosting
15. Custom domain with auto-TLS
16. White-labelled emails
17. JS SDK (3-line embed)

### Phase 4 — Users & Roles
18. Full role system (Admin/Instructor/Learner/Observer/Parent/Advisor/Counsellor/Alumni)
19. SSO (Google/Microsoft/SAML)
20. Bulk CSV import
21. Student Registration System

### Phase 5 — Courses & Content
22. Drag-drop course builder with guided instructional design mode
23. Video upload and HLS transcoding
24. SCORM import
25. Course templates gallery

### Phase 6 — Assessments
26. Question bank
27. Randomised CATs with full integrity suite
28. Deadline-based assignments
29. Weighted gradebook
30. Exam Bank

### Phase 7 — Communication
31. Announcements
32. Direct messaging
33. Discussion Forums (full design)
34. Notification centre + push notifications

### Phase 8 — Live Classes
35. Session scheduling
36. Zoom/Meet/Jitsi integration
37. Auto-recording and transcript
38. Attendance tracking

### Phase 9 — Institution Operations (African market features)
39. Exam Cards
40. Attendance System (QR-based)
41. Fee Management System (M-Pesa + Stripe + bank transfer)
42. Clearance System
43. Timetable & Scheduling
44. Parents Portal
45. Student & Alumni ID Cards

### Phase 10 — Automation & Gamification
46. Visual rule builder (IF/THEN, no code)
47. Certificate issuer with QR verification
48. Badges, XP, leaderboards
49. Outbound webhooks

### Phase 11 — Advanced Academic Features
50. Competency-Based Education (CBE)
51. Micro-Credentials & Digital Badges (Open Badges 3.0)
52. Student Wellbeing module
53. Academic Advising module
54. Research & Postgraduate Supervision
55. Peer Learning & Structured Peer Review
56. Student Portfolio System
57. Recognition of Prior Learning (RPL)
58. Alumni Portal

### Phase 12 — Analytics & Reporting
59. Learner dashboard
60. Course analytics
61. Cohort comparison
62. Custom report builder
63. xAPI/Tin Can export

### Phase 13 — Julia AI/ML Engine
64. Local models (Starter tier — all institutions)
65. Central Julia service infrastructure
66. DKT+ and knowledge tracing
67. Dropout prediction (full ML)
68. Content recommendation
69. Essay auto-grading
70. Question generation
71. Semantic search
72. Video transcription and intelligence
73. Personalisation engine
74. Instructor intelligence tools
75. Forum sentiment analysis
76. Real-time writing feedback
77. Agentic Julia (Enterprise)
78. Oral assessment and voice analysis
79. Code assessment
80. Computer Adaptive Testing
81. Multimodal intelligence
82. Ethics and transparency layer

### Phase 14 — Employer & Career
83. Employer Portal
84. Job board and internship tracking
85. Portfolio sharing with employers

### Phase 15 — Library & Content Repository
86. Library module
87. Collection management
88. Citation export
89. OPDS feed (Enterprise)

### Phase 16 — Security Hardening, Compliance & Packaging
90. Built-in proctoring (all 4 tiers)
91. Offline-first architecture (true offline)
92. Accessibility audit (WCAG 2.2 AA) + VPAT
93. USSD interface (Enterprise, experimental)
94. All five packaging formats
95. Moodle migrator (open-sourced)
96. Canvas QTI importer
97. SOC 2 Type I audit
98. Source code escrow setup

### Phase 17 — Developer Platform
99. GraphQL API
100. TypeScript SDK npm package
101. API playground
102. Marketplace for third-party integrations
103. Developer documentation

---

## 19. Full Module Map

| # | Module | Tier | Phase |
|---|---|---|---|
| 1 | Engine foundation | All | 1 |
| 2 | JavaScript SDK | All | 3 |
| 3 | White-label system | All | 3 |
| 4 | Institution onboarding | All | 2 |
| 5 | Users & roles | All | 4 |
| 6 | Courses & content | All | 5 |
| 7 | Assessments | All | 6 |
| 8 | Live classes | Growth+ | 8 |
| 9 | Communication | All | 7 |
| 10 | Julia AI/ML engine | Growth+ (local on Starter) | 13 |
| 11 | Automation engine | All | 10 |
| 12 | Analytics & reporting | All | 12 |
| 13 | Billing & finance | All | 9 |
| 14 | Library | Growth | 15 |
| 15 | Exam Bank | Growth | 6 |
| 16 | Student Registration System | All | 4 |
| 17 | Exam Cards | Growth | 9 |
| 18 | Parents Portal | Growth | 9 |
| 19 | Attendance System | All | 9 |
| 20 | Fee Management System | Growth | 9 |
| 21 | Clearance System | Enterprise | 9 |
| 22 | Timetable & Scheduling | Growth | 9 |
| 23 | Student & Alumni ID Cards | All | 9 |
| 24 | Alumni Portal | Enterprise | 11 |
| 25 | Discussion Forums (full) | All | 7 |
| 26 | Competency-Based Education | Growth | 11 |
| 27 | Micro-Credentials & Digital Badges | Growth | 11 |
| 28 | Student Wellbeing | Growth | 11 |
| 29 | Academic Advising | Growth | 11 |
| 30 | Research & Postgrad Supervision | Enterprise | 11 |
| 31 | Offline-First Architecture | All | 16 |
| 32 | Peer Learning & Structured Peer Review | Growth | 11 |
| 33 | Employer & Industry Portal | Enterprise | 14 |
| 34 | Recognition of Prior Learning (RPL) | Growth | 11 |
| 35 | Student Portfolio System | Growth | 11 |
| 36 | Built-in Proctoring (4 tiers) | All tiers/tiered | 16 |
| 37 | Accessibility (first-class) | All | 16 |

---

*SmartLMS Engine — Master Reference Document v0.2*  
*Compiled from planning sessions. All decisions subject to revision during build.*  
*Next milestone: Phase 0 pre-build tasks — Trust Center, Community Hub, first Reseller Partners.*
