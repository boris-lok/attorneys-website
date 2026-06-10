use crate::domain::articles::entity::{
    ArticleData, ArticleEntity, CategoryData, CategoryEntity, SimpleArticleEntity,
};
use crate::domain::entity::Pagination;
use crate::domain::member::entity::{MemberData, MemberEntity, SimpleMemberEntity};
use crate::domain::resources::entity::{
    ContactData, ContactEntity, HomeData, HomeEntity, Language, ResourceID, ResourceType,
};
use crate::domain::resources::repository::{
    ResourceReadRepository, ResourceRepository, ResourceWriteRepository,
};
use crate::domain::services::entity::{ServiceData, ServiceEntity};
use crate::domain::users::entity::AvatarData;
use crate::infrastructure::db::connection::{PostgresRepo, ResourceRepo};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::{FromRow, PgConnection, QueryBuilder};

macro_rules! fetch_resource {
    ($query:expr, $pool:expr, $id:expr, $lang:expr, $from_sqlx:ty, $entity:ty) => {
        sqlx::query_as::<_, $from_sqlx>($query)
            .bind($id.as_str())
            .bind($lang.as_str())
            .fetch_optional($pool)
            .await?
            .map(<$entity>::from)
            .and_then(|e| serde_json::value::to_value(e).ok())
    };
}

const CREATE_SQL: &str = r"
  INSERT INTO 'resource' (id, created_at, resource_type, seq) VALUES ($1, now(), $2, $3);
";

const DELETE_SQL: &str = r"
  UPDATE 'resource' SET deleted_at = now() WHERE id = $1;
";

const UPDATE_SEQ_SQL: &str = r"
  UPDATE 'resource' SET seq = $2 WHERE id = $1;
";

type PostgresResourceRepo<'tx> = PostgresRepo<'tx, ResourceRepo>;

