# Course Management Implementation

## Overview
Complete implementation of course management functionality for instructors to create, edit, and organize courses/modules/lessons.

## Backend Changes

### Database Migration (`011_course_management.sql`)
Created comprehensive database schema with:
- `courses` - Main course entity with metadata
- `modules` - Course sections containing lessons
- `lessons` - Individual learning units
- `enrollments` - User course enrollments
- `course_progress` - Progress tracking
- `course_reviews` - Ratings and reviews

### Models (`src/models/course.rs`)
Added request/response types:
- `UpdateModuleRequest` - Module update payload
- `UpdateLessonRequest` - Lesson update payload
- `ReorderItemsRequest` - Bulk reorder operations
- `ReorderItem` - Individual reorder item

### Database Layer (`src/db/course.rs`)
Implemented CRUD operations:
- `update_module()` - Update module details
- `delete_module()` - Remove module
- `update_lesson()` - Update lesson content
- `delete_lesson()` - Remove lesson
- `reorder_modules()` - Bulk reorder modules
- `reorder_lessons()` - Bulk reorder lessons
- `delete_course()` - Remove course
- `get_instructor_courses()` - List instructor's courses

### Service Layer (`src/services/courses.rs`)
Business logic functions:
- `update_module()` - Module update service
- `delete_module()` - Module deletion service
- `update_lesson()` - Lesson update service
- `delete_lesson()` - Lesson deletion service
- `reorder_modules()` - Module reordering service
- `reorder_lessons()` - Lesson reordering service
- `delete_course()` - Course deletion service
- `get_instructor_courses()` - Instructor courses listing
- `archive_course()` - Archive existing course

### API Routes (`src/api/courses.rs`)
RESTful endpoints:
- `GET /api/courses` - List all published courses
- `GET /api/courses/instructor` - List instructor's courses
- `POST /api/courses` - Create new course
- `PUT /api/courses/:id` - Update course
- `DELETE /api/courses/:id` - Delete course
- `POST /api/courses/:id/publish` - Publish course
- `POST /api/courses/:id/archive` - Archive course
- `POST /api/courses/modules` - Create module
- `PUT /api/courses/modules/:id` - Update module
- `DELETE /api/courses/modules/:id` - Delete module
- `PATCH /api/courses/modules/reorder` - Reorder modules
- `POST /api/courses/lessons` - Create lesson
- `PUT /api/courses/lessons/:id` - Update lesson
- `DELETE /api/courses/lessons/:id` - Delete lesson
- `PATCH /api/courses/lessons/reorder` - Reorder lessons

## Frontend Changes

### Course Builder Component (`src/components/courses/CourseBuilder.tsx`)
Interactive drag-and-drop interface for:
- **Module Management**
  - Add/edit/delete modules
  - Drag-to-reorder modules
  - Set preview availability
  
- **Lesson Management**
  - Add/edit/delete lessons
  - Drag-to-reorder lessons within modules
  - Multiple lesson types: video, text, quiz, assignment, document, external, SCORM
  - Set preview and free lesson flags
  - Content editing based on lesson type

- **Features**
  - Two-panel layout (modules | lessons)
  - Visual lesson type icons
  - Badge indicators for preview/free lessons
  - Duration tracking
  - Dialog-based editing forms

## Usage

### For Instructors

1. **Create a Course**
   ```bash
   POST /api/courses
   {
     "title": "Introduction to Programming",
     "description": "Learn programming basics",
     "category": "Computer Science",
     "language": "en",
     "difficulty": "beginner"
   }
   ```

2. **Add Modules**
   ```bash
   POST /api/courses/modules
   {
     "course_id": "uuid",
     "title": "Getting Started",
     "description": "Introduction module",
     "order": 0
   }
   ```

3. **Add Lessons**
   ```bash
   POST /api/courses/lessons
   {
     "module_id": "uuid",
     "title": "Welcome Video",
     "lesson_type": "video",
     "video_url": "https://...",
     "order": 0,
     "is_preview": true
   }
   ```

4. **Reorder Content**
   ```bash
   PATCH /api/courses/modules/reorder
   {
     "items": [
       { "id": "module-uuid-1", "order": 0 },
       { "id": "module-uuid-2", "order": 1 }
     ]
   }
   ```

5. **Use the Course Builder UI**
   - Navigate to course creation page
   - Click "Add Module" to create modules
   - Select a module to view/add lessons
   - Drag items to reorder
   - Click edit icons to modify content

## API Endpoints Summary

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/courses` | List published courses |
| GET | `/api/courses/instructor` | List instructor's courses |
| POST | `/api/courses` | Create course |
| PUT | `/api/courses/:id` | Update course |
| DELETE | `/api/courses/:id` | Delete course |
| POST | `/api/courses/:id/publish` | Publish course |
| POST | `/api/courses/:id/archive` | Archive course |
| POST | `/api/courses/modules` | Create module |
| PUT | `/api/courses/modules/:id` | Update module |
| DELETE | `/api/courses/modules/:id` | Delete module |
| PATCH | `/api/courses/modules/reorder` | Reorder modules |
| POST | `/api/courses/lessons` | Create lesson |
| PUT | `/api/courses/lessons/:id` | Update lesson |
| DELETE | `/api/courses/lessons/:id` | Delete lesson |
| PATCH | `/api/courses/lessons/reorder` | Reorder lessons |

## Next Steps

1. **Integration Testing** - Test all API endpoints
2. **Frontend Integration** - Connect CourseBuilder to actual API
3. **Permission Checks** - Add RBAC for instructor/admin actions
4. **File Upload** - Add support for document/video uploads
5. **Rich Text Editor** - Integrate WYSIWYG for lesson content
6. **Bulk Operations** - Import/export course content
7. **Versioning** - Track course content versions
8. **Analytics** - Track student engagement per lesson

## Files Modified

### Backend
- `/smartlms-backend/migrations/011_course_management.sql` (NEW)
- `/smartlms-backend/src/models/course.rs`
- `/smartlms-backend/src/db/course.rs`
- `/smartlms-backend/src/services/courses.rs`
- `/smartlms-backend/src/api/courses.rs`

### Frontend
- `/smartlms-frontend/src/components/courses/CourseBuilder.tsx` (NEW)
