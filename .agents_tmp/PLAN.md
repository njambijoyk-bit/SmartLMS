# SmartLMS Implementation Plan

## 1. OBJECTIVE

Add **Attribute-Based Access Control (ABAC)** to enhance the existing RBAC system, and document what remains to be built in the broader SmartLMS system (per the master reference document with ~100 features across 17 phases).

---

## 2. CONTEXT SUMMARY

### Current State (Authorization Layer)
- **RBAC implemented**: `smartlms-backend/src/services/rbac.rs` contains Role enum with 9 roles (SuperAdmin, Admin, Instructor, Learner, Observer, Parent, Advisor, Counsellor, Alumni) and Permission enum with ~35 granular permissions
- **Security service**: `smartlms-backend/src/services/security.rs` has encryption, PII detection, and retention policies
- **Auth flow**: JWT-based authentication with tenant middleware

### What's Built So Far (Broader LMS)
- Backend: Multi-tenant routing, tenant context, user/auth APIs, courses, enrollments, assessments, attendance, live classes, backup services, SSO, whitelabel, license, onboarding
- Frontend: ~30 page components covering dashboard, courses, users, grades, forums, library, proctoring, etc.
- Infrastructure: Docker, Kubernetes manifests, Helm chart

### Master Reference (What Was Planned)
The `smartlms_master_reference.md` outlines 100 features across 17 build phases, covering:
- Phase 1-3: Engine foundation, institution onboarding, white-label
- Phase 4: Users & Roles (mostly done - RBAC exists)
- Phase 5-6: Courses, content, assessments
- Phase 7-9: Communication, live classes, African market features
- Phase 10-12: Automation, analytics, advanced academics
- Phase 13: Julia AI/ML engine
- Phase 14-17: Employer portal, library, security hardening, developer platform

---

## 3. APPROACH OVERVIEW

### For ABAC Enhancement:
1. **Extend RBAC with ABAC** — Rather than replacing RBAC, create a layered approach where ABAC policies refine/override RBAC decisions based on attributes
2. **Policy engine design** — JSON-based policy documents stored per-institution, evaluated at request time
3. **Attribute sources** — User attributes (from user profile), resource attributes (from entity metadata), environment attributes (time, IP, device)

### For Remaining Build:
1. **Gap analysis** — Compare what's implemented vs. the 100-feature master reference
2. **Prioritization** — Group remaining items by dependency and tier (Starter/Growth/Enterprise)
3. **Implementation sequence** — Follow the build sequence from the master reference

---

## 4. IMPLEMENTATION STEPS

### PART A: ABAC Layer (Authorization Enhancement)

#### Step A.1: Define ABAC Data Structures
- **Goal**: Create Rust types for ABAC policy modeling
- **Method**: Add new module `smartlms-backend/src/services/abac.rs` with:
  - `Attribute` enum (user, resource, environment)
  - `AttributeValue` type (string, number, boolean, list, datetime)
  - `Condition` struct (attribute + operator + value)
  - `Policy` struct (effect, subjects, actions, resources, conditions)
  - `PolicySet` collection of policies
- **Reference**: Create `/workspace/project/SmartLMS/smartlms-backend/src/services/abac.rs`

#### Step A.2: Implement Policy Evaluation Engine
- **Goal**: Build the core ABAC policy evaluation logic
- **Method**:
  - Support operators: `eq`, `neq`, `gt`, `gte`, `lt`, `lte`, `in`, `not_in`, `contains`, `starts_with`, `ends_with`, `between`
  - Support logical operators: `and`, `or`, `not`
  - Add `evaluate(request, policy)` function returning `Decision::Allow` or `Decision::Deny`
  - Implement short-circuit evaluation (deny overrides allow)
- **Reference**: Extend `abac.rs` with evaluation logic

#### Step A.3: Create RBAC + ABAC Integration Layer
- **Goal**: Combine RBAC and ABAC into unified authorization
- **Method**:
  - Create `AccessDecision` enum: `Allow`, `Deny(String)`, `Indeterminate`
  - Implement `authorize(request, role, rbac_permission, abac_policies)` function
  - Flow: First check RBAC permission → then evaluate ABAC policies → combine results
  - Add `AbacPolicy` storage in database (per-institution)
