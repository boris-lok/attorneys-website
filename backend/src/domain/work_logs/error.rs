use crate::domain::cases::error::CaseError;

#[derive(Debug, thiserror::Error)]
pub enum WorkLogError {
    #[error("work log not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("unknown error: {0}")]
    Unknown(String),
    #[error("case is closed")]
    CaseIsClosed,
    #[error("case not found")]
    CaseNotFound,
}

impl From<CaseError> for WorkLogError {
    fn from(value: CaseError) -> Self {
        match value {
            CaseError::NotFound => WorkLogError::CaseNotFound,
            CaseError::CaseIsClosed => WorkLogError::CaseIsClosed,
            CaseError::Unknown(e) => WorkLogError::Unknown(e),
        }
    }
}
