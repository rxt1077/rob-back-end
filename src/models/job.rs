use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use anyhow::Result;
//use sqlx::types::chrono::DateTime;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub group_id: i32,
    pub test_id: i32,
    pub output: String,
}

impl Job {
    // creates a new job (id doesn't matter, the new one will be returned)
    pub async fn create(job: Job, pool: &PgPool) -> Result<Job> {
        Ok(sqlx::query_as!(Job,
            r#"
                INSERT INTO jobs (status, group_id, test_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            "QUEUED", job.group_id, job.test_id,
        )
        .fetch_one(&*pool)
        .await?)
    }

    // finds a job based on its ID
    pub async fn find(id: i32, pool: &PgPool) -> Result<Job> {
        Ok(sqlx::query_as!(Job, "SELECT * FROM jobs WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?)
    }

    // lists all the jobs
    pub async fn list(pool: &PgPool) -> Result<Vec<Job>> {
        Ok(sqlx::query_as!(Job, "SELECT * FROM jobs").fetch_all(&*pool).await?)
    }

    // deletes a job
    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32> {
        sqlx::query!("DELETE FROM jobs WHERE id = $1", id).execute(&*pool).await?;
        Ok(id)
    }
}
