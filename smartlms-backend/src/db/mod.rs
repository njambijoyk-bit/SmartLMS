// Database module - DB operations
pub mod abac;
pub mod assessment;
pub mod attendance;
pub mod backup;
pub mod communication;
pub mod course;
pub mod course_group;
pub mod institution;
pub mod iot;
pub mod live;
pub mod user;
pub mod parents_alumni;

pub use communication::*;
pub use institution::*;
pub use iot::*;
pub use user::*;
pub use parents_alumni::*;
