use crate::api::api_error::ApiError;
use crate::api::cases::create::{
    __path_create_case, create_case, CreateCaseRequest, CreateCaseResponse,
};
use crate::api::cases::delete::{__path_delete_case, delete_case};
use crate::api::cases::list::{__path_list_cases, list_cases, ListCasesResponse};
use crate::api::cases::settle::settle;
use crate::api::cases::update::update_case;
use crate::domain::cases::entity::Case;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_case, delete_case, list_cases),
    components(schemas(
        CreateCaseRequest,
        CreateCaseResponse,
        ApiError,
        ListCasesResponse,
        Case
    ))
)]
pub struct CasesApi;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cases", post(create_case).get(list_cases))
        .route("/cases/{id}", delete(delete_case).patch(update_case))
        .route("/case/{id}/settlement", post(settle))
}
