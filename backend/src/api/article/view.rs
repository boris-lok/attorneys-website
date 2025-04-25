use crate::api::api_error::ApiError;
use crate::repositories::{Connection, SqlxArticleViewsRepository};
use crate::startup::AppState;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::Mutex;

pub async fn view_article(
    State(state): State<AppState>,
    Path(params): Path<HashMap<String, String>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<StatusCode, ApiError> {
    let conn = Connection::Pool(state.pool);
    let repo = SqlxArticleViewsRepository::new(conn);

    let article_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let user_agent = user_agent.to_string();

    let req = crate::domain::articles::add_view::Request {
        article_id: article_id.to_string(),
        ip: addr.ip(),
        user_agent,
    };

    match crate::domain::articles::add_view::execute(Mutex::new(repo), req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(crate::domain::articles::add_view::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e))
        }
    }
}
