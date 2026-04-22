# Assessment Engine Implementation Guide

## Overview
Advanced assessment system supporting multiple question types, auto-grading, exam mode, and integration with Course Groups for lecturer-specific assessments.

## Database Schema

### Core Tables

```sql
-- Questions table (question bank)
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES courses(id),
    created_by UUID NOT NULL REFERENCES users(id),
    question_type VARCHAR(50) NOT NULL, -- multiple_choice, true_false, short_answer, essay, code, file_upload
    text TEXT NOT NULL,
    media_url TEXT,
    options JSONB, -- Array of {id, text, is_correct}
    correct_answer_hint TEXT,
    points INTEGER NOT NULL DEFAULT 1,
    tags TEXT, -- Comma-separated
    is_public BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Assessments (exams/quizzes/assignments)
CREATE TABLE assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_group_id UUID REFERENCES course_groups(id), -- NULL if for whole course
    course_id UUID NOT NULL REFERENCES courses(id),
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    assessment_type VARCHAR(50) NOT NULL, -- quiz, exam, assignment, practice
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    due_date TIMESTAMPTZ,
    time_limit_minutes INTEGER,
    passing_score INTEGER NOT NULL DEFAULT 60,
    shuffle_questions BOOLEAN DEFAULT false,
    show_results_immediately BOOLEAN DEFAULT false,
    require_lockdown_browser BOOLEAN DEFAULT false,
    allow_late_submission BOOLEAN DEFAULT false,
    late_penalty_percent INTEGER DEFAULT 0,
    status VARCHAR(50) DEFAULT 'draft', -- draft, published, archived
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Assessment-Question linkage
CREATE TABLE assessment_questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id),
    order_index INTEGER NOT NULL,
    points_override INTEGER, -- NULL uses question default
    UNIQUE(assessment_id, question_id)
);

-- Student attempts
CREATE TABLE assessment_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID NOT NULL REFERENCES assessments(id),
    student_id UUID NOT NULL REFERENCES users(id),
    started_at TIMESTAMPTZ DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    status VARCHAR(50) DEFAULT 'in_progress', -- in_progress, submitted, graded, expired
    total_score DECIMAL(5,2),
    percentage DECIMAL(5,2),
    is_late BOOLEAN DEFAULT false,
    lockdown_session_id TEXT,
    ip_address INET,
    attempt_number INTEGER DEFAULT 1,
    UNIQUE(assessment_id, student_id, attempt_number)
);

-- Individual answers
CREATE TABLE attempt_answers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    attempt_id UUID NOT NULL REFERENCES assessment_attempts(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id),
    answer_text TEXT,
    selected_options JSONB, -- Array of option IDs for MCQ
    code_content TEXT,
    file_url TEXT,
    auto_grade_score DECIMAL(5,2),
    manual_grade_score DECIMAL(5,2),
    feedback TEXT,
    graded_by UUID REFERENCES users(id),
    graded_at TIMESTAMPTZ,
    UNIQUE(attempt_id, question_id)
);

-- Gradebook entries
CREATE TABLE gradebook_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_group_id UUID NOT NULL REFERENCES course_groups(id),
    student_id UUID NOT NULL REFERENCES users(id),
    assessment_id UUID REFERENCES assessments(id),
    score DECIMAL(5,2) NOT NULL,
    max_score DECIMAL(5,2) NOT NULL,
    percentage DECIMAL(5,2) NOT NULL,
    letter_grade VARCHAR(10),
    comments TEXT,
    graded_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_group_id, student_id, assessment_id)
);

-- Indexes for performance
CREATE INDEX idx_questions_course ON questions(course_id);
CREATE INDEX idx_assessments_course ON assessments(course_id);
CREATE INDEX idx_assessments_group ON assessments(course_group_id);
CREATE INDEX idx_attempts_assessment ON assessment_attempts(assessment_id);
CREATE INDEX idx_attempts_student ON assessment_attempts(student_id);
CREATE INDEX idx_answers_attempt ON attempt_answers(attempt_id);
```

## API Endpoints

### Question Bank Management

```
POST   /api/v1/questions              - Create question
GET    /api/v1/questions              - List questions (filter by course, type, tags)
GET    /api/v1/questions/:id          - Get question details
PUT    /api/v1/questions/:id          - Update question
DELETE /api/v1/questions/:id          - Delete question
POST   /api/v1/questions/bulk-import  - Bulk import from CSV/Excel
```

### Assessment Management

