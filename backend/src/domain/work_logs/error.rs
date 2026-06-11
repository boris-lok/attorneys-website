#[derive(Debug, thiserror::Error)]
pub enum WorkLogError {
    #[error("work log not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("unknown error: {0}")]
    Unknown(String),
}
