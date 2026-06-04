use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, Postgres};

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
