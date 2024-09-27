// use crate::api::api_error::ApiError;
// use crate::domain::member::update::Error;
// use crate::startup::AppState;
// use crate::uow::member::SqlxMemberUnitOfWork;
// use axum::extract::State;
// use axum::http::StatusCode;
// use axum::Json;
// use axum_extra::extract::WithRejection;
// use serde::Deserialize;
// use tokio::sync::Mutex;
//
// #[derive(Debug, Deserialize)]
// pub(crate) struct UpdateMemberRequest {
//     member_id: String,
//     name: String,
//     description: String,
//     language: String,
// }
//
// pub async fn update_member(
//     State(state): State<AppState>,
//     WithRejection(Json(req), _): WithRejection<Json<UpdateMemberRequest>, ApiError>,
// ) -> Result<StatusCode, ApiError> {
//     let req = crate::domain::member::update::Request {
//         member_id: req.member_id,
//         data: crate::domain::member::entities::Data {
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
//     match crate::domain::member::update::execute(uow, req).await {
//         Ok(()) => Ok(StatusCode::OK),
//         Err(Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
//     }
// }
