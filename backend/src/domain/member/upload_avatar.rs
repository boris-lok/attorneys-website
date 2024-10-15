use crate::domain::entities::{ResourceID, ResourceType};
use crate::domain::member::entities::{AvatarData, AvatarJson};
use crate::repositories::IAvatarRepository;
use crate::repositories::IResourceRepository;
use crate::uow::IResourceUnitOfWork;
use crate::utils::image::{IImage, Size};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Request {
    pub(crate) id: String,
    pub(crate) resource_type: ResourceType,
    pub(crate) data: Vec<u8>,
}

pub(crate) enum Error {
    BadRequest,
    NotFound,
    ImageProcess,
    CreateImage,
    Unknown,
}

async fn resize_image_and_save_it(
    util: Arc<dyn IImage + Sync + Send>,
    data: &[u8],
    size: Size,
    out: &str,
    id: &str,
) -> Result<String, Error> {
    let image = util.resize(data, size).map_err(|_| Error::ImageProcess)?;
    let path = format!("{}/{}_{}_{}.png", out, id, image.width(), image.height());
    util.save_to_file(path.as_str(), image)
        .await
        .map_err(|_| Error::CreateImage)?;
    Ok(path)
}
pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    out: Arc<String>,
    image_util: Arc<dyn IImage + Sync + Send>,
    req: Request,
) -> Result<String, Error>
where
    IUnitOfWork: IResourceUnitOfWork,
{
    let mut lock = uow.lock().await;
    let member_id = ResourceID::try_from(req.id).map_err(|_| Error::BadRequest)?;

    match lock
        .resource_repository()
        .contains(&member_id, &req.resource_type)
        .await
    {
        Ok(exist) if !exist => return Err(Error::NotFound),
        Err(_) => return Err(Error::Unknown),
        Ok(_) => {}
    };

    let large_image_path = resize_image_and_save_it(
        image_util.clone(),
        &req.data,
        Size::new(128, 128),
        out.as_ref(),
        member_id.as_str(),
    )
    .await?;

    let small_image_path = resize_image_and_save_it(
        image_util.clone(),
        &req.data,
        Size::new(48, 48),
        out.as_ref(),
        member_id.as_str(),
    )
    .await?;

    let avatar_data = AvatarData {
        large_image: large_image_path,
        small_image: small_image_path,
    };
    let avatar_json = AvatarJson::try_from(avatar_data).map_err(|_| Error::Unknown)?;

    let avatar_id = match lock
        .avatar_repository()
        .insert(member_id.clone(), avatar_json)
        .await
    {
        Ok(id) => Ok(id),
        _ => Err(Error::Unknown),
    }?;

    drop(lock);
    uow.into_inner()
        .commit()
        .await
        .map_err(|_| Error::Unknown)?;

    Ok(avatar_id.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::image::FakeImageUtil;
    use tokio::fs;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    use ulid::Ulid;

    async fn read_tests_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = format!("tests/{filename}");
        let mut file = File::open(&path).await.expect("get the test file");
        let metadata = fs::metadata(&path).await.expect("get the file metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)
            .await
            .expect("read the test file");

        Ok(buffer)
    }

    #[tokio::test]
    async fn it_should_work_otherwise() {
        // Arrange
        let util = FakeImageUtil::new();
        let mut uow = crate::uow::InMemory::new();
        let id = Ulid::new().to_string();
        let id = ResourceID::try_from(id).unwrap();
        uow.resource_repository()
            .insert(id.clone(), ResourceType::Member)
            .await
            .expect("can't insert a member");

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            id: id.as_str().to_string(),
            resource_type: ResourceType::Member,
            data: buffer,
        };

        let out = Arc::new("".to_string());

        let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;

        match res {
            Ok(id) => assert_eq!(id, id.as_str()),
            Err(_) => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_image_is_invalid() {
        // Arrange
        let util = FakeImageUtil::new();
        let mut uow = crate::uow::InMemory::new();
        let id = Ulid::new().to_string();
        let id = ResourceID::try_from(id).unwrap();
        uow.resource_repository()
            .insert(id.clone(), ResourceType::Member)
            .await
            .expect("can't insert a member");

        let req = Request {
            id: id.as_str().to_string(),
            resource_type: ResourceType::Member,
            data: vec![1, 2, 3, 4],
        };

        let out = Arc::new("".to_string());

        let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;

        match res {
            Err(Error::ImageProcess) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_file_fails_to_create() {
        // Arrange
        let util = FakeImageUtil::new().with_save_file_error();
        let mut uow = crate::uow::InMemory::new();
        let id = Ulid::new().to_string();
        let id = ResourceID::try_from(id).unwrap();
        uow.resource_repository()
            .insert(id.clone(), ResourceType::Member)
            .await
            .expect("can't insert a member");

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            id: id.as_str().to_string(),
            resource_type: ResourceType::Member,
            data: buffer,
        };

        let out = Arc::new("".to_string());

        let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;

        match res {
            Err(Error::CreateImage) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_member_is_not_exist() {
        // Arrange
        let util = FakeImageUtil::new().with_save_file_error();
        let uow = crate::uow::InMemory::new();
        let id = Ulid::new().to_string();

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            id: id.clone(),
            resource_type: ResourceType::Member,
            data: buffer,
        };

        let out = Arc::new("".to_string());

        let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }
}
