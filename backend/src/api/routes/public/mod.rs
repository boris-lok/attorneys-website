mod articles;
mod categories;
mod contacts;
mod home;
mod members;
mod services;

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
}
