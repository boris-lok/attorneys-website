use axum::http::StatusCode;
pub use health::health_check;
use tokio::sync::Mutex;

pub use home::create::create_home;
pub use home::list::list_home;
pub use home::retrieve::retrieve_home;
pub use home::update::update_home;

pub use member::create::create_member;
pub use member::delete::delete_member;
pub use member::list::list_members;
pub use member::retrieve::retrieve_member;
pub use member::update::update_member;
pub use member::upload_avatar::upload_member_avatar;

pub use service::create::create_service;
pub use service::delete::delete_service;
pub use service::list::list_services;
pub use service::retrieve::retrieve_service;
pub use service::update::update_service;

pub use contact::create::create_contact;
pub use contact::list::list_contact;
pub use contact::retrieve::retrieve_contact;

use crate::api::api_error::ApiError;
use crate::uow::IResourceUnitOfWork;
pub use article::create::create_article;
pub use article::delete::delete_article;
pub use article::list::list_articles;
pub use article::retrieve::retrieve_article;
pub use article::update::update_article;

mod api_error;

mod health;

mod home;

mod member;

mod service;

mod article;

mod contact;

/// A handler for updating the resource
async fn update_resource_handler(
    uow: Mutex<impl IResourceUnitOfWork>,
    req: crate::domain::resources::update::Request,
) -> Result<StatusCode, ApiError> {
    match crate::domain::resources::update::execute(uow, req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(crate::domain::resources::update::Error::NotFound) => Err(ApiError::NotFound),
        Err(crate::domain::resources::update::Error::BadRequest) => Err(ApiError::BadRequest),
        Err(crate::domain::resources::update::Error::Unknown(e)) => {
            Err(ApiError::InternalServerError(e.to_string()))
        }
    }
}
