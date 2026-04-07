// RBAC - Role-Based Access Control system
use serde::{Deserialize, Serialize};

/// Permission names - granular actions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Users
    UsersCreate,
    UsersRead,
    UsersUpdate,
    UsersDelete,
    
    // Courses
    CoursesCreate,
    CoursesRead,
    CoursesUpdate,
    CoursesDelete,
    CoursesPublish,
    
    // Enrollments
    EnrollmentsCreate,
    EnrollmentsRead,
    EnrollmentsUpdate,
    EnrollmentsDelete,
    
    // Assessments
    AssessmentsCreate,
    AssessmentsRead,
    AssessmentsUpdate,
    AssessmentsDelete,
    AssessmentsGrade,
    
    // Grades
    GradesRead,
    GradesWrite,
    GradesExport,
    
    // Reports
    ReportsView,
    ReportsExport,
    
    // Settings
    SettingsRead,
    SettingsUpdate,
    
    // Institution
    InstitutionManage,
    InstitutionUsers,
    
    // Billing (Growth+)
    BillingManage,
}

impl Permission {
    pub fn name(&self) -> &'static str {
        match self {
            Permission::UsersCreate => "users:create",
            Permission::UsersRead => "users:read",
            Permission::UsersUpdate => "users:update",
            Permission::UsersDelete => "users:delete",
            Permission::CoursesCreate => "courses:create",
            Permission::CoursesRead => "courses:read",
            Permission::CoursesUpdate => "courses:update",
            Permission::CoursesDelete => "courses:delete",
            Permission::CoursesPublish => "courses:publish",
            Permission::EnrollmentsCreate => "enrollments:create",
            Permission::EnrollmentsRead => "enrollments:read",
            Permission::EnrollmentsUpdate => "enrollments:update",
            Permission::EnrollmentsDelete => "enrollments:delete",
            Permission::AssessmentsCreate => "assessments:create",
            Permission::AssessmentsRead => "assessments:read",
            Permission::AssessmentsUpdate => "assessments:update",
            Permission::AssessmentsDelete => "assessments:delete",
            Permission::AssessmentsGrade => "assessments:grade",
            Permission::GradesRead => "grades:read",
            Permission::GradesWrite => "grades:write",
            Permission::GradesExport => "grades:export",
            Permission::ReportsView => "reports:view",
            Permission::ReportsExport => "reports:export",
            Permission::SettingsRead => "settings:read",
            Permission::SettingsUpdate => "settings:update",
            Permission::InstitutionManage => "institution:manage",
            Permission::InstitutionUsers => "institution:users",
            Permission::BillingManage => "billing:manage",
        }
    }
}

/// Role definitions with associated permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    // System roles
    SuperAdmin,    // Can manage all institutions
    
    // Institution roles
    Admin,         // Full institution control
    Instructor,    // Can create/manage courses
    Learner,       // Can take courses
    Observer,      // View-only, no modifications
    Parent,        // Guardian view of child progress
    Advisor,       // Academic advisor
    Counsellor,    // Student welfare
    Alumni,        // Past student access
}

