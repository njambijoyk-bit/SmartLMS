# Course Management Implementation - Next Steps

## ✅ Completed Components

### Backend (Rust/Axum)
1. **Database Layer** (`src/db/course.rs`)
   - ✅ `update_module()` - Update module details
   - ✅ `delete_module()` - Remove modules
   - ✅ `update_lesson()` - Update lesson content
   - ✅ `delete_lesson()` - Remove lessons
   - ✅ `reorder_modules()` - Bulk reorder modules
   - ✅ `reorder_lessons()` - Bulk reorder lessons
   - ✅ `delete_course()` - Delete entire course
   - ✅ `get_instructor_courses()` - Filter courses by instructor

2. **Service Layer** (`src/services/courses.rs`)
   - ✅ Business logic for all CRUD operations
   - ✅ Archive/publish functionality
   - ✅ Validation and error handling

3. **API Routes** (`src/api/courses.rs`)
   - ✅ `GET /api/courses/instructor` - List instructor's courses
   - ✅ `PUT /api/courses/modules/:id` - Update module
   - ✅ `DELETE /api/courses/modules/:id` - Delete module
   - ✅ `PUT /api/courses/lessons/:id` - Update lesson
   - ✅ `DELETE /api/courses/lessons/:id` - Delete lesson
   - ✅ `PATCH /api/courses/modules/reorder` - Reorder modules
   - ✅ `PATCH /api/courses/lessons/reorder` - Reorder lessons
   - ✅ `POST /api/courses/:id/archive` - Archive course
   - ✅ `DELETE /api/courses/:id` - Delete course

### Frontend (React/TypeScript)
1. **API Service** (`src/services/courseApi.ts`)
   - ✅ TypeScript interfaces for all entities
   - ✅ API client functions for all operations
   - ✅ Error handling and response parsing

2. **Custom Hooks** (`src/hooks/useCourses.ts`)
   - ✅ `useInstructorCourses()` - Manage instructor course list
   - ✅ `useCourseDetail()` - Fetch and manage course details

3. **Components**
   - ✅ `CourseBuilder.tsx` - Drag-and-drop course editor
     - Module management (add/edit/delete/reorder)
     - Lesson management with 7 types (video, text, quiz, assignment, document, external, SCORM)
     - Visual indicators for preview/free status
   
4. **Pages**
   - ✅ `InstructorCoursesPage.tsx` - Course listing for instructors
     - Grid/List view toggle
     - Search and filter by status
     - Quick actions (publish, archive, edit, delete)
     - Pagination support
   
   - ✅ `CourseEditorPage.tsx` - Course creation/editing interface
     - Integration with CourseBuilder component
     - Publish/Archive/Delete actions
     - Auto-save indication

## 🔧 Remaining Tasks

### 1. Backend Enhancements

#### A. Complete Course Creation Flow
```rust
// TODO: Add bulk save endpoint for course structure
POST /api/courses/:id/bulk-save
{
  "modules": [
    {
      "title": "...",
      "lessons": [...]
    }
  ]
}
```

#### B. Add Missing Features
- [ ] Lesson completion tracking for instructors
- [ ] Course duplication/cloning
- [ ] Module/lesson bulk operations
- [ ] Course import/export (SCORM, xAPI)
- [ ] Rich text content storage for lessons
- [ ] File upload endpoints for lesson attachments
- [ ] Video transcoding integration

#### C. Permission & Authorization
- [ ] Verify instructor ownership before edits
- [ ] Role-based access control (instructor vs admin)
- [ ] Audit logging for course changes

### 2. Frontend Enhancements

#### A. Course Builder Improvements
- [ ] **Backend Integration**: Connect drag-and-drop to actual API calls
- [ ] **Auto-save**: Implement debounced save on changes
- [ ] **Undo/Redo**: Track state history for undo functionality
- [ ] **Bulk Edit**: Select multiple lessons for batch operations
- [ ] **Rich Text Editor**: Integrate Tiptap or similar for lesson content
- [ ] **File Upload**: Add drag-drop file upload for documents/videos
- [ ] **Preview Mode**: Show how course appears to students
- [ ] **Validation**: Form validation before save

