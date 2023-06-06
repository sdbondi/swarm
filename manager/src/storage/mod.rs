use sqlx::migrate::MigrateDatabase;
use sqlx::pool::PoolConnection;
use sqlx::{Sqlite, SqlitePool};

pub struct ManagerStorage {
    connection: SqlitePool,
}

impl ManagerStorage {
    pub async fn init(url: &str) -> anyhow::Result<Self> {
        if !Sqlite::database_exists(url).await? {
            Sqlite::create_database(url).await?;
        }
        let connection = SqlitePool::connect(url).await?;
        sqlx::migrate!("./migrations").run(&connection).await?;
        Ok(Self { connection })
    }

    pub async fn get_connection(&self) -> anyhow::Result<PoolConnection<Sqlite>> {
        let conn = self.connection.acquire().await?;
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let _ = ManagerStorage::init(":memory:").await;
    }
}
