use crate::api::work_logs::create::create_work_log;
use crate::api::work_logs::delete::delete_work_log;
use crate::api::work_logs::download::download;
use crate::api::work_logs::list::list_work_logs;
use crate::api::work_logs::update::update_work_log;
use crate::api::work_logs::update_status::update_work_log_status;
use crate::startup::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/work_logs", post(create_work_log).get(list_work_logs))
        .route("/work_logs", put(update_work_log))
        .route("/work_logs/status", put(update_work_log_status))
        .route("/work_logs/{id}", delete(delete_work_log))
        .route("/work_logs/download", get(download))
}
