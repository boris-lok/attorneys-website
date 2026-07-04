use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct CaseID(Uuid);

impl From<&CaseID> for Uuid {
    fn from(value: &CaseID) -> Self {
        value.0
    }
}

impl From<Uuid> for CaseID {
    fn from(value: Uuid) -> Self {
        CaseID(value)
    }
}

impl TryFrom<String> for CaseID {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&value).map_err(|_| String::from("invalid id"))?;
        Ok(CaseID(uuid))
    }
}

impl From<CaseID> for String {
    fn from(value: CaseID) -> Self {
        value.0.to_string()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Case {
    #[schema(value_type = String, format = "uuid")]
    pub id: CaseID,
    pub name: String,
    pub used_minutes: i32,
    pub estimated_minutes: i32,
    pub billing_cycle: i32,
    pub pending_logs: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed: bool,
}

#[derive(Debug)]
pub struct CreateCaseRequest {
    pub id: CaseID,
    pub name: String,
    pub estimated_minutes: i32,
    pub billing_cycle: i32,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct UpdateCaseRequest {
    pub id: CaseID,
    pub name: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub billing_cycle: Option<i32>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub closed: Option<bool>,
}

#[derive(Debug)]
pub struct SimpleCase {
    pub id: CaseID,
    pub closed: bool,
}
