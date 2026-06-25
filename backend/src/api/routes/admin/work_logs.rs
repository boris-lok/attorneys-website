use crate::api::work_logs::create::create_work_log;
use crate::api::work_logs::delete::delete_work_log;
use crate::api::work_logs::download::download;
use crate::api::work_logs::list::list_work_logs;
use crate::api::work_logs::update::update_work_log;
use crate::startup::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/work_logs", post(create_work_log).get(list_work_logs))
        .route("/work_logs/download", get(download))
        .route(
            "/work_logs/{id}",
            delete(delete_work_log).patch(update_work_log),
        )
}
