use crate::api::api_error::ApiError;
use crate::domain::entity::Claims;
use crate::domain::member::upload_avatar::{execute, Error};
use crate::domain::resources::entity::ResourceID;
use crate::domain::uow::common::UnitOfWorkFactory;
use crate::infrastructure::db::uow::PostgresUoWFactory;
use crate::startup::AppState;
use crate::utils::image::ImageUtil;
use axum::extract::{Multipart, Path, State};
use axum::Extension;
use std::collections::HashMap;
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn upload_member_avatar(
    _: Claims,
    State(state): State<AppState>,
    Extension(image_util): Extension<Arc<ImageUtil>>,
    Path(params): Path<HashMap<String, String>>,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    let member_id = params.get("id").ok_or(ApiError::BadRequest)?;
    let member_id = ResourceID::try_from(member_id.clone()).map_err(|_| ApiError::BadRequest)?;
    let mut processed = false;

    let uow = PostgresUoWFactory::new(state.pool.clone())
        .begin()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::BadRequest)?
    {
        let content_type = field.content_type().unwrap_or_default();

        if content_type.starts_with("image/") {
            let data = field.bytes().await.map_err(|_| ApiError::BadRequest)?;
            let data = data.to_vec();

            match execute(
                uow,
                image_util.clone(),
                &state.upload_folder,
                &member_id,
                data,
            )
            .await
            {
                Ok(_) => {}
                Err(Error::ImageProcess) => {
                    return Err(ApiError::InternalServerError(
                        "Can't resize image".to_string(),
                    ))
                }
                Err(Error::CreateImage) => {
                    return Err(ApiError::InternalServerError(
                        "Can't create image".to_string(),
                    ));
                }
                Err(Error::Unknown(s)) => {
                    return Err(ApiError::InternalServerError(s));
                }
            }
            processed = true;
            break;
        }
    }

    if !processed {
        return Err(ApiError::BadRequest);
    }

    Ok(())
}
