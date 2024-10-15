use crate::api::api_error::ApiError;
use crate::domain::entities::ResourceType;
use crate::domain::member::upload_avatar::{execute, Error, Request};
use crate::startup::AppState;
use crate::uow::InDatabase;
use crate::utils::image::ImageUtil;
use axum::extract::{Multipart, Path, State};
use axum::Extension;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn upload_member_avatar(
    State(state): State<AppState>,
    Extension(image_util): Extension<Arc<ImageUtil>>,
    Path(params): Path<HashMap<String, String>>,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    let uow = InDatabase::new(&state.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    let uow = Mutex::new(uow);
    let member_id = params.get("id").ok_or(ApiError::BadRequest)?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::BadRequest)?
    {
        let content_type = field.content_type().unwrap().to_string();

        if content_type.starts_with("image/") {
            let data = field.bytes().await.map_err(|_| ApiError::BadRequest)?;
            let data = data.to_vec();
            let req = Request {
                id: member_id.to_string(),
                resource_type: ResourceType::Member,
                data,
            };

            match execute(uow, state.upload_folder.clone(), image_util, req).await {
                Ok(_) => {}
                Err(Error::NotFound) => return Err(ApiError::NotFound),
                Err(Error::BadRequest) => return Err(ApiError::BadRequest),
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
                Err(Error::Unknown) => {
                    return Err(ApiError::InternalServerError("Unknown error".to_string()));
                }
            }
            break;
        }
    }

    Ok(())
}
