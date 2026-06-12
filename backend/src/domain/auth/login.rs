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

pub async fn execute(
    query: &impl Query,
    session: Arc<dyn SessionStore + Send + Sync>,
    jwt_encoding_key: Arc<EncodingKey>,
    creds: Credentials,
    jwt_expire_duration: time::Duration,
) -> Result<String, Error> {
    let id = validate_credentials(&mut query.user_repo(), creds)
        .await
        .map_err(|_| Error::InvalidCredentials)?;

    let exp = time::OffsetDateTime::now_utc() + jwt_expire_duration;

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
        exp: exp.unix_timestamp() as usize,
        roles,
        nickname,
    };

    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jwt_encoding_key)
        .map_err(|_| Error::CreateJWTFailed)?;

    session
        .create_session(
            &id.to_string(),
            &format!("{}", exp.unix_timestamp()),
            jwt_expire_duration.whole_seconds() as u64,
        )
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(token)
}
