pub use resource::IResourceUnitOfWork;
pub use resource::InDatabase;

#[cfg(test)]
pub use resource::InMemory;

pub mod resource;
