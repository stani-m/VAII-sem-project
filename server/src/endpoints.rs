use crate::models::User;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Serialize)]
struct Response<T: Serialize> {
    code: i32,
    body: T,
}

impl<T: Serialize> Response<T> {
    fn ok(body: T) -> Self {
        Self { code: 0, body }
    }

    fn error(body: T) -> Self {
        Self { code: 1, body }
    }

    fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<T: Serialize> From<Response<T>> for tide::Response {
    fn from(response: Response<T>) -> Self {
        tide::Response::builder(200)
            .body(response.json())
            .content_type(tide::http::mime::JSON)
            .build()
    }
}

pub async fn sign_up(mut request: tide::Request<MySqlPool>) -> tide::Result {
    let user: User = request.body_json().await?;
    let pool = request.state();
    if user.username_in_use(pool).await? {
        Ok(Response::error("Username is already in use.").into())
    } else if !user.is_valid() {
        Ok(Response::error("Invalid request.").into())
    } else {
        user.save(pool).await?;
        Ok(Response::ok(()).into())
    }
}

pub async fn log_in(mut request: tide::Request<MySqlPool>) -> tide::Result {
    let user: User = request.body_json().await?;
    if user.has_correct_password(request.state()).await? {
        Ok(Response::ok(()).into())
    } else {
        Ok(Response::error("Incorrect username or password.").into())
    }
}

pub async fn change_username(mut request: tide::Request<MySqlPool>) -> tide::Result {
    #[derive(Deserialize)]
    struct Data {
        #[serde(rename = "newUsername")]
        new_username: String,
        user: User,
    }

    let Data { new_username, user } = request.body_json().await?;
    let pool = request.state();

    if user.has_correct_password(pool).await? {
        user.change_username(&new_username, pool).await?;
        Ok(Response::ok(()).into())
    } else {
        Ok(Response::error("Incorrect password.").into())
    }
}

pub async fn change_password(mut request: tide::Request<MySqlPool>) -> tide::Result {
    #[derive(Deserialize)]
    struct Data {
        #[serde(rename = "newPassword")]
        new_password: String,
        user: User,
    }

    let Data { new_password, user } = request.body_json().await?;
    let pool = request.state();

    if user.has_correct_password(pool).await? {
        user.change_password(&new_password, pool).await?;
        Ok(Response::ok(()).into())
    } else {
        Ok(Response::error("Incorrect password.").into())
    }
}

pub async fn delete_account(mut request: tide::Request<MySqlPool>) -> tide::Result {
    let user: User = request.body_json().await?;
    let pool = request.state();
    if user.has_correct_password(pool).await? {
        user.delete(pool).await?;
        Ok(Response::ok(()).into())
    } else {
        Ok(Response::error("Incorrect password.").into())
    }
}
