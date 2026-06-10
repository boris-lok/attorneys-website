use crate::api::auth::login::login;
use crate::api::auth::logout::logout;
use crate::api::users::change_password::change_password;
use crate::api::users::list::list_users;
use crate::startup::AppState;
use axum::routing::{get, post, put};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/users", get(list_users))
        .route("/password", put(change_password))
}
