use crate::domain::entities::{AvatarData, AvatarJson, MemberID};
use crate::repositories::avatar_repository::IAvatarRepository;
use crate::uow::member::IMemberUnitOfWork;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub struct Request {
    pub(crate) member_id: String,
    pub(crate) data: Vec<u8>,
}

pub(crate) enum Error {
    BadRequest,
    Conflict,
    Unknown,
}
pub async fn execute<IUnitOfWork>(uow: Mutex<IUnitOfWork>, req: Request) -> Result<MemberID, Error>
where
    IUnitOfWork: IMemberUnitOfWork,
{
    // TODO: set up the output folder in config
    let out = "./upload";
    let member_id = MemberID::try_from(req.member_id).map_err(|_| Error::BadRequest)?;

    // TODO: resize the image

    let file_path = format!("{}/{}", out, member_id.0.as_str());
    let mut file = File::create(file_path.as_str())
        .await
        .map_err(|_| Error::Unknown)?;
    file.write_all(&req.data)
        .await
        .map_err(|_| Error::Unknown)?;

    let avatar_data = AvatarData {
        large_image: file_path.clone(),
        small_image: file_path,
    };
    let avatar_json = serde_json::value::to_value(&avatar_data).map_err(|_| Error::Unknown)?;

    let mut lock = uow.lock().await;

    let avatar_id = match lock
        .avatar_repository()
        .insert(member_id.clone(), AvatarJson(avatar_json))
        .await
    {
        Ok(id) => Ok(id),
        _ => Err(Error::Unknown),
    }?;

    Ok(avatar_id)
}
