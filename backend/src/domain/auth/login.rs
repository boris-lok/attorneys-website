use crate::domain::entity::Claims;
use crate::domain::session::store::SessionStore;
use crate::domain::uow::common::Query;
use crate::domain::users::authentication::{validate_credentials, Credentials};
use crate::domain::users::repository::{UserReadRepository, UserRoleReadRepository};
use jsonwebtoken::EncodingKey;
use std::sync::Arc;

pub enum Error {
    CreateJWTFailed,
    InvalidCredentials,
    Unknown(String),
}

const EXPIRATION_TIME: chrono::Duration = chrono::Duration::days(30);

pub async fn execute(
    query: &impl Query,
    session: Arc<dyn SessionStore + Send + Sync>,
    jwt_encoding_key: Arc<EncodingKey>,
    creds: Credentials,
) -> Result<String, Error> {
    let id = validate_credentials(&mut query.user_repo(), creds)
        .await
        .map_err(|_| Error::InvalidCredentials)?;

    let exp = chrono::Utc::now() + EXPIRATION_TIME;

    let roles = query
        .user_role_repo()
        .get_user_roles(&id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;
    let nickname = query
        .user_repo()
        .get_user_nickname(&id)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    let claims = Claims {
        sub: id.clone().to_string(),
        exp: exp.timestamp() as usize,
        roles,
        nickname,
    };

    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jwt_encoding_key)
        .map_err(|_| Error::CreateJWTFailed)?;

    session
        .create_session(&id, exp.timestamp())
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(token)
}
