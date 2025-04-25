use crate::repositories::IArticleViewsRepository;
use std::net::IpAddr;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct Request {
    pub article_id: String,
    pub ip: IpAddr,
    pub user_agent: String,
}

pub enum Error {
    Unknown(String),
}

pub async fn execute(
    repo: Mutex<impl IArticleViewsRepository>,
    req: Request,
) -> Result<Uuid, Error> {
    let id = {
        let lock = repo.lock().await;
        lock.save(req.article_id, req.ip, req.user_agent)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
    };

    Ok(id)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::InMemoryArticleViewsRepository;

    #[tokio::test]
    async fn it_should_work_otherwise() {
        let repo = InMemoryArticleViewsRepository::new();

        let req = Request {
            article_id: "1".to_string(),
            ip: "127.0.0.1".parse().unwrap(),
            user_agent: "test".to_string(),
        };

        let resp = execute(Mutex::new(repo), req).await;

        match resp {
            Ok(_) => {}
            Err(_) => unreachable!(),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_otherwise() {
        let repo = InMemoryArticleViewsRepository::new().with_error();

        let req = Request {
            article_id: "1".to_string(),
            ip: "127.0.0.1".parse().unwrap(),
            user_agent: "test".to_string(),
        };

        let resp = execute(Mutex::new(repo), req).await;

        if resp.is_ok() {
            unreachable!()
        }
    }
}