#[async_trait::async_trait]
impl<'tx> ResourceWriteRepository for PostgresResourceRepo<'tx> {
    async fn create(&mut self, id: &ResourceID, t: &ResourceType, seq: i32) -> anyhow::Result<()> {
        sqlx::query(CREATE_SQL)
            .bind(id.as_str())
            .bind(t.as_str())
            .bind(seq)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn delete(&mut self, id: &ResourceID) -> anyhow::Result<()> {
        sqlx::query(DELETE_SQL)
            .bind(id.as_str())
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }

    async fn update_seq(&mut self, id: &ResourceID, seq: i32) -> anyhow::Result<()> {
        sqlx::query(UPDATE_SEQ_SQL)
            .bind(id.as_str())
            .bind(seq)
            .execute(self.conn().await?)
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<'tx> ResourceReadRepository for PostgresResourceRepo<'tx> {
    async fn retrieve<T: DeserializeOwned + Serialize>(
        &mut self,
        id: &ResourceID,
        lang: &Language,
        kind: &ResourceType,
    ) -> anyhow::Result<Option<T>> {
        let default_query = r"
select
  resource.id, content.data, content.language, resource.seq
from resource, content
where resource.id = content.id
  and content.language = $2
  and resource.id = $1
  and resource.deleted_at is null;
        ";

        let member_query = r"
select
  resource.id,
  content.data,
  avatar.data as avatar,
  content.language,
  resource.seq
from resource, content
left join avatar on avatar.id = content.id
where resource.id = content.id
  and content.language = $2
  and resource.id = $1
  and resource.deleted_at is null;
        ";

        let res = match kind {
            ResourceType::Member => fetch_resource!(
                member_query,
                self.conn().await?,
                id,
                lang,
                MemberEntityFromSQLx,
                MemberEntity
            ),
            ResourceType::Service => fetch_resource!(
                default_query,
                self.conn().await?,
                id,
                lang,
                ServiceEntityFromSQLx,
                ServiceEntity
            ),
            ResourceType::Home => fetch_resource!(
                default_query,
                self.conn().await?,
                id,
                lang,
                HomeEntityFromSQLx,
                HomeEntity
            ),
            ResourceType::Contact => fetch_resource!(
                default_query,
                self.conn().await?,
                id,
                lang,
                ContactEntityFromSQLx,
                ContactEntity
            ),
            ResourceType::Article => fetch_resource!(
                default_query,
                self.conn().await?,
                id,
                lang,
                ArticleEntityFromSQLx,
                ArticleEntity
            ),
            ResourceType::Category => fetch_resource!(
                default_query,
                self.conn().await?,
                id,
                lang,
                CategoryEntityFromSQLx,
                CategoryEntity
            ),
        };

        res.map(serde_json::from_value::<T>)
            .transpose()
            .map_err(|e| anyhow::anyhow!("deserialisation into T failed: {e}"))
    }

    async fn list<T: DeserializeOwned + Serialize>(
        &mut self,
        lang: &Language,
        kind: &ResourceType,
        filter_str: &str,
        page: &Pagination,
    ) -> anyhow::Result<Vec<T>> {
        let offset = pagination_to_offset(page);

        let values =
            fetch_resource_rows(self.conn().await?, lang, filter_str, kind, &offset).await?;

        Ok(values
            .into_iter()
            .map(serde_json::from_value::<T>)
            .filter_map(Result::ok)
            .collect())
    }

    async fn count(
        &mut self,
        lang: &Language,
        kind: &ResourceType,
        filter_str: &str,
    ) -> anyhow::Result<i64> {
        count_resource_rows(self.conn().await?, lang, kind, filter_str).await
    }
}

async fn fetch_resource_rows(
    conn: &mut PgConnection,
    lang: &Language,
    filter_str: &str,
    kind: &ResourceType,
    offset: &str,
) -> anyhow::Result<Vec<serde_json::Value>> {
    let base_qb = || {
        let mut qb = QueryBuilder::new(
            r#"select resource.id, content.data, content.language, resource.seq
               from resource
               join content on resource.id = content.id
               where resource.deleted_at is null"#,
        );
        qb.push(" and content.language = ");
        qb.push_bind(lang.as_str());
        qb.push(" and resource.resource_type = ");
        qb.push_bind(kind.as_str());
        qb.push(filter_str);
        qb.push(" order by seq, resource.created_at desc ");
        qb.push(offset);
        qb
    };

    let rows: Vec<serde_json::Value> = match kind {
        ResourceType::Member => {
            let mut qb = QueryBuilder::new(
                r#"select resource.id, content.data->>'name' as name,
                          avatar.data->>'small_image' as avatar, resource.seq
                   from resource
                   join content on resource.id = content.id
                   left join avatar on content.id = avatar.id
                   where resource.deleted_at is null"#,
            );
            qb.push(" and content.language = ");
            qb.push_bind(lang.as_str());
            qb.push(" and resource.resource_type = ");
            qb.push_bind(kind.as_str());
            qb.push(" order by seq, resource.created_at desc ");
            qb.push(offset);

            qb.build_query_as::<SimpleMemberEntityFromSQLx>()
                .fetch_all(conn)
                .await?
                .into_iter()
                .map(SimpleMemberEntity::from)
                .filter_map(|e| serde_json::to_value(e).ok())
                .collect()
        }
        ResourceType::Article => {
            let mut qb = QueryBuilder::new(
                r#"select resource.id, content.data->>'title' as title,
                          content.created_at, content.language, resource.seq
                   from resource
                   join content on resource.id = content.id
                   where resource.deleted_at is null"#,
            );
            qb.push(" and content.language = ");
            qb.push_bind(lang.as_str());
            qb.push(" and resource.resource_type = ");
            qb.push_bind(kind.as_str());
            qb.push(filter_str);
            qb.push(" order by seq, resource.created_at desc ");
            qb.push(offset);

            qb.build_query_as::<SimpleArticleEntityFromSQLx>()
                .fetch_all(conn)
                .await?
                .into_iter()
                .map(SimpleArticleEntity::from)
                .filter_map(|e| serde_json::to_value(e).ok())
                .collect()
        }
        ResourceType::Service => base_qb()
            .build_query_as::<ServiceEntityFromSQLx>()
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(ServiceEntity::from)
            .filter_map(|e| serde_json::to_value(e).ok())
            .collect(),
        ResourceType::Home => base_qb()
            .build_query_as::<HomeEntityFromSQLx>()
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(HomeEntity::from)
            .filter_map(|e| serde_json::to_value(e).ok())
            .collect(),
        ResourceType::Contact => base_qb()
            .build_query_as::<ContactEntityFromSQLx>()
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(ContactEntity::from)
            .filter_map(|e| serde_json::to_value(e).ok())
            .collect(),
        ResourceType::Category => base_qb()
            .build_query_as::<CategoryEntityFromSQLx>()
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(CategoryEntity::from)
            .filter_map(|e| serde_json::to_value(e).ok())
            .collect(),
    };

    Ok(rows)
}

fn pagination_to_offset(page: &Pagination) -> String {
    match page {
        Pagination::All => ";".to_string(),
        Pagination::Single => "limit 1;".to_string(),
        Pagination::Page(p) => format!("offset {} limit {};", p.page * p.size, p.size),
    }
}

async fn count_resource_rows(
    conn: &mut PgConnection,
    lang: &Language,
    kind: &ResourceType,
    filter_str: &str,
) -> anyhow::Result<i64> {
    let mut qb = QueryBuilder::new(
        r#"select count(*)
           from resource
           join content on resource.id = content.id
           where resource.deleted_at is null"#,
    );
    qb.push(" and content.language = ").push_bind(lang.as_str());
    qb.push(" and resource.resource_type = ")
        .push_bind(kind.as_str());
    qb.push(filter_str);

    let (count,): (i64,) = qb.build_query_as().fetch_one(conn).await?;

    Ok(count)
}

impl<'tx> ResourceRepository for PostgresResourceRepo<'tx> {}

#[derive(Debug, FromRow)]
pub struct MemberEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<MemberData>,
    pub avatar: Option<sqlx::types::Json<AvatarData>>,
    pub seq: i16,
}

impl From<MemberEntityFromSQLx> for MemberEntity {
    fn from(value: MemberEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            avatar: value.avatar.map(|a| a.0),
            seq: value.seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ServiceEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ServiceData>,
    pub seq: i16,
}

impl From<ServiceEntityFromSQLx> for ServiceEntity {
    fn from(value: ServiceEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            seq: value.seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct HomeEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<HomeData>,
}

impl From<HomeEntityFromSQLx> for HomeEntity {
    fn from(value: HomeEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ContactEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ContactData>,
}

impl From<ContactEntityFromSQLx> for ContactEntity {
    fn from(value: ContactEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0.data,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ArticleEntityFromSQLx {
    pub id: String,
    pub language: String,
    pub data: sqlx::types::Json<ArticleData>,
    pub seq: i16,
}

impl From<ArticleEntityFromSQLx> for ArticleEntity {
    fn from(value: ArticleEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            seq: value.seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct CategoryEntityFromSQLx {
    pub id: String,
    pub data: sqlx::types::Json<CategoryData>,
    pub language: String,
    pub seq: i16,
}

impl From<CategoryEntityFromSQLx> for CategoryEntity {
    fn from(value: CategoryEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            language: value.language.trim().to_owned(),
            data: value.data.0,
            seq: value.seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SimpleMemberEntityFromSQLx {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub seq: i16,
}

impl From<SimpleMemberEntityFromSQLx> for SimpleMemberEntity {
    fn from(value: SimpleMemberEntityFromSQLx) -> Self {
        Self {
            id: value.id.to_owned(),
            name: value.name.to_owned(),
            avatar: value.avatar,
            seq: value.seq,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SimpleArticleEntityFromSQLx {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub seq: i16,
}

impl From<SimpleArticleEntityFromSQLx> for SimpleArticleEntity {
    fn from(value: SimpleArticleEntityFromSQLx) -> Self {
        Self {
            id: value.id.trim().to_owned(),
            title: value.title.trim().to_owned(),
            language: value.language.trim().to_owned(),
            created_at: value.created_at.timestamp_millis(),
            seq: value.seq,
        }
    }
}