- **Reference**: Add to `smartlms-backend/src/services/abac.rs` and wire into middleware

#### Step A.4: Add ABAC Policy Management APIs
- **Goal**: Allow institution admins to create/manage ABAC policies
- **Method**:
  - POST `/api/v1/abac/policies` — Create policy
  - GET `/api/v1/abac/policies` — List policies
  - PUT `/api/v1/abac/policies/:id` — Update policy
  - DELETE `/api/v1/abac/policies/:id` — Delete policy
  - POST `/api/v1/abac/policies/:id/test` — Test policy against sample request
- **Reference**: Create `smartlms-backend/src/api/abac.rs`

#### Step A.5: Pre-built Policy Templates
- **Goal**: Provide common ABAC patterns out of the box
- **Method**: Add templates for:
  - Time-based access (e.g., "only during business hours")
  - Department-based access (e.g., "only access own department's courses")
  - Enrollment status (e.g., "only active students can submit assignments")
  - IP-based restrictions (e.g., "allow only from campus network")
  - Course ownership (e.g., "instructors can only edit their own courses")
- **Reference**: Add template functions in `abac.rs`

---

### PART B: Remaining SmartLMS Build

#### Step B.1: Core Engine Foundation (Phase 1) - COMPLETE
| Item | Status |
|---|---|
| Multi-tenant router | ✅ Implemented in `tenant/` |
| Per-institution DB provisioning | ✅ Implemented |
| Upgrade service | ⚠️ Needs implementation |
| Licence server integration | ⚠️ Needs implementation |

#### Step B.2: Institution Onboarding (Phase 2) - PARTIAL
| Item | Status |
|---|---|
| Self-serve signup | ⚠️ Needs implementation |
| Guided setup wizard | ⚠️ Frontend exists, backend needs work |
| Plan and quota management | ⚠️ Needs implementation |
| 14-day sandbox with sample data | ❌ Not implemented |

#### Step B.3: White-label & SDK (Phase 3) - PARTIAL
| Item | Status |
|---|---|
| CSS variable injection + logo/favicon | ⚠️ Frontend exists |
| Custom domain with auto-TLS | ❌ Not implemented |
| White-labelled emails | ❌ Not implemented |
| JS SDK (3-line embed) | ❌ Not implemented |

#### Step B.4: Users & Roles (Phase 4) - PARTIAL
| Item | Status |
|---|---|
| Full role system | ✅ Implemented (RBAC) |
| SSO (Google/Microsoft/SAML) | ⚠️ Service exists, needs full integration |
| Bulk CSV import | ❌ Not implemented |
| Student Registration System | ⚠️ Frontend page exists, backend needs work |

#### Step B.5: Courses & Content (Phase 5) - PARTIAL
| Item | Status |
|---|---|
| Drag-drop course builder | ❌ Not implemented |
| Video upload and HLS transcoding | ❌ Not implemented |
| SCORM import | ❌ Not implemented |
| Course templates gallery | ❌ Not implemented |

#### Step B.6: Assessments (Phase 6) - PARTIAL
| Item | Status |
|---|---|
| Question bank | ⚠️ Basic structure exists |
| Randomised CATs with integrity suite | ❌ Not implemented |
| Deadline-based assignments | ⚠️ Basic implementation |
| Weighted gradebook | ⚠️ Basic implementation |
| Exam Bank | ⚠️ Frontend exists |

#### Step B.7: Communication (Phase 7) - PARTIAL
| Item | Status |
|---|---|
| Announcements | ❌ Not implemented |
| Direct messaging | ❌ Not implemented |
| Discussion Forums (full design) | ⚠️ Frontend exists |
| Notification centre + push notifications | ❌ Not implemented |

