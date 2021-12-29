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
            password,
        )
        .execute(pool)
        .await?;
        if query_result.rows_affected() != 1 {
            panic!("Something very bad has gone very wrong!");
        }
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

    pub fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() <= 20
    }

    pub fn is_valid_password(password: &str) -> bool {
        password.len() >= 8 && password.len() <= 50
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_username(&self.username) && Self::is_valid_password(&self.password)
    }

    pub async fn username_in_use(&self, pool: &MySqlPool) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(r#"SELECT "x" FROM users WHERE username = ?"#, self.username)
            .fetch_optional(pool)
            .await?;
        Ok(result.is_some())
    }

    pub async fn has_correct_password(&self, pool: &MySqlPool) -> Result<bool, sqlx::Error> {
        if let Some(user) = User::from_username(&self.username, pool).await? {
            Ok(self.password == user.password)
        } else {
            Ok(false)
        }
    }

    pub async fn save(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        Self::create(&self.username, &self.password, pool).await?;
        Ok(())
    }

    pub async fn change_username(
        &self,
        new_username: &str,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET username = ? WHERE username = ? AND password = ?",
            new_username,
            self.username,
            self.password,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn change_password(
        &self,
        new_password: &str,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET password = ? WHERE username = ? AND password = ?",
            new_password,
            self.username,
            self.password,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users WHERE username = ? AND password = ?",
            self.username,
            self.password,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
