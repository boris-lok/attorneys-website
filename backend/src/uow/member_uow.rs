use crate::repositories::content_repository::IContentRepository;
use crate::repositories::member_repository::IMemberRepository;
use std::sync::Arc;

pub struct MemberUOW {
    pub(crate) member_repository: Arc<dyn IMemberRepository>,
    pub(crate) content_repository: Arc<dyn IContentRepository>,
}
