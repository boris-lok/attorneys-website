// use crate::api::api_error::ApiError;
// use crate::domain::member::delete::Error;
// use crate::startup::AppState;
// use crate::uow::member::SqlxMemberUnitOfWork;
// use axum::extract::{Path, State};
// use axum::http::StatusCode;
// use std::collections::HashMap;
// use tokio::sync::Mutex;
//
// pub async fn delete_member(
//     State(state): State<AppState>,
//     Path(params): Path<HashMap<String, String>>,
// ) -> Result<StatusCode, ApiError> {
//     let uow = SqlxMemberUnitOfWork::new(&state.pool)
//         .await
//         .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     let uow = Mutex::new(uow);
//     let member_id = params.get("id").ok_or(ApiError::BadRequest)?;
//
//     let req = crate::domain::member::delete::Request {
//         member_id: member_id.to_string(),
//     };
//
//     match crate::domain::member::delete::execute(uow, req).await {
//         Ok(_) => Ok(StatusCode::OK),
//         Err(Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(Error::MemberNotFound) => Err(ApiError::NotFound),
//         Err(Error::Unknown(reason)) => Err(ApiError::InternalServerError(reason)),
//     }
// }
