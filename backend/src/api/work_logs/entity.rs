use crate::api::api_error::ApiError;
use crate::domain::work_logs::entity::WorkLogFilters;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct APIWorkLogFilters {
    started_at: Option<String>,
    ended_at: Option<String>,
    settled: Option<String>,
}

impl TryFrom<APIWorkLogFilters> for WorkLogFilters {
    type Error = ApiError;

    fn try_from(value: APIWorkLogFilters) -> Result<Self, Self::Error> {
        let parse_bool = |s: &str| match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(ApiError::BadRequest),
        };

        let parse_dt = |s: &str| s.parse::<DateTime<Utc>>().map_err(|_| ApiError::BadRequest);

        Ok(Self {
            started_at: value.started_at.as_deref().map(parse_dt).transpose()?,
            ended_at: value.ended_at.as_deref().map(parse_dt).transpose()?,
            settled: value.settled.as_deref().map(parse_bool).transpose()?,
        })
    }
}
