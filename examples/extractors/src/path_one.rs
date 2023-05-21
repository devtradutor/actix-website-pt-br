// <path-one>
use actix_web::{get, web, App, HttpServer, Result};

/// Extrair informações de caminho da URL "/users/{user_id}/{friend}".
/// {user_id} - Deserializa para um u32.
/// {friend} - Deserializa para uma String.
#[get("/users/{user_id}/{friend}")] // <- Define os parâmetros do caminho.
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Bem-vindo {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </path-one>
