use actix_web::{App, HttpServer, main, middleware::Compress, middleware::Logger};
use env_logger::Env;
use std::{env, fs, io::Result};

use droplet::services;

#[main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    dotenvy::dotenv().ok();

    let port = env::var("PORT")
        .map_or(Ok(8080), |v| v.parse())
        .unwrap_or(8080);

    // Create a directory to serve files from
    let dir = env::current_dir()?.join("share");
    fs::create_dir_all(&dir)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(services::serve_dir(&dir))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
