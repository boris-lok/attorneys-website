use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, Postgres};
use std::marker::PhantomData;

pub enum PgConn<'tx> {
    Pool(&'tx sqlx::PgPool),
    Acquired(PoolConnection<Postgres>),
    Transaction(&'tx mut PgConnection),
}

pub struct PostgresRepo<'tx, T> {
    conn: PgConn<'tx>,
    _marker: PhantomData<T>,
}

impl<'tx, T> PostgresRepo<'tx, T> {
    pub fn from_pool(pool: &'tx sqlx::PgPool) -> Self {
        Self {
            conn: PgConn::Pool(pool),
            _marker: PhantomData,
        }
    }

    pub fn with_tx(tx: &'tx mut PgConnection) -> Self {
        Self {
            conn: PgConn::Transaction(tx),
            _marker: PhantomData,
        }
    }

    pub(crate) async fn conn(&mut self) -> anyhow::Result<&mut PgConnection> {
        if let PgConn::Pool(pool) = self.conn {
            self.conn = PgConn::Acquired(pool.acquire().await?);
        }

        match &mut self.conn {
            PgConn::Transaction(tx) => Ok(*tx),
            PgConn::Acquired(c) => Ok(c.as_mut()),
            PgConn::Pool(_) => unreachable!(),
        }
    }
}

pub struct CaseRepo;
pub struct WorkLogRepo;
pub struct WorkLogMappingRepo;
pub struct UserRepo;
pub struct RoleRepo;
pub struct UserRoleRepo;
pub struct AvatarRepo;
pub struct ArticleViewRepo;
pub struct ContentRepo;
pub struct ResourceRepo;
