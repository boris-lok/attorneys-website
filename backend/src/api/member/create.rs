// use crate::api::api_error::ApiError;
// use crate::domain::member::create;
// use crate::domain::member::create::{execute, Request};
// use crate::domain::member::entities::Data;
// use crate::startup::AppState;
// use crate::uow::member::SqlxMemberUnitOfWork;
// use axum::extract::State;
// use axum::Json;
// use axum_extra::extract::WithRejection;
// use axum_macros::debug_handler;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Mutex;
// use ulid::Ulid;
//
// #[derive(Debug, Deserialize)]
// pub(crate) struct CreateMemberRequest {
//     name: String,
//     description: String,
//     language: String,
// }
//
// #[derive(Debug, Serialize)]
// pub(crate) struct CreateMemberResponse {
//     member_id: String,
// }
//
// #[debug_handler]
// pub async fn create_member(
//     State(state): State<AppState>,
//     WithRejection(Json(req), _): WithRejection<Json<CreateMemberRequest>, ApiError>,
// ) -> Result<Json<CreateMemberResponse>, ApiError> {
//     let member_id = Ulid::new().to_string();
//
//     let request = Request {
//         member_id,
//         data: Data {
//             name: req.name,
//             description: req.description,
//         },
//         language: req.language,
//     };
//
//     let uow = SqlxMemberUnitOfWork::new(&state.pool)
//         .await
//         .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     let uow = Mutex::new(uow);
//
//     match execute(uow, request).await {
//         Ok(id) => Ok(Json(CreateMemberResponse { member_id: id })),
//         Err(create::Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(create::Error::Unknown(e)) => Err(ApiError::InternalServerError(e)),
//     }
// }
