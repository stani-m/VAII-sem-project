#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let mut app = tide::new();
    app.at("/").serve_file("../www/index.html")?;
    app.at("/").serve_dir("../www")?;
    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
