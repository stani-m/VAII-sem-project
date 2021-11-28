use crate::models::User;
use serde::Serialize;
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
    let User { username, password } = request.body_json().await?;
    let pool = request.state();
    if User::exists(&username, pool).await? {
        Ok(Response::error("Username is already in use.").into())
    } else if username.is_empty()
        || username.len() > 20
        || password.len() < 8
        || password.len() > 50
    {
        Ok(Response::error("Invalid request.").into())
    } else {
        User::create(&username, &password, pool).await?;
        Ok(Response::ok(()).into())
    }
}

pub async fn log_in(mut request: tide::Request<MySqlPool>) -> tide::Result {
    let error_message = "Incorrect username or password.";
    let login_data: User = request.body_json().await?;
    if let Some(user) = User::from_username(&login_data.username, request.state()).await? {
        if login_data.password == user.password {
            Ok(Response::ok(()).into())
        } else {
            Ok(Response::error(error_message).into())
        }
    } else {
        Ok(Response::error(error_message).into())
    }
}