impl Role {
    /// Get all permissions for this role
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::SuperAdmin => vec![
                Permission::UsersCreate, Permission::UsersRead, 
                Permission::UsersUpdate, Permission::UsersDelete,
                Permission::CoursesCreate, Permission::CoursesRead,
                Permission::CoursesUpdate, Permission::CoursesDelete, Permission::CoursesPublish,
                Permission::EnrollmentsCreate, Permission::EnrollmentsRead,
                Permission::EnrollmentsUpdate, Permission::EnrollmentsDelete,
                Permission::AssessmentsCreate, Permission::AssessmentsRead,
                Permission::AssessmentsUpdate, Permission::AssessmentsDelete, Permission::AssessmentsGrade,
                Permission::GradesRead, Permission::GradesWrite, Permission::GradesExport,
                Permission::ReportsView, Permission::ReportsExport,
                Permission::SettingsRead, Permission::SettingsUpdate,
                Permission::InstitutionManage, Permission::InstitutionUsers,
                Permission::BillingManage,
            ],
            Role::Admin => vec![
                Permission::UsersCreate, Permission::UsersRead,
                Permission::UsersUpdate, Permission::UsersDelete,
                Permission::CoursesCreate, Permission::CoursesRead,
                Permission::CoursesUpdate, Permission::CoursesDelete, Permission::CoursesPublish,
                Permission::EnrollmentsCreate, Permission::EnrollmentsRead,
                Permission::EnrollmentsUpdate, Permission::EnrollmentsDelete,
                Permission::AssessmentsCreate, Permission::AssessmentsRead,
                Permission::AssessmentsUpdate, Permission::AssessmentsDelete, Permission::AssessmentsGrade,
                Permission::GradesRead, Permission::GradesWrite, Permission::GradesExport,
                Permission::ReportsView, Permission::ReportsExport,
                Permission::SettingsRead, Permission::SettingsUpdate,
                Permission::InstitutionUsers,
            ],
            Role::Instructor => vec![
                Permission::CoursesCreate, Permission::CoursesRead,
                Permission::CoursesUpdate, Permission::CoursesPublish,
                Permission::EnrollmentsRead,
                Permission::AssessmentsCreate, Permission::AssessmentsRead,
                Permission::AssessmentsUpdate, Permission::AssessmentsDelete, Permission::AssessmentsGrade,
                Permission::GradesRead, Permission::GradesWrite,
                Permission::ReportsView,
            ],
            Role::Learner => vec![
                Permission::CoursesRead,
                Permission::EnrollmentsRead,
                Permission::AssessmentsRead,
                Permission::GradesRead,
            ],
            Role::Observer => vec![
                Permission::CoursesRead,
                Permission::EnrollmentsRead,
                Permission::AssessmentsRead,
                Permission::GradesRead,
            ],
            Role::Parent => vec![
                Permission::CoursesRead,
                Permission::GradesRead,
                Permission::ReportsView,
            ],
            Role::Advisor => vec![
                Permission::UsersRead,
                Permission::CoursesRead,
                Permission::EnrollmentsRead,
                Permission::AssessmentsRead,
                Permission::GradesRead,
                Permission::ReportsView,
            ],
            Role::Counsellor => vec![
                Permission::UsersRead,
                Permission::CoursesRead,
                Permission::ReportsView,
            ],
            Role::Alumni => vec![
                Permission::CoursesRead,
                Permission::GradesRead,
            ],
        }
    }
    
    /// Check if role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions().contains(permission)
    }
    
    /// Convert role to string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::SuperAdmin => "super_admin",
            Role::Admin => "admin",
            Role::Instructor => "instructor",
            Role::Learner => "learner",
            Role::Observer => "observer",
            Role::Parent => "parent",
            Role::Advisor => "advisor",
            Role::Counsellor => "counsellor",
            Role::Alumni => "alumni",
        }
    }
}

/// Convert string to Role
impl TryFrom<&str> for Role {
    type Error = String;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "super_admin" | "superadmin" => Ok(Role::SuperAdmin),
            "admin" => Ok(Role::Admin),
            "instructor" => Ok(Role::Instructor),
            "learner" | "student" => Ok(Role::Learner),
            "observer" => Ok(Role::Observer),
            "parent" | "guardian" => Ok(Role::Parent),
            "advisor" => Ok(Role::Advisor),
            "counsellor" | "counselor" => Ok(Role::Counsellor),
            "alumni" | "alumnus" => Ok(Role::Alumni),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

/// Authorization helper - check if user can perform action
pub mod authz {
    use super::*;
    
    /// Check if a role can perform an action
    pub fn can(role: &str, permission: Permission) -> bool {
        Role::try_from(role)
            .map(|r| r.has_permission(&permission))
            .unwrap_or(false)
    }
    
    /// Filter list of items based on permissions (for list endpoints)
    pub fn filter_by_permission<T>(
        items: Vec<T>,
        role: &str,
        permission: Permission,
    ) -> Vec<T>
    where T: Clone
    {
        if can(role, permission) {
            items
        } else {
            vec![]
        }
    }
}