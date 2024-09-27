use crate::repositories::IAvatarRepository;
use crate::repositories::IContentRepository;
use crate::repositories::IResourceRepository;

/** Define a unit of work to organize all related repositories.
*
* - resource repository
* - content repository
* - avatar repository
*/
pub trait IResourceUnitOfWork {
    /** Resource repository stores the data by different types (e.g. members, services, home, etc.) */
    fn resource_repository(&mut self) -> &mut impl IResourceRepository;

    /** Content repository stores multiple language data */
    fn content_repository(&mut self) -> &mut impl IContentRepository;

    /** Avatar repository stores all avatars associated with the members. */
    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository;

    /** Commit the transaction */
    async fn commit(self) -> anyhow::Result<()>;
    /** Rollback the transaction */
    async fn rollback(self) -> anyhow::Result<()>;
}
