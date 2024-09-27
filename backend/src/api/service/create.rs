// use crate::api::api_error::ApiError;
// use crate::startup::AppState;
// use crate::uow::service::SqlxServiceUnitOfWork;
// use axum::extract::State;
// use axum::Json;
// use axum_extra::extract::WithRejection;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Mutex;
// use ulid::Ulid;
//
// #[derive(Debug, Deserialize)]
// pub(crate) struct CreateServiceRequest {
//     data: String,
//     language: String,
// }
//
// #[derive(Debug, Serialize)]
// pub(crate) struct CreateServiceResponse {
//     service_id: String,
// }
//
// pub async fn create_service(
//     State(state): State<AppState>,
//     WithRejection(Json(req), _): WithRejection<Json<CreateServiceRequest>, ApiError>,
// ) -> Result<Json<CreateServiceResponse>, ApiError> {
//     let service_id = Ulid::new().to_string();
//
//     let request = crate::domain::service::create::Request {
//         service_id,
//         data: req.data,
//         language: req.language,
//     };
//
//     let uow = SqlxServiceUnitOfWork::new(&state.pool)
//         .await
//         .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     let uow = Mutex::new(uow);
//
//     match crate::domain::service::create::execute(uow, request).await {
//         Ok(id) => Ok(Json(CreateServiceResponse { service_id: id })),
//         Err(crate::domain::service::create::Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(crate::domain::service::create::Error::Unknown(e)) => {
//             Err(ApiError::InternalServerError(e))
//         }
//     }
// }
