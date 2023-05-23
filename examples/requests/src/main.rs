pub mod manual;
pub mod multipart;
pub mod streaming;
pub mod urlencoded;

// <json-request>
use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// Extraindo Info usando serde
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Bem-vindo {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </json-request>
