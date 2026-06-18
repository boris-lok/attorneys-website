use crate::domain::cases::entity::CaseID;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateWorkLogRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub case_id: CaseID,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub is_collaborative: bool,
}

#[derive(Debug)]
pub struct UpdateWorkLogRequest {
    pub id: Uuid,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkLog {
    pub id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub user: SimpleUser,
    pub duration: i32,
    pub description: String,
    pub is_collaborative: bool,
    pub collaborators: Vec<Collaborator>,
}

#[derive(Debug, Serialize)]
pub struct SimpleUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Collaborator {
    pub parent_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub status: String,
}

#[derive(Debug)]
pub struct WorkLogFilters {
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub settled: Option<bool>,
}
