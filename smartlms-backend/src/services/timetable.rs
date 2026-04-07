// Timetable & Scheduling System — Physical class timetables and exam scheduling
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, NaiveTime, Utc, Weekday};

/// A physical room or venue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub name: String,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub capacity: i32,
    pub equipment_tags: Vec<String>, // e.g. ["projector", "lab_computers", "whiteboard"]
    pub is_active: bool,
}

/// Day-of-week enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    pub fn from_chrono(w: Weekday) -> Self {
        match w {
            Weekday::Mon => DayOfWeek::Monday,
            Weekday::Tue => DayOfWeek::Tuesday,
            Weekday::Wed => DayOfWeek::Wednesday,
            Weekday::Thu => DayOfWeek::Thursday,
            Weekday::Fri => DayOfWeek::Friday,
            Weekday::Sat => DayOfWeek::Saturday,
            Weekday::Sun => DayOfWeek::Sunday,
        }
    }
}

/// A single timetable slot (recurring weekly)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableSlot {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub course_id: Uuid,
    pub course_code: String,
    pub course_name: String,
    pub instructor_id: Uuid,
    pub instructor_name: String,
    pub room_id: Uuid,
    pub room_name: String,
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub cohort_ids: Vec<Uuid>,
    pub academic_year: String,
    pub semester: u8,
    pub slot_type: SlotType,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotType {
    Lecture,
    Tutorial,
    Lab,
    Seminar,
    Workshop,
}

/// Conflict detected between slots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableConflict {
    pub conflict_type: ConflictType,
    pub slot_a_id: Uuid,
    pub slot_b_id: Uuid,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictType {
    RoomDoubleBooked,
    InstructorClash,
    StudentCohortClash,
}

/// Request to create a timetable slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSlotRequest {
    pub institution_id: Uuid,
    pub course_id: Uuid,
    pub instructor_id: Uuid,
    pub room_id: Uuid,
    pub day_of_week: DayOfWeek,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub cohort_ids: Vec<Uuid>,
    pub academic_year: String,
    pub semester: u8,
    pub slot_type: SlotType,
}

/// Exam timetable entry (links exam paper to a time slot and room)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamTimetableEntry {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub exam_paper_id: Uuid,
    pub course_code: String,
    pub course_name: String,
    pub exam_date: DateTime<Utc>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room_id: Uuid,
    pub room_name: String,
    pub room_capacity: i32,
    pub registered_students: i32,
    pub invigilator_ids: Vec<Uuid>,
    pub is_published: bool,
    pub academic_year: String,
    pub semester: u8,
}

/// Student's personal timetable view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentTimetable {
    pub student_id: Uuid,
    pub academic_year: String,
    pub semester: u8,
    pub weekly_slots: Vec<TimetableSlot>,
    pub exam_schedule: Vec<ExamTimetableEntry>,
}

/// iCal export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICalExport {
    pub student_id: Uuid,
    pub ical_content: String, // RFC 5545 iCalendar format
}

/// Room availability query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomAvailability {
    pub room_id: Uuid,
    pub room_name: String,
    pub capacity: i32,
    pub is_available: bool,
    pub conflicting_slots: Vec<Uuid>,
}

pub struct TimetableService;

impl TimetableService {
    /// Create a new room
    pub async fn create_room(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        name: &str,
        building: Option<String>,
        capacity: i32,
        equipment: Vec<String>,
    ) -> Result<Room, sqlx::Error> {
        Ok(Room {
            id: Uuid::new_v4(),
            institution_id,
            name: name.to_string(),
            building,
            floor: None,
            capacity,
            equipment_tags: equipment,
            is_active: true,
        })
    }

