// use crate::api::api_error::ApiError;
// use crate::domain::member::entities::{Language, SimpleMember};
// use crate::domain::member::list::Error;
// use crate::startup::AppState;
// use crate::uow::member::SqlxMemberUnitOfWork;
// use axum::extract::State;
// use axum::http::HeaderMap;
// use axum::Json;
// use serde::Serialize;
// use tokio::sync::Mutex;
//
// #[derive(Debug, Serialize)]
// pub(crate) struct ListMembersResponse {
//     members: Vec<SimpleMember>,
// }
// pub async fn list_members(
//     State(state): State<AppState>,
//     headers: HeaderMap,
// ) -> Result<Json<ListMembersResponse>, ApiError> {
//     let uow = SqlxMemberUnitOfWork::new(&state.pool)
//         .await
//         .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
//     let uow = Mutex::new(uow);
//
//     let lang = headers
//         .get("Accept-Language")
//         .and_then(|v| v.to_str().ok())
//         .unwrap_or("zh");
//
//     let req = crate::domain::member::list::Request {
//         language: lang.to_string(),
//         default_language: Language::ZH,
//     };
//
//     match crate::domain::member::list::execute(uow, req).await {
//         Ok(members) => Ok(Json(ListMembersResponse { members })),
//         Err(Error::BadRequest) => Err(ApiError::BadRequest),
//         Err(Error::Unknown(e)) => Err(ApiError::InternalServerError(e.to_string())),
//     }
// }