#### B. Additional Pages Needed
- [ ] **Course Settings Page**: 
  - Course metadata editing
  - Enrollment settings
  - Pricing configuration
  - Certificate settings
  
- [ ] **Analytics Dashboard**:
  - Student enrollment trends
  - Completion rates
  - Engagement metrics
  - Assessment performance

- [ ] **Content Library**:
  - Reusable lesson templates
  - Media library
  - Question bank for quizzes

#### C. User Experience
- [ ] Loading states and skeletons
- [ ] Toast notifications for actions
- [ ] Confirmation dialogs for destructive actions
- [ ] Keyboard shortcuts for power users
- [ ] Mobile responsive improvements

### 3. Database Migrations

Check if migration exists:
```bash
ls -la smartlms-backend/migrations/*course*.sql
```

Required tables verification:
- [ ] `courses` - with proper indexes
- [ ] `modules` - with order indexing
- [ ] `lessons` - with full-text search
- [ ] `enrollments` - composite indexes
- [ ] `course_progress` - tracking table
- [ ] `course_reviews` - rating system

### 4. Testing

#### Backend Tests
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_create_course() {}
    
    #[tokio::test]
    async fn test_reorder_modules() {}
    
    #[tokio::test]
    async fn test_delete_course_cascade() {}
}
```

#### Frontend Tests
- [ ] Component unit tests (Vitest/React Testing Library)
- [ ] Integration tests for course builder
- [ ] E2E tests (Playwright/Cypress)

### 5. Documentation

- [ ] API documentation (OpenAPI/Swagger)
- [ ] Instructor user guide
- [ ] Video tutorials for course creation
- [ ] Admin documentation for moderation

## 🎯 Priority Recommendations

### Immediate (This Sprint)
1. **Connect CourseBuilder to backend** - Make the drag-and-drop actually save
2. **Add file upload support** - Enable document/video uploads
3. **Implement auto-save** - Prevent data loss
4. **Add rich text editor** - Better content creation experience

### Short-term (Next 2 Weeks)
1. **Course analytics dashboard** - Show instructor insights
2. **Bulk operations** - Improve efficiency for large courses
3. **Mobile responsiveness** - Ensure tablet/desktop works well
4. **Permission system** - Secure course editing

### Medium-term (Next Month)
1. **SCORM/xAPI support** - Industry standard compliance
2. **Course templates** - Speed up course creation
3. **Collaboration features** - Multiple instructors per course
4. **Version history** - Track and restore previous versions

## 📋 Quick Start Guide for Instructors

Once implementation is complete, instructors can:

1. **Create a Course**
   ```
   Dashboard → My Courses → Create Course
   ```

2. **Add Content**
   ```
   Course Editor → Add Module → Add Lessons
   - Drag to reorder
   - Click to edit
   - Set preview/free lessons
   ```

3. **Publish**
   ```
   Course Editor → Publish (when ready)
   ```

4. **Manage**
   ```
   - Edit anytime (auto-saves)
   - View analytics
   - Archive when done
   ```

## 🔗 Related Files

- Backend API: `/workspace/smartlms-backend/src/api/courses.rs`
- Service Layer: `/workspace/smartlms-backend/src/services/courses.rs`
- DB Layer: `/workspace/smartlms-backend/src/db/course.rs`
- Frontend Service: `/workspace/smartlms-frontend/src/services/courseApi.ts`
- Course Builder: `/workspace/smartlms-frontend/src/components/courses/CourseBuilder.tsx`
- Instructor Pages: `/workspace/smartlms-frontend/src/pages/instructor/`

---

**Status**: Core infrastructure complete. Ready for integration testing and UX refinements.
