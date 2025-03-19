use actix_files::Files;
use actix_web::{App, HttpServer, main};
use std::{env, fs, io::Result};

#[main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let port = env::var("PORT")
        .map_or(Ok(8080), |v| v.parse())
        .unwrap_or(8080);

    // Create a directory to serve files from
    let dir = env::current_dir()?.join("share");
    fs::create_dir_all(&dir)?;

    HttpServer::new(move || App::new().service(Files::new("/", &dir).show_files_listing()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
