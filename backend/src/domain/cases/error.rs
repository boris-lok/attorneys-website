#[derive(Debug, thiserror::Error)]
pub enum CaseError {
    #[error("case not found")]
    NotFound,
    #[error("case is closed")]
    CaseIsClosed,
    #[error("unknown error: {0}")]
    Unknown(String),
}
