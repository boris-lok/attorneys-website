#[derive(Debug, Clone)]
pub enum WorkLogMappingStatus {
    Pending,
    Rejected,
    Approved,
}

impl From<WorkLogMappingStatus> for String {
    fn from(value: WorkLogMappingStatus) -> String {
        match value {
            WorkLogMappingStatus::Pending => "pending".to_string(),
            WorkLogMappingStatus::Rejected => "rejected".to_string(),
            WorkLogMappingStatus::Approved => "approved".to_string(),
        }
    }
}

impl TryFrom<String> for WorkLogMappingStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "pending" => Ok(WorkLogMappingStatus::Pending),
            "rejected" => Ok(WorkLogMappingStatus::Rejected),
            "approved" => Ok(WorkLogMappingStatus::Approved),
            _ => Err("Invalid status".to_string()),
        }
    }
}
