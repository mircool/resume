use rocket_sync_db_pools::{database, diesel};

#[database("resume_db")]
pub struct ResumeDb(diesel::SqliteConnection);

pub use diesel::prelude::*;
pub use crate::schema::*;
