pub use avatar_repository::IAvatarRepository;
pub use avatar_repository::InMemoryAvatarRepository;
pub use content_repository::IContentRepository;
pub use content_repository::InMemoryContentRepository;
pub use resource_repository::IResourceRepository;
pub use resource_repository::InMemoryResourceRepository;

mod avatar_repository;
mod content_repository;

mod resource_repository;
