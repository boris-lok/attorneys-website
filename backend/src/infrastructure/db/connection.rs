use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, Postgres};
use std::marker::PhantomData;

pub enum PgConn<'tx> {
    Pool(PoolConnection<Postgres>),
    Transaction(&'tx mut PgConnection),
}

impl<'tx> PgConn<'tx> {
    pub async fn from_pool(pool: &sqlx::Pool<Postgres>) -> anyhow::Result<Self> {
        Ok(PgConn::Pool(pool.acquire().await?))
    }

    pub fn as_conn(&mut self) -> &mut PgConnection {
        match self {
            PgConn::Pool(conn) => &mut *conn,
            PgConn::Transaction(tx) => &mut **tx,
        }
    }
}

pub struct PostgresRepo<'tx, T> {
    conn: PgConn<'tx>,
    _marker: PhantomData<T>,
}

impl<'tx, T> PostgresRepo<'tx, T> {
    pub async fn new(pool: &sqlx::Pool<Postgres>) -> anyhow::Result<Self> {
        let conn = PgConn::from_pool(pool).await?;
        Ok(Self {
            conn,
            _marker: PhantomData,
        })
    }

    pub fn with_tx(tx: &'tx mut PgConnection) -> Self {
        Self {
            conn: PgConn::Transaction(tx),
            _marker: PhantomData,
        }
    }

    pub(crate) fn get_conn(&mut self) -> &mut PgConnection {
        self.conn.as_conn()
    }
}

pub struct CaseRepo;
pub struct WorkLogRepo;
pub struct WorkLogMappingRepo;
pub struct UserRepo;
pub struct RoleRepo;
pub struct UserRoleRepo;
