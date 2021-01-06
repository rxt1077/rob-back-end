use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use anyhow::Result;
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub git_url: String,
    pub members: HashSet<String>,
}

// returns a HashSet<String> of ucids that are members of a group. For internal use.
async fn get_members(id: i32, pool: &PgPool) -> Result<HashSet<String>> {
    let members = sqlx::query!("SELECT ucid FROM group_membership WHERE id = $1", id)
        .fetch_all(&*pool)
        .await?;
    Ok(members.iter().map(|row| row.ucid.clone()).collect())
}

// adds all members to the membership of a group. For internal use.
async fn add_members(id: i32, members: &HashSet<String>, pool: &PgPool) -> Result<HashSet<String>> {
    for ucid in members.iter() {
        sqlx::query!("INSERT INTO group_membership (id, ucid) VALUES ($1, $2)", id, ucid)
            .execute(&*pool).await?;
    }
    Ok(members.clone())
}

// deletes all members of a group. For internal use.
async fn delete_members(id: i32, pool: &PgPool) -> Result<i32> {
    sqlx::query!("DELETE FROM group_membership WHERE id = $1", id)
        .execute(&*pool).await?;
    Ok(id)
}

impl Group {
    // creates a new group (including members)
    pub async fn create(group: Group, pool: &PgPool) -> Result<Group> {
        // add group
        let group_row = sqlx::query!(
            r#"
                INSERT INTO groups (name, git_url)
                VALUES ($1, $2)
                RETURNING *
            "#,
            group.name,
            group.git_url,
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Group {
            id: group_row.id,
            name: group_row.name,
            git_url: group_row.git_url,
            members: add_members(group_row.id, &group.members, pool).await?,
        })
    }

    // finds a group based on it's ID
    pub async fn find(id: i32, pool: &PgPool) -> Result<Group> {
        let group_row = sqlx::query!("SELECT * FROM groups WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;

        Ok(Group {
            id: group_row.id,
            name: group_row.name,
            git_url: group_row.git_url,
            members: get_members(id, pool).await?,
        })
    }

    // lists all the groups (including their memberships)
    pub async fn list(pool: &PgPool) -> Result<Vec<Group>> {
        let group_rows = sqlx::query!("SELECT * FROM groups").fetch_all(&*pool).await?;

        let mut groups = Vec::<Group>::new();
        for row in group_rows.iter() {
            groups.push(Group {
                id: row.id,
                name: row.name.clone(),
                git_url: row.git_url.clone(),
                members: get_members(row.id, pool).await?,
            })
        }
        Ok(groups)
    }

    // updates a group (and it's memberships)
    pub async fn update(group: Group, pool: &PgPool) -> Result<Group> {
        // update the basic group attributes
        sqlx::query!(
            r#"
                UPDATE groups
                SET name = $1, git_url = $2
                WHERE id = $3
            "#,
            group.name, group.git_url, group.id
        )
        .execute(&*pool)
        .await?;

        // update group memberships
        delete_members(group.id, pool).await?;
        add_members(group.id, &group.members, pool).await?;

        Ok(group)
    }

    // deletes a group (and it's memberships)
    pub async fn delete(id: i32, pool: &PgPool) -> Result<i32> {
        sqlx::query!("DELETE FROM groups WHERE id = $1", id)
            .execute(&*pool).await?;
        delete_members(id, pool).await?;

        Ok(id)
    }
}
