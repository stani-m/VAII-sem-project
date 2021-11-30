mod endpoints;
mod models;

use sqlx::mysql::MySqlPoolOptions;
use std::env;

// docker run --name mariadbtest -e MYSQL_ROOT_PASSWORD=mypass -p 3306:3306 -d docker.io/library/mariadb:10.7
#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Warn);

    let pool = MySqlPoolOptions::new()
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let mut app = tide::with_state(pool.clone());

    app.at("/").serve_file("../www/index.html")?;
    app.at("/sign-up").post(endpoints::sign_up);
    app.at("/log-in").post(endpoints::log_in);
    app.at("/delete-account").post(endpoints::delete_account);
    app.at("/change-username").post(endpoints::change_username);
    app.at("/change-password").post(endpoints::change_password);

    app.at("/").serve_dir("../www")?;
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
