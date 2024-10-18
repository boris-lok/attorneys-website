pub use avatar_repository::IAvatarRepository;
pub use avatar_repository::InMemoryAvatarRepository;
pub use avatar_repository::SqlxAvatarRepository;

pub use content_repository::IContentRepository;
pub use content_repository::InMemoryContentRepository;
pub use content_repository::SqlxContentRepository;

pub use resource_repository::IResourceRepository;
pub use resource_repository::InMemoryResourceRepository;
pub use resource_repository::SqlxResourceRepository;

pub use user_repository::IUserRepository;
pub use user_repository::InMemoryUserRepository;

mod avatar_repository;

mod content_repository;

mod resource_repository;

mod user_repository;
