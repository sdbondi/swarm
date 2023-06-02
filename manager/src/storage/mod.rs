use sqlx::SqlitePool;

pub struct ManagerStorage {
    connection: SqlitePool,
}

impl ManagerStorage {
    pub async fn init(url: &str) -> anyhow::Result<Self> {
        let connection = SqlitePool::connect(url).await?;
        sqlx::migrate!("./migrations").run(&connection).await?;
        Ok(Self { connection })
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