```
POST   /api/v1/assessments                  - Create assessment
GET    /api/v1/assessments                  - List assessments (filter by course, group, status)
GET    /api/v1/assessments/:id              - Get assessment details with questions
PUT    /api/v1/assessments/:id              - Update assessment
DELETE /api/v1/assessments/:id              - Delete assessment
POST   /api/v1/assessments/:id/publish      - Publish assessment
POST   /api/v1/assessments/:id/questions    - Add questions to assessment
PUT    /api/v1/assessments/:id/questions    - Reorder questions
```

### Student Attempts

```
POST   /api/v1/assessments/:id/start        - Start new attempt
GET    /api/v1/assessments/:id/attempts     - Get all attempts (instructor)
GET    /api/v1/attempts/:id                 - Get attempt details
POST   /api/v1/attempts/:id/submit          - Submit attempt
POST   /api/v1/attempts/:id/answers         - Save answer (auto-save during exam)
POST   /api/v1/attempts/:id/grade           - Grade submission (instructor)
```

### Code Execution (Auto-Grading)

```
POST   /api/v1/code/execute                 - Execute code submission
GET    /api/v1/code/languages               - Get supported languages
```

### Gradebook

```
GET    /api/v1/gradebook/:course_id         - Get gradebook for course
GET    /api/v1/gradebook/:course_id/student/:student_id
POST   /api/v1/gradebook/entries            - Create/update grade entry
GET    /api/v1/gradebook/export             - Export grades (CSV/Excel)
```

## Key Features

### 1. Course Group Integration
- Lecturers can create assessments specific to their group
- Same course, different lecturers → different assessments
- Shared question banks across course groups optional

### 2. Exam Mode (Lockdown Browser)
- Require lockdown browser setting
- Track session ID and IP address
- Prevent tab switching (frontend enforcement)
- Flag suspicious behavior for review

### 3. Auto-Grading
- **MCQ/TrueFalse**: Instant grading
- **Short Answer**: Text matching with fuzzy logic
- **Code**: Execute in sandbox, compare output against test cases
- **Essay/File Upload**: Queue for manual grading

### 4. Late Submission Handling
- Configurable late penalty percentage
- Automatic score reduction calculation
- Instructor override capability

### 5. Question Randomization
- Shuffle questions per student
- Shuffle options within MCQ
- Pull random subset from question bank

### 6. Multiple Attempts
- Configurable retry limits
- Track attempt history
- Keep best/average/latest score option

## Frontend Components

### Instructor View
- `AssessmentBuilder` - Create/edit assessments
- `QuestionBankManager` - Manage question library
- `GradebookView` - Grade submissions, view analytics
- `AttemptReviewer` - Review individual student attempts

### Student View
- `AssessmentList` - View available/upcoming assessments
- `AssessmentTaker` - Take exam/quiz with timer
- `AttemptHistory` - View past attempts and scores
- `GradebookStudent` - View personal grades

## Integration Points

### With Course Groups
```rust
// When creating assessment
let assessment = create_assessment(pool, user_id, &CreateAssessmentRequest {
    course_id: course.id,
    course_group_id: Some(group.id), // Specific to lecturer's group
    ..default
}).await?;
```

### With Live Classes
- Link recorded lectures as study materials
- Schedule assessments after live sessions
- Attendance-based assessment unlocking

### With African Features
- Exam cards generated only for paid students
- M-Pesa payment verification before assessment access
- QR code attendance linked to exam eligibility

### With Julia AI (Future)
- Adaptive difficulty adjustment
- Automated essay scoring
- Dropout risk prediction based on assessment performance
- Plagiarism detection for code/essays

## Security Considerations

1. **Access Control**: RBAC ensures only enrolled students can attempt
2. **IP Logging**: Track location for proctoring
3. **Time Enforcement**: Server-side timer validation
4. **Answer Encryption**: Encrypt answers at rest
5. **Audit Trail**: Log all assessment activities

## Performance Optimization

1. **Question Caching**: Cache question banks in Redis
2. **Lazy Loading**: Load questions progressively
3. **Database Indexing**: Optimize for common queries
4. **Connection Pooling**: Efficient DB connection management

## Testing Strategy

1. **Unit Tests**: Individual function testing
2. **Integration Tests**: API endpoint testing
3. **Load Tests**: Simulate concurrent exam takers
4. **Security Tests**: Penetration testing for exam mode

## Migration Path

1. Create database migrations
2. Update Rust models (✅ Done)
3. Implement DB layer functions
4. Create API routes
5. Build frontend components
6. Integration testing
7. Documentation

## Next Steps

- [ ] Create SQL migration files
- [ ] Complete db/assessment.rs with all CRUD operations
- [ ] Build api/assessments.rs routes
- [ ] Create React components for assessment builder
- [ ] Implement code execution sandbox service
- [ ] Add real-time timer WebSocket for exams
- [ ] Build gradebook analytics dashboard
