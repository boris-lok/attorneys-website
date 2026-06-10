mod articles;
mod cases;
mod categories;
mod contacts;
mod home;
mod members;
mod services;
mod users;
mod work_logs;

use crate::startup::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(members::router())
        .merge(services::router())
        .merge(home::router())
        .merge(contacts::router())
        .merge(articles::router())
        .merge(categories::router())
        .merge(users::router())
        .merge(cases::router())
        .merge(work_logs::router())
}
