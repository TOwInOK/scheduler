use sqlx::{Pool, Sqlite, query, query_as};

use crate::{cells::cell::groups::Groups, error::Result};

pub struct User {
    pub id: i64,
    pub selected_group: Groups,
}

impl User {
    pub fn new(id: i64, selected_group: Groups) -> Self {
        Self { id, selected_group }
    }
    pub async fn get_user_by_id(pool: Pool<Sqlite>, id: i64) -> Result<Self> {
        Ok(query_as!(
            Self,
            "SELECT id, selected_group FROM users WHERE id = ?",
            id
        )
        .fetch_one(&pool)
        .await?)
    }
    pub async fn update_selected_group(&self, pool: Pool<Sqlite>, group: Groups) -> Result<()> {
        let group: i64 = group.into();
        query!(
            "UPDATE users SET selected_group = ? WHERE id = ?",
            group,
            self.id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
    pub async fn delete_user(&self, pool: Pool<Sqlite>) -> Result<()> {
        query!("DELETE FROM users WHERE id = ?", self.id)
            .execute(&pool)
            .await?;
        Ok(())
    }
    pub async fn create_user(&self, pool: Pool<Sqlite>) -> Result<()> {
        let group: i64 = self.selected_group.into();
        query!(
            "INSERT INTO users (id, selected_group) VALUES (?, ?)",
            self.id,
            group
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
}
