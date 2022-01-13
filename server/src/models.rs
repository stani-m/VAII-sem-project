use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: Option<String>,
    password_hash: Option<String>,
}

impl User {
    pub fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() <= 20
    }

    pub fn is_valid_password(password: &str) -> bool {
        password.len() >= 8 && password.len() <= 50
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_username(&self.username)
            && Self::is_valid_password(self.password.as_ref().expect("Password missing!"))
    }

    pub async fn username_in_use(&self, pool: &MySqlPool) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(r#"SELECT "x" FROM users WHERE username = ?"#, self.username)
            .fetch_optional(pool)
            .await?;
        Ok(result.is_some())
    }

    pub fn generate_hash(&mut self) {
        self.password_hash = Some(
            bcrypt::hash(
                self.password.as_ref().expect("Password missing!"),
                bcrypt::DEFAULT_COST,
            )
            .unwrap(),
        );
    }

    pub async fn load_hash(&mut self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        self.password_hash = Some(
            sqlx::query!(
                "SELECT password_hash FROM users WHERE username = ?",
                self.username
            )
            .fetch_one(pool)
            .await?
            .password_hash,
        );
        Ok(())
    }

    pub fn verify(&self) -> bool {
        bcrypt::verify(
            self.password.as_ref().expect("Password missing!"),
            &self.password_hash.as_ref().expect("Password missing!"),
        )
        .unwrap()
    }

    pub async fn save(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users(username, password_hash) VALUES(?, ?)",
            self.username,
            self.password_hash.as_ref().expect("Password hash missing!")
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn change_username(
        &self,
        new_username: &str,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET username = ? WHERE username = ? AND password_hash = ?",
            new_username,
            self.username,
            self.password_hash.as_ref().expect("Password hash missing!"),
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn change_password(
        &mut self,
        new_password: &str,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        let new_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST).unwrap();
        sqlx::query!(
            "UPDATE users SET password_hash = ? WHERE username = ? AND password_hash = ?",
            new_hash,
            self.username,
            self.password_hash.as_ref().expect("Password hash missing!"),
        )
        .execute(pool)
        .await?;
        self.password_hash = Some(new_hash);
        Ok(())
    }

    pub async fn delete(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users WHERE username = ? AND password_hash = ?",
            self.username,
            self.password_hash.as_ref().expect("Password hash missing!"),
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
