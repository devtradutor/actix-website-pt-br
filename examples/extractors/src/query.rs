// <query>
use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// Este manipulador é chamado se a consulta for desserializada com sucesso em `Info`.
// Caso contrário, uma resposta de erro 400 Bad Request é retornada.
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Bem-vindo {}!", info.username)
}
// </query>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
