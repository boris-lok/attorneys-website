// use crate::api::api_error::ApiError;
// use crate::domain::home::entities::Home;
// use crate::domain::member::entities::Language;
// use crate::startup::AppState;
// use crate::uow::home::SqlxHomeUnitOfWork;
// use axum::extract::{Path, State};
// use axum::http::HeaderMap;
// use axum::Json;
// use serde::Serialize;
// use std::collections::HashMap;
// use tokio::sync::Mutex;
//
// #[derive(Serialize)]
// pub(crate) struct RetrieveHomeResponse {
//     home: Home,
// }
//
// pub async fn retrieve_home(
//     State(state): State<AppState>,
//     Path(params): Path<HashMap<String, String>>,
//     headers: HeaderMap,
// ) -> Result<Json<RetrieveHomeResponse>, ApiError> {
//     let uow = SqlxHomeUnitOfWork::new(&state.pool)
//         .await
//         .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     let uow = Mutex::new(uow);
//
//     let home_id = params.get("id").ok_or(ApiError::BadRequest)?;
//     let lang = headers
//         .get("Accept-Language")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or("zh");
//
//     let req = crate::domain::home::retrieve::Request {
//         home_id: home_id.to_string(),
//         language: lang.to_string(),
//         default_language: Language::ZH,
//     };
//
//     match crate::domain::home::retrieve::execute(uow, req).await {
//         Ok(service) => match service {
//             None => Err(ApiError::NotFound),
//             Some(s) => Ok(Json(RetrieveHomeResponse { home: s })),
//         },
//         Err(crate::domain::home::retrieve::Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(crate::domain::home::retrieve::Error::HomeNotFound) => Err(ApiError::NotFound),
//         Err(crate::domain::home::retrieve::Error::Unknown) => {
//             Err(ApiError::InternalServerError("Unknown error".to_string()))
//         }
//     }
// }
