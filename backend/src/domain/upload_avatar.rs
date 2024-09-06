use crate::domain::entities::{AvatarData, AvatarJson, MemberID};
use crate::repositories::avatar_repository::IAvatarRepository;
use crate::repositories::member_repository::IMemberRepository;
use crate::uow::member::IMemberUnitOfWork;
use crate::utils::image::{IImage, Size};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Request {
    pub(crate) member_id: String,
    pub(crate) data: Vec<u8>,
}

pub(crate) enum Error {
    BadRequest,
    MemberNotFound,
    ImageProcess,
    CreateImage,
    Unknown,
}

async fn resize_image_and_save_it(
    util: Arc<dyn IImage + Sync + Send>,
    data: &[u8],
    size: Size,
    file_path: &str,
) -> Result<(), Error> {
    let image = util.resize(data, size).map_err(|_| Error::ImageProcess)?;
    util.save_to_file(file_path, image)
        .await
        .map_err(|_| Error::CreateImage)?;
    Ok(())
}
pub async fn execute<IUnitOfWork>(
    uow: Mutex<IUnitOfWork>,
    image_util: Arc<dyn IImage + Sync + Send>,
    req: Request,
) -> Result<String, Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    let mut lock = uow.lock().await;
    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;

    match lock.member_repository().contains(&member_id).await {
        Ok(exist) if exist == false => return Err(Error::MemberNotFound),
        Err(_) => return Err(Error::Unknown),
        Ok(_) => {}
    }

    // TODO: set up the output folder in config
    let out = "./uploads";

    let large_image_path = format!("{}/{}_128.png", out, member_id.0.as_str());
    resize_image_and_save_it(
        image_util.clone(),
        &req.data,
        Size {
            width: 128,
            height: 128,
        },
        &large_image_path,
    )
    .await?;
    let small_image_path = format!("{}/{}_48.png", out, member_id.0.as_str());
    resize_image_and_save_it(
        image_util.clone(),
        &req.data,
        Size {
            width: 48,
            height: 48,
        },
        &small_image_path,
    )
    .await?;

    let avatar_data = AvatarData {
        large_image: large_image_path,
        small_image: small_image_path,
    };
    let avatar_json = serde_json::value::to_value(&avatar_data).map_err(|_| Error::Unknown)?;

    let avatar_id = match lock
        .avatar_repository()
        .insert(member_id.clone(), AvatarJson(avatar_json))
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

    Ok(avatar_id.0)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::image::FakeImageUtil;
    use tokio::fs;
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use ulid::Ulid;

    async fn read_tests_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = format!("tests/{filename}");
        let mut file = File::open(&path).await.expect("get the test file");
        let metadata = fs::metadata(&path).await.expect("get the file metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).await.expect("read the test file");

        Ok(buffer)
    }

    #[tokio::test]
    async fn it_should_work_otherwise() {
        // Arrange
        let util = FakeImageUtil::new();
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();
        uow.member_repository()
            .insert(MemberID(member_id.clone()))
            .await
            .expect("can't insert a member");

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            member_id: member_id.clone(),
            data: buffer,
        };

        let res = execute(Mutex::new(uow), Arc::new(util), req).await;

        match res {
            Ok(id) => assert_eq!(id, member_id),
            Err(_) => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_image_is_invalid() {
        // Arrange
        let util = FakeImageUtil::new();
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();
        uow.member_repository()
            .insert(MemberID(member_id.clone()))
            .await
            .expect("can't insert a member");

        let req = Request {
            member_id: member_id.clone(),
            data: vec![1, 2, 3, 4],
        };

        let res = execute(Mutex::new(uow), Arc::new(util), req).await;

        match res {
            Err(Error::ImageProcess) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_file_fails_to_create() {
        // Arrange
        let util = FakeImageUtil::new().with_save_file_error();
        let mut uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();
        uow.member_repository()
            .insert(MemberID(member_id.clone()))
            .await
            .expect("can't insert a member");

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            member_id: member_id.clone(),
            data: buffer,
        };

        let res = execute(Mutex::new(uow), Arc::new(util), req).await;

        match res {
            Err(Error::CreateImage) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_an_error_when_member_is_not_exist() {
        // Arrange
        let util = FakeImageUtil::new().with_save_file_error();
        let uow = crate::uow::member::InMemoryMemberUnitOfWork::new();
        let member_id = Ulid::new().to_string();

        let buffer = read_tests_file("basn6a16.png")
            .await
            .expect("read the test file");

        let req = Request {
            member_id: member_id.clone(),
            data: buffer,
        };

        let res = execute(Mutex::new(uow), Arc::new(util), req).await;

        match res {
            Err(Error::MemberNotFound) => {}
            _ => unreachable!(),
        }
    }
}
