use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use anyhow::Result;

/* this is used by an instructor to update a user
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub group_num: i32,
    pub instructor: bool
}*/

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub ucid: String,
    pub instructor: bool,
    pub google_id: String,
    pub google_email: String,
    pub google_verified_email: bool,
    pub google_name: String,
    pub google_given_name: String,
    pub google_family_name: String,
    pub google_picture: String,
    pub google_locale: String,
    pub google_hd: String,
}

impl User {
    pub async fn find(ucid: String, pool: &PgPool) -> Result<User> {
        let user = sqlx::query_as!(User,
            r#"
                SELECT * FROM users WHERE ucid = $1
            "#,
            ucid
        )
        .fetch_one(&*pool)
        .await?;

        Ok(user)
    }

    // lists all the users
    pub async fn list(pool: &PgPool) -> Result<Vec<User>> {
        let users_rows = sqlx::query!("SELECT * FROM users").fetch_all(&*pool).await?;
        let mut users = Vec::<User>::new();
        for row in users_rows.iter() {
            users.push(User {
                ucid: row.ucid.clone(),
                instructor: row.instructor,
                google_id: row.google_id.clone(), 
                google_email: row.google_email.clone(),
                google_verified_email: row.google_verified_email,
                google_name: row.google_name.clone(),
                google_given_name: row.google_given_name.clone(),
                google_family_name: row.google_family_name.clone(),
                google_picture: row.google_picture.clone(),
                google_locale: row.google_locale.clone(),
                google_hd: row.google_hd.clone(),
            })
        }

        Ok(users)
    }

/*    pub async fn update(ucid: String, user: UserRequest, pool: &PgPool) -> Result<User> {
        let user = sqlx::query_as!(User,
            r#"
                UPDATE users
                SET group_num = $1, instructor = $2
                WHERE ucid = $3
                RETURNING *
            "#,
            user.group_num, user.instructor, ucid
        )
        .fetch_one(&*pool)
        .await?;

        Ok(user)
    } */

    pub async fn update(user: User, pool: &PgPool) -> Result<User> {
        let user = sqlx::query_as!(User,
            r#"
                UPDATE users
                SET
                    instructor = $2,
                    google_id = $3,
                    google_email = $4,
                    google_verified_email = $5,
                    google_name = $6,
                    google_given_name = $7,
                    google_family_name = $8,
                    google_picture = $9,
                    google_locale = $10,
                    google_hd = $11
                WHERE ucid = $1
                RETURNING *
            "#,
            user.ucid,
            user.instructor,
            user.google_id,
            user.google_email,
            user.google_verified_email,
            user.google_name,
            user.google_given_name,
            user.google_family_name,
            user.google_picture,
            user.google_locale,
            user.google_hd
        )
        .fetch_one(&*pool)
        .await?;

        Ok(user)
    }

    pub async fn create(user: User, pool: &PgPool) -> Result<User> {
        sqlx::query!(
            r#"
                INSERT INTO users (
                    ucid,
                    instructor,
                    google_id,
                    google_email,
                    google_verified_email,
                    google_name,
                    google_given_name,
                    google_family_name,
                    google_picture,
                    google_locale,
                    google_hd
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
            "#,
            user.ucid,
            user.instructor,
            user.google_id,
            user.google_email,
            user.google_verified_email,
            user.google_name,
            user.google_given_name,
            user.google_family_name,
            user.google_picture,
            user.google_locale,
            user.google_hd
        )
        .execute(&*pool)
        .await?;

        Ok(user)
    }
}
