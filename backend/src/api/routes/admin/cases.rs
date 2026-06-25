use crate::api::cases::create::create_case;
use crate::api::cases::delete::delete_case;
use crate::api::cases::list::list_cases;
use crate::api::cases::settle::settle;
use crate::api::cases::update::update_case;
use crate::startup::AppState;
use axum::routing::{delete, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cases", post(create_case).get(list_cases))
        .route("/cases/{id}", delete(delete_case).patch(update_case))
        .route("/case/{id}/settlement", post(settle))
}
