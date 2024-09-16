use crate::domain::member::entities::{ContentID, Language, Member, MemberID, SimpleMember};
use crate::repositories::avatar_repository::{
    IAvatarRepository, InMemoryAvatarRepository, SqlxAvatarRepository,
};
use crate::repositories::content_repository::{
    IContentRepository, InMemoryContentRepository, SqlxContentRepository,
};
use crate::repositories::member_repository::{
    IMemberRepository, InMemoryMemberRepository, SqlxMemberRepository,
};
use anyhow::anyhow;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait IMemberUnitOfWork {
    fn member_repository(&mut self) -> &mut impl IMemberRepository;
    fn content_repository(&mut self) -> &mut impl IContentRepository;
    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository;

    // Get a member with full details
    async fn get_member<'id, 'lang>(
        &mut self,
        member_id: &'id MemberID,
        language: &'lang Language,
    ) -> anyhow::Result<Option<Member>>;

    // Get all members for a specific language
    async fn get_all_members_by_language(
        &mut self,
        language: &Language,
    ) -> anyhow::Result<Vec<SimpleMember>>;

    async fn commit(mut self) -> anyhow::Result<()>;
    #[allow(dead_code)]
    async fn rollback(mut self) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct InMemoryMemberUnitOfWork {
    error: bool,
    member_repository: Option<InMemoryMemberRepository>,
    content_repository: Option<InMemoryContentRepository>,
    avatar_repository: Option<InMemoryAvatarRepository>,
}

#[cfg(test)]
impl InMemoryMemberUnitOfWork {
    pub fn new() -> Self {
        Self {
            error: false,
            member_repository: None,
            content_repository: None,
            avatar_repository: None,
        }
    }

    pub fn with_error(mut self) -> Self {
        self.member_repository = self.member_repository.map(|r| r.with_error());
        self.content_repository = self.content_repository.map(|r| r.with_error());
        self.avatar_repository = self.avatar_repository.map(|r| r.with_error());
        Self {
            error: true,
            ..self
        }
    }
}

#[async_trait::async_trait]
impl IMemberUnitOfWork for InMemoryMemberUnitOfWork {
    fn member_repository(&mut self) -> &mut impl IMemberRepository {
        if self.member_repository.is_none() {
            let member_repo = if self.error {
                InMemoryMemberRepository::new().with_error()
            } else {
                InMemoryMemberRepository::new()
            };
            self.member_repository = Some(member_repo);
        }
        self.member_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut impl IContentRepository {
        if self.content_repository.is_none() {
            let content_repo = if self.error {
                InMemoryContentRepository::new().with_error()
            } else {
                InMemoryContentRepository::new()
            };
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository {
        if self.avatar_repository.is_none() {
            let avatar_repo = if self.error {
                InMemoryAvatarRepository::new().with_error()
            } else {
                InMemoryAvatarRepository::new()
            };
            self.avatar_repository = Some(avatar_repo);
        }
        self.avatar_repository.as_mut().unwrap()
    }

    async fn get_member<'id, 'lang>(
        &mut self,
        member_id: &'id MemberID,
        language: &'lang Language,
    ) -> anyhow::Result<Option<Member>> {
        let member = self
            .member_repository
            .as_mut()
            .unwrap()
            .get(member_id)
            .await?;
        let content_id = ContentID::try_from(member_id.clone()).unwrap();
        let content = self
            .content_repository
            .as_mut()
            .unwrap()
            .get(&content_id, language)
            .await?;
        let avatar = self
            .avatar_repository
            .as_mut()
            .unwrap()
            .get(member_id)
            .await?;

        match (member, content, avatar) {
            (None, None, None) => Ok(None),
            (Some(_), None, None) => Ok(None),
            (Some(member_id), Some(content), None) => Ok(Some(Member {
                member_id,
                content: content.0,
                avatar_data: None,
            })),
            (Some(member_id), Some(content), Some(avatar)) => Ok(Some(Member {
                member_id,
                content: content.0,
                avatar_data: Some(avatar.get()),
            })),
            _ => unreachable!(),
        }
    }

    async fn get_all_members_by_language(
        &mut self,
        language: &Language,
    ) -> anyhow::Result<Vec<SimpleMember>> {
        let res = self
            .content_repository
            .as_mut()
            .unwrap()
            .list(language)
            .await?
            .iter()
            .map(|(id, name)| SimpleMember {
                member_id: id.clone(),
                name: name.clone(),
                avatar: None,
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    async fn commit(self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn rollback(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SqlxMemberUnitOfWork<'tx> {
    pool: &'tx PgPool,
    tx: Arc<Mutex<Transaction<'tx, Postgres>>>,
    member_repository: Option<SqlxMemberRepository<'tx>>,
    content_repository: Option<SqlxContentRepository<'tx>>,
    avatar_repository: Option<SqlxAvatarRepository<'tx>>,
}

impl<'tx> SqlxMemberUnitOfWork<'tx> {
    pub async fn new(pool: &'tx PgPool) -> anyhow::Result<Self> {
        let tx = pool.begin().await?;
        let tx = Arc::new(Mutex::new(tx));

        Ok(Self {
            pool,
            tx,
            member_repository: None,
            content_repository: None,
            avatar_repository: None,
        })
    }
}

#[async_trait::async_trait]
impl<'tx> IMemberUnitOfWork for SqlxMemberUnitOfWork<'tx> {
    fn member_repository(&mut self) -> &mut impl IMemberRepository {
        if self.member_repository.is_none() {
            let member_repo = SqlxMemberRepository::new(Arc::downgrade(&self.tx));
            self.member_repository = Some(member_repo);
        }
        self.member_repository.as_mut().unwrap()
    }

    fn content_repository(&mut self) -> &mut impl IContentRepository {
        if self.content_repository.is_none() {
            // Use Arc::downgrade to obtain a weak reference to the transaction
            // if we don't do this, when we call the commit/rollback method will fail.
            // It can't `try_unwrap` because there are at least two strong references, preventing
            // the use of `try_unwrap`.
            //
            // If we want to use strong references, then we need to drop the repository
            // when we try to call commit/rollback methods.
            let content_repo = SqlxContentRepository::new(Arc::downgrade(&self.tx));
            self.content_repository = Some(content_repo);
        }
        self.content_repository.as_mut().unwrap()
    }

    fn avatar_repository(&mut self) -> &mut impl IAvatarRepository {
        if self.avatar_repository.is_none() {
            let avatar_repo = SqlxAvatarRepository::new(Arc::downgrade(&self.tx));
            self.avatar_repository = Some(avatar_repo);
        }
        self.avatar_repository.as_mut().unwrap()
    }

    async fn get_member<'id, 'lang>(
        &mut self,
        member_id: &'id MemberID,
        language: &'lang Language,
    ) -> anyhow::Result<Option<Member>> {
        let query = r#"
select member.id as member_id, content.data as content, avatar.data as avatar_data
from member,
     content
         left join avatar on avatar.id = content.id
where member.id = content.id
  and content.language = $2
  and member.id = $1;
        "#;

        let res = sqlx::query_as::<_, Member>(query)
            .bind(member_id.as_str())
            .bind(language.as_str())
            .fetch_optional(self.pool)
            .await?;

        Ok(res)
    }

    async fn get_all_members_by_language(
        &mut self,
        language: &Language,
    ) -> anyhow::Result<Vec<SimpleMember>> {
        let query = r#"select member.id as member_id,
content.data->>'name' as name,
avatar.data->>'small_image' as avatar
from member,
     content
left join avatar on content.id = avatar.id
where member.id = content.id
and content.language = $1
order by seq;
        "#;

        let res = sqlx::query_as::<_, SimpleMember>(query)
            .bind(language.as_str())
            .fetch_all(self.pool)
            .await?;

        Ok(res)
    }

    async fn commit(mut self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().commit().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't commit transaction")),
        }
    }

    async fn rollback(mut self) -> anyhow::Result<()> {
        match Arc::try_unwrap(self.tx) {
            Ok(lock) => {
                lock.into_inner().rollback().await?;
                Ok(())
            }
            Err(_) => Err(anyhow!("can't rollback transaction")),
        }
    }
}
