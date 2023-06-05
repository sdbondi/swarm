use crate::storage::WriteAccess;
use sqlx::{Executor, Row, Sqlite, Transaction};

#[derive(Debug, Clone)]
pub struct ProcessEntity {
    id: i64,
}

impl ProcessEntity {
    pub async fn create(
        tx: &mut Transaction<'_, Sqlite>,
        name: &str,
        process: &crate::process::Process,
    ) -> anyhow::Result<Self> {
        tx.execute(
            sqlx::query("INSERT INTO processes (name, pid) VALUES (?, ?)")
                .bind(name)
                .bind(process.id() as i64),
        )
        .await?;

        let id = tx
            .fetch_one(sqlx::query("SELECT last_insert_rowid()"))
            .await?;

        Ok(Self { id: id.get(0) })
    }
}
