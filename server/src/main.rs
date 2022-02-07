use std::env;

use sqlx::mysql::MySqlPoolOptions;

mod endpoints;
mod models;

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
    app.at("/change-username").put(endpoints::change_username);
    app.at("/change-password").put(endpoints::change_password);
    app.at("/delete-account").delete(endpoints::delete_account);
    app.at("/submit-run").post(endpoints::submit_run);
    app.at("/get-runs").post(endpoints::get_runs);
    app.at("/message").post(endpoints::send_message);
    app.at("/get-messages").post(endpoints::get_messages);
    app.at("/block").post(endpoints::block);

    app.at("/").serve_dir("../www")?;

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
