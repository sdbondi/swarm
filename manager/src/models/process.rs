use crate::process::Process;
use manifest::InstanceId;
use sqlx::{Executor, Row, Sqlite, Transaction};

#[derive(Debug, Clone)]
pub struct ProcessEntity {
    pub id: i64,
}

impl ProcessEntity {
    pub async fn create(
        tx: &mut Transaction<'_, Sqlite>,
        instance_name: &str,
        process: &Process,
    ) -> anyhow::Result<Self> {
        tx.execute(
            sqlx::query("INSERT INTO processes (instance_id, instance_name, pid) VALUES (?, ?, ?)")
                .bind(process.instance_id() as i64)
                .bind(instance_name)
                .bind(process.pid()),
        )
        .await?;

        let id = tx
            .fetch_one(sqlx::query("SELECT last_insert_rowid()"))
            .await?;

        Ok(Self { id: id.try_get(0)? })
    }

    pub async fn instance_exists(
        tx: &mut Transaction<'_, Sqlite>,
        instance_name: &str,
        instance_id: InstanceId,
    ) -> anyhow::Result<bool> {
        let res = tx
            .fetch_one(
                sqlx::query("SELECT count(1) FROM processes WHERE instance_name = ? AND instance_id = ? LIMIT 1")
                    .bind(instance_name)
                    .bind(instance_id as i64),
            )
            .await?;
        let exists: i64 = res.try_get(0)?;
        Ok(exists == 1i64)
    }

    pub async fn create_if_nexist(
        tx: &mut Transaction<'_, Sqlite>,
        instance_name: &str,
        process: &Process,
    ) -> anyhow::Result<bool> {
        let exists = Self::instance_exists(tx, instance_name, process.instance_id()).await?;
        if !exists {
            Self::create(tx, instance_name, process).await?;
        }
        Ok(exists)
    }
}
