use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn create(
        username: &str,
        password: &str,
        pool: &MySqlPool,
    ) -> Result<Self, sqlx::Error> {
        let query_result = sqlx::query!(
            "INSERT INTO users(username, password) VALUES(?, ?)",
            username,
            password
        )
        .execute(pool)
        .await?;
        if query_result.rows_affected() != 1 {}
        Ok(Self::from_username(username, pool).await?.unwrap())
    }

    pub async fn from_username(
        username: &str,
        pool: &MySqlPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE username = ?", username)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    pub async fn exists(username: &str, pool: &MySqlPool) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(r#"SELECT "x" FROM users WHERE username = ?"#, username)
            .fetch_optional(pool)
            .await?;
        Ok(result.is_some())
    }
}
