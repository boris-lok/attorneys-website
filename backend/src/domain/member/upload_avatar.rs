use crate::domain::entities::ResourceID;
use crate::domain::uow::common::UnitOfWork;
use crate::domain::users::entity::{AvatarData, AvatarJson};
use crate::domain::users::repository::AvatarRepository;
use crate::utils::image::{IImage, Size};
use std::sync::Arc;
use uuid::Uuid;

pub enum Error {
    ImageProcess,
    CreateImage,
    Unknown(String),
}

async fn cleanup_images(paths: &[String]) {
    for path in paths {
        if let Err(e) = tokio::fs::remove_file(&path).await {
            tracing::warn!(path = %path, error = %e, "Failed to clean up orphaned image file");
        }
    }
}

pub async fn execute(
    mut uow: impl UnitOfWork,
    image_util: Arc<dyn IImage + Send + Sync>,
    out_folder: &str,
    id: &ResourceID,
    data: Vec<u8>,
) -> Result<String, Error> {
    let image_id = Uuid::new_v4().to_string();

    // 1. Resize in memory only — no side effects yet
    let large_image = image_util
        .resize(&data, Size::new(256, 256))
        .map_err(|_| Error::ImageProcess)?;
    let small_image = image_util
        .resize(&data, Size::new(64, 64))
        .map_err(|_| Error::ImageProcess)?;

    let large_img = format!("{}_{}.png", &image_id, "lg");
    let small_img = format!("{}_{}.png", &image_id, "sm");

    let avatar_data = AvatarData {
        large_image: large_img.clone(),
        small_image: small_img.clone(),
    };
    let json_data = AvatarJson::try_from(avatar_data).map_err(|e| Error::Unknown(e.to_string()))?;

    uow.avatar_repo()
        .create(id, json_data)
        .await
        .map_err(|e| Error::Unknown(e.to_string()))?;

    let large_path = format!("{}/{}", &out_folder, &large_img);
    let small_path = format!("{}/{}", &out_folder, &small_img);

    if tokio::try_join!(
        image_util.save_to_file(&large_path, large_image),
        image_util.save_to_file(&small_path, small_image)
    )
    .is_err()
    {
        cleanup_images(&[large_path, small_path]).await;
        return Err(Error::CreateImage);
    }

    if let Err(e) = uow.commit().await {
        cleanup_images(&[large_path.clone(), small_path.clone()]).await;
        return Err(Error::Unknown(e.to_string()));
    }

    Ok(image_id)
}

#[cfg(test)]
mod test {
    // use tokio::fs;
    // use tokio::fs::File;
    // use tokio::io::AsyncReadExt;

    // async fn read_tests_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    //     let path = format!("tests/{filename}");
    //     let mut file = File::open(&path).await.expect("get the test file");
    //     let metadata = fs::metadata(&path).await.expect("get the file metadata");
    //     let mut buffer = vec![0; metadata.len() as usize];
    //     file.read_exact(&mut buffer)
    //         .await
    //         .expect("read the test file");
    //
    //     Ok(buffer)
    // }

    // #[tokio::test]
    // async fn it_should_work_otherwise() {
    //     // Arrange
    //     let util = FakeImageUtil::new();
    //     let mut uow = crate::uow::InMemory::new();
    //     let id = Ulid::new().to_string();
    //     let id = ResourceID::try_from(id).unwrap();
    //     uow.resource_repository()
    //         .insert(id.clone(), ResourceType::Member, 0)
    //         .await
    //         .expect("can't insert a member");
    //
    //     let buffer = read_tests_file("basn6a16.png")
    //         .await
    //         .expect("read the test file");
    //
    //     let req = Request {
    //         id: id.to_string(),
    //         resource_type: ResourceType::Member,
    //         data: buffer,
    //     };
    //
    //     let out = Arc::new("".to_string());
    //
    //     let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;
    //
    //     match res {
    //         Ok(id) => assert_eq!(id, id.as_str()),
    //         Err(_) => unreachable!(),
    //     }
    // }
    //
    // #[tokio::test]
    // async fn it_should_return_an_error_when_image_is_invalid() {
    //     // Arrange
    //     let util = FakeImageUtil::new();
    //     let mut uow = crate::uow::InMemory::new();
    //     let id = Ulid::new().to_string();
    //     let id = ResourceID::try_from(id).unwrap();
    //     uow.resource_repository()
    //         .insert(id.clone(), ResourceType::Member, 0)
    //         .await
    //         .expect("can't insert a member");
    //
    //     let req = Request {
    //         id: id.to_string(),
    //         resource_type: ResourceType::Member,
    //         data: vec![1, 2, 3, 4],
    //     };
    //
    //     let out = Arc::new("".to_string());
    //
    //     let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;
    //
    //     match res {
    //         Err(Error::ImageProcess) => {}
    //         _ => unreachable!(),
    //     }
    // }
    //
    // #[tokio::test]
    // async fn it_should_return_an_error_when_file_fails_to_create() {
    //     // Arrange
    //     let util = FakeImageUtil::new().with_save_file_error();
    //     let mut uow = crate::uow::InMemory::new();
    //     let id = Ulid::new().to_string();
    //     let id = ResourceID::try_from(id).unwrap();
    //     uow.resource_repository()
    //         .insert(id.clone(), ResourceType::Member, 0)
    //         .await
    //         .expect("can't insert a member");
    //
    //     let buffer = read_tests_file("basn6a16.png")
    //         .await
    //         .expect("read the test file");
    //
    //     let req = Request {
    //         id: id.to_string(),
    //         resource_type: ResourceType::Member,
    //         data: buffer,
    //     };
    //
    //     let out = Arc::new("".to_string());
    //
    //     let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;
    //
    //     match res {
    //         Err(Error::CreateImage) => {}
    //         _ => unreachable!(),
    //     }
    // }
    //
    // #[tokio::test]
    // async fn it_should_return_an_error_when_member_is_not_exist() {
    //     // Arrange
    //     let util = FakeImageUtil::new().with_save_file_error();
    //     let uow = crate::uow::InMemory::new();
    //     let id = Ulid::new().to_string();
    //
    //     let buffer = read_tests_file("basn6a16.png")
    //         .await
    //         .expect("read the test file");
    //
    //     let req = Request {
    //         id: id.clone(),
    //         resource_type: ResourceType::Member,
    //         data: buffer,
    //     };
    //
    //     let out = Arc::new("".to_string());
    //
    //     let res = execute(Mutex::new(uow), out, Arc::new(util), req).await;
    //
    //     match res {
    //         Err(Error::NotFound) => {}
    //         _ => unreachable!(),
    //     }
    // }
}
