use sqlx::migrate::MigrateDatabase;
use sqlx::{Connection, Executor, Sqlite, SqlitePool};

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

    pub async fn write_access(&self) -> anyhow::Result<WriteAccess> {
        let mut conn = self.connection.acquire().await?;
        Ok(WriteAccess { connection: conn })
    }
}

pub struct WriteAccess {
    pub connection: sqlx::pool::PoolConnection<sqlx::Sqlite>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let _ = ManagerStorage::init(":memory:").await;
    }
}