    /// Create a timetable slot — checks for conflicts first
    pub async fn create_slot(
        _pool: &sqlx::PgPool,
        req: CreateSlotRequest,
    ) -> Result<(TimetableSlot, Vec<TimetableConflict>), sqlx::Error> {
        // TODO: SELECT existing slots WHERE room_id = req.room_id AND day = req.day
        //       AND time ranges overlap → RoomDoubleBooked
        //       AND instructor_id = req.instructor_id → InstructorClash
        //       AND cohort_ids overlap → StudentCohortClash
        let conflicts: Vec<TimetableConflict> = vec![];

        let slot = TimetableSlot {
            id: Uuid::new_v4(),
            institution_id: req.institution_id,
            course_id: req.course_id,
            course_code: String::new(),
            course_name: String::new(),
            instructor_id: req.instructor_id,
            instructor_name: String::new(),
            room_id: req.room_id,
            room_name: String::new(),
            day_of_week: req.day_of_week,
            start_time: req.start_time,
            end_time: req.end_time,
            cohort_ids: req.cohort_ids,
            academic_year: req.academic_year,
            semester: req.semester,
            slot_type: req.slot_type,
            is_published: false,
            created_at: Utc::now(),
        };

        Ok((slot, conflicts))
    }

    /// Publish the timetable — notifies enrolled students
    pub async fn publish_timetable(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        academic_year: &str,
        semester: u8,
    ) -> Result<i64, sqlx::Error> {
        let _ = (institution_id, academic_year, semester);
        // TODO: UPDATE timetable_slots SET is_published = true WHERE institution_id = $1
        //       AND academic_year = $2 AND semester = $3
        //       Then push notifications to all affected students
        Ok(0)
    }

    /// Get a student's personal timetable
    pub async fn get_student_timetable(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
        academic_year: &str,
        semester: u8,
    ) -> Result<StudentTimetable, sqlx::Error> {
        let _ = (student_id, academic_year, semester);
        // TODO: SELECT slots via enrollments JOIN courses JOIN timetable_slots
        Ok(StudentTimetable {
            student_id,
            academic_year: academic_year.to_string(),
            semester,
            weekly_slots: vec![],
            exam_schedule: vec![],
        })
    }

    /// Schedule an exam paper into a room and time slot
    pub async fn schedule_exam(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        exam_paper_id: Uuid,
        room_id: Uuid,
        exam_date: DateTime<Utc>,
        start_time: NaiveTime,
        end_time: NaiveTime,
        invigilator_ids: Vec<Uuid>,
    ) -> Result<(ExamTimetableEntry, Vec<TimetableConflict>), sqlx::Error> {
        let _ = institution_id;
        // TODO: check room capacity >= registered students for this paper
        //       check no student has two exams at the same time
        let entry = ExamTimetableEntry {
            id: Uuid::new_v4(),
            institution_id,
            exam_paper_id,
            course_code: String::new(),
            course_name: String::new(),
            exam_date,
            start_time,
            end_time,
            room_id,
            room_name: String::new(),
            room_capacity: 0,
            registered_students: 0,
            invigilator_ids,
            is_published: false,
            academic_year: String::new(),
            semester: 1,
        };
        Ok((entry, vec![]))
    }

    /// Check room availability on a given day/time
    pub async fn check_room_availability(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        room_id: Uuid,
        day: DayOfWeek,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Result<RoomAvailability, sqlx::Error> {
        let _ = (institution_id, day, start_time, end_time);
        Ok(RoomAvailability {
            room_id,
            room_name: String::new(),
            capacity: 0,
            is_available: true,
            conflicting_slots: vec![],
        })
    }

    /// Export student timetable as iCal (RFC 5545)
    pub async fn export_ical(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
        academic_year: &str,
        semester: u8,
    ) -> Result<ICalExport, sqlx::Error> {
        // TODO: Build proper iCal from student timetable slots
        let ical = format!(
            "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//SmartLMS//EN\r\n\
             X-WR-CALNAME:SmartLMS {academic_year} Sem {semester}\r\nEND:VCALENDAR\r\n"
        );
        Ok(ICalExport {
            student_id,
            ical_content: ical,
        })
    }

    /// List all rooms for an institution
    pub async fn list_rooms(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
    ) -> Result<Vec<Room>, sqlx::Error> {
        let _ = institution_id;
        // TODO: SELECT * FROM rooms WHERE institution_id = $1
        Ok(vec![])
    }
}
