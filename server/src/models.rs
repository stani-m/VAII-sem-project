use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
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
            &self.password_hash.as_ref().expect("Password hash missing!"),
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

    pub async fn fetch_runs(&self, pool: &MySqlPool) -> Result<Vec<Run>, sqlx::Error> {
        let runs = sqlx::query_as!(
            Run,
            r#"SELECT id as "id?", user_id as "user_id?", score, time as "time?" FROM runs WHERE user_id = (SELECT id FROM users WHERE username = ?)"#,
            self.username
        )
            .fetch_all(pool)
            .await?;
        Ok(runs)
    }

    pub async fn fetch_messages(&self, pool: &MySqlPool) -> Result<Vec<Message>, sqlx::Error> {
        let id = self.id(pool).await?;
        let messages = sqlx::query_as!(
            Message,
            "SELECT * FROM messages WHERE from_user_id = ? OR to_user_id = ?",
            id,
            id
        )
            .fetch_all(pool)
            .await?;
        Ok(messages)
    }

    pub async fn id(&self, pool: &MySqlPool) -> Result<u32, sqlx::Error> {
        Ok(
            sqlx::query!("SELECT id FROM users WHERE username = ?", self.username)
                .fetch_one(pool)
                .await?
                .id,
        )
    }

    pub async fn username_from_id(id: u32, pool: &MySqlPool) -> Result<String, sqlx::Error> {
        Ok(sqlx::query!("SELECT username FROM users WHERE id = ?", id)
            .fetch_one(pool)
            .await?
            .username)
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

#[derive(Serialize, Deserialize)]
pub struct Run {
    id: Option<u32>,
    user_id: Option<u32>,
    score: u32,
    time: Option<chrono::NaiveDateTime>,
}

impl Run {
    pub fn generate_time(&mut self) {
        self.time = Some(chrono::Utc::now().naive_utc());
    }

    pub async fn submit_for_user(&self, user: &User, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO runs(user_id, score, time) VALUES((SELECT id FROM users WHERE username = ?), ?, ?)",
            user.username,
            self.score,
            self.time.expect("Time missing!")
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_all(pool: &MySqlPool) -> Result<Vec<Run>, sqlx::Error> {
        let runs = sqlx::query_as!(
            Self,
            r#"SELECT id as "id?", user_id as "user_id?", score, time as "time?" FROM runs"#
        )
        .fetch_all(pool)
        .await?;
        Ok(runs)
    }

    pub async fn get_username(&self, pool: &MySqlPool) -> Result<String, sqlx::Error> {
        Ok(
            sqlx::query!("SELECT username FROM users WHERE id = ?", self.user_id)
                .fetch_one(pool)
                .await?
                .username,
        )
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn time(&self) -> Option<chrono::NaiveDateTime> {
        self.time
    }
}

pub struct Message {
    id: u32,
    from_user_id: u32,
    show_sender: i8,
    to_user_id: u32,
    show_recipient: i8,
    time: chrono::NaiveDateTime,
    text: String,
}

impl Message {
    pub fn new(from_user_id: u32, to_user_id: u32, time: chrono::NaiveDateTime, text: String) -> Message {
        Self {
            id: 0,
            from_user_id,
            show_sender: 1,
            to_user_id,
            show_recipient: 1,
            time,
            text,
        }
    }

    pub async fn save(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO messages(from_user_id, to_user_id, time, text) VALUES(?, ?, ?, ?)",
            self.from_user_id,
            self.to_user_id,
            self.time,
            self.text
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn sender_username(&self, pool: &MySqlPool) -> Result<String, sqlx::Error> {
        User::username_from_id(self.from_user_id, pool).await
    }

    pub async fn recipient_username(&self, pool: &MySqlPool) -> Result<String, sqlx::Error> {
        User::username_from_id(self.to_user_id, pool).await
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn show_sender(&self) -> bool {
        self.show_sender == 1
    }

    pub fn show_recipient(&self) -> bool {
        self.show_recipient == 1
    }

    pub fn time(&self) -> chrono::NaiveDateTime {
        self.time
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
