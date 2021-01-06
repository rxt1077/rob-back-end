use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use anyhow::Result;

#[derive(FromRow)]
struct TestNoJobs {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub memsnapshot: Option<String>,
    pub tests_image: Option<String>,
    pub base_image: String,
    pub command: String,
    pub command_timeout: i32,
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub memsnapshot: Option<String>,
    pub tests_image: Option<String>,
    pub base_image: String,
    pub command: String,
    pub command_timeout: i32,
    pub prompt: String,
    pub jobs: Vec<i32>,
}

// creates a Test struct from a TestNoJobs and vec of job ids
// TODO: mutable reference to clean up clones?
fn add_jobs(test: &TestNoJobs, jobs: Vec<i32>) -> Test {
    Test {
        id: test.id,
        name: test.name.clone(),
        description: test.description.clone(),
        memsnapshot: test.memsnapshot.clone(),
        tests_image: test.tests_image.clone(),
        base_image: test.base_image.clone(),
        command: test.command.clone(),
        command_timeout: test.command_timeout,
        prompt: test.prompt.clone(),
        jobs: jobs,
    }
}

// gets all of the job ids using a particular test
async fn job_ids(test_id: i32, pool: &PgPool) -> Result<Vec<i32>> {
    let rows = sqlx::query!("SELECT id FROM jobs WHERE test_id = $1", test_id)
        .fetch_all(&*pool)
        .await?;
    Ok(rows.iter().map(|row| row.id).collect())
}

impl Test {
    // creates a new Test
    // uses the abs() of command_timeout in case it is negative
    pub async fn create(test: Test, pool: &PgPool) -> Result<Test> {
        let test_no_job = sqlx::query_as!(TestNoJobs,
            r#"
                INSERT INTO tests (name, description, memsnapshot, tests_image,
                    base_image, command, command_timeout, prompt)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING *
            "#,
            test.name, test.description, test.memsnapshot, test.tests_image,
            test.base_image, test.command, test.command_timeout.abs(),
            test.prompt,
            )
           .fetch_one(&*pool)
           .await?;
        Ok(add_jobs(&test_no_job, vec![]))
    }

    // finds a Test based on its ID
    pub async fn find(id: i32, pool: &PgPool) -> Result<Test> {
        let test = sqlx::query_as!(TestNoJobs,
                                   "SELECT * FROM tests WHERE id = $1", id)
           .fetch_one(&*pool)
           .await?;
        Ok(add_jobs(&test, job_ids(id, pool).await?))
    }

    // lists all the Tests
    pub async fn list(pool: &PgPool) -> Result<Vec<Test>> {
        let tests_no_jobs = sqlx::query_as!(TestNoJobs, "SELECT * FROM tests")
            .fetch_all(&*pool).await?;
        let mut tests = Vec::<Test>::new();
        for test_no_job in tests_no_jobs.iter() {
            tests.push(add_jobs(test_no_job,
                job_ids(test_no_job.id, pool).await?));
        }
        Ok(tests)
    }

    // deletes a Test
    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32> {
        sqlx::query!("DELETE FROM tests WHERE id = $1", id).execute(&*pool).await?;
        Ok(id)
    }

    // updates a test (uses abs for command_timeout)
    pub async fn update(test: Test, pool: &PgPool) -> Result<Test> {
        // update the basic group attributes
        sqlx::query!(
            r#"
                UPDATE tests
                SET name = $1, description = $2, memsnapshot = $3,
                    tests_image = $4, base_image = $5, command = $6,
                    command_timeout = $7, prompt = $8
                WHERE id = $9
            "#,
            test.name, test.description, test.memsnapshot, test.tests_image,
            test.base_image, test.command, test.command_timeout, test.prompt,
            test.id
        )
        .execute(&*pool)
        .await?;

        Ok(test)
    }
}