#### Step B.8: Live Classes (Phase 8) - PARTIAL
| Item | Status |
|---|---|
| Session scheduling | ⚠️ Basic implementation |
| Zoom/Meet/Jitsi integration | ❌ Not implemented |
| Auto-recording and transcript | ❌ Not implemented |
| Attendance tracking | ⚠️ Basic implementation |

#### Step B.9: African Market Features (Phase 9) - PARTIAL
| Item | Status |
|---|---|
| Exam Cards | ⚠️ Frontend exists |
| Attendance System (QR-based) | ❌ Not implemented |
| Fee Management (M-Pesa + Stripe) | ❌ Not implemented |
| Clearance System | ⚠️ Frontend exists |
| Timetable & Scheduling | ⚠️ Frontend exists |
| Parents Portal | ⚠️ Frontend exists |
| Student & Alumni ID Cards | ⚠️ Frontend exists |

#### Step B.10: Automation & Gamification (Phase 10) - NOT STARTED
| Item | Status |
|---|---|
| Visual rule builder (IF/THEN) | ❌ Not implemented |
| Certificate issuer with QR verification | ❌ Not implemented |
| Badges, XP, leaderboards | ❌ Not implemented |
| Outbound webhooks | ❌ Not implemented |

#### Step B.11: Advanced Academic Features (Phase 11) - NOT STARTED
| Item | Status |
|---|---|
| Competency-Based Education | ❌ Not implemented |
| Micro-Credentials & Digital Badges | ❌ Not implemented |
| Student Wellbeing module | ❌ Not implemented |
| Academic Advising module | ❌ Not implemented |
| Research & Postgraduate Supervision | ❌ Not implemented |
| Peer Learning & Structured Peer Review | ❌ Not implemented |
| Student Portfolio System | ❌ Not implemented |
| Recognition of Prior Learning (RPL) | ❌ Not implemented |
| Alumni Portal | ❌ Not implemented |

#### Step B.12: Analytics & Reporting (Phase 12) - NOT STARTED
| Item | Status |
|---|---|
| Learner dashboard | ⚠️ Frontend exists |
| Course analytics | ❌ Not implemented |
| Cohort comparison | ❌ Not implemented |
| Custom report builder | ❌ Not implemented |
| xAPI/Tin Can export | ❌ Not implemented |

#### Step B.13: Julia AI/ML Engine (Phase 13) - NOT STARTED
| Item | Status |
|---|---|
| Local models (Starter tier) | ❌ Not implemented |
| Central Julia service | ❌ Not implemented |
| DKT+ and knowledge tracing | ❌ Not implemented |
| Dropout prediction | ❌ Not implemented |
| Content recommendation | ❌ Not implemented |
| Essay auto-grading | ❌ Not implemented |
| Question generation | ❌ Not implemented |
| All other ML features | ❌ Not implemented |

#### Step B.14-17: Remaining Phases - NOT STARTED
- Phase 14: Employer & Career Portal
- Phase 15: Library & Content Repository
- Phase 16: Security Hardening, Compliance & Packaging
- Phase 17: Developer Platform (GraphQL, SDK, Marketplace)

---

## 5. TESTING AND VALIDATION

### ABAC Testing:
- Unit tests for each operator (`eq`, `gt`, `in`, etc.)
- Policy evaluation tests with combinations of conditions
- Integration tests: request → middleware → RBAC check → ABAC policy evaluation → response
- Admin UI test: create policy → verify behavior matches expectation

### Remaining Build Validation:
- Feature parity matrix: compare implementation vs. master reference
- API endpoint coverage: ensure each module has CRUD endpoints
- Integration tests between modules
- Phase-gated release testing (each phase independently deployable)

---

## 6. RECOMMENDED NEXT STEPS

1. **Immediate**: Implement ABAC layer (Steps A.1–A.5) — adds fine-grained access control
2. **Short-term**: Complete Phase 2–4 remaining items (onboarding, SSO, white-label)
3. **Medium-term**: Build out Phase 5–9 (courses, assessments, communication, live, African features)
4. **Long-term**: Phase 10+ (automation, analytics, ML engine, developer platform)
