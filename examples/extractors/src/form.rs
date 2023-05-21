// <form>
use actix_web::{post, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extrair dados de formulário usando serde
/// este manipulador é chamado apenas se o tipo de conteúdo for *x-www-form-urlencoded*
/// e o conteúdo da solicitação puder ser desserializado em uma estrutura `FormData`
#[post("/")]
async fn index(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Bem-vindo {}!", form.username))
}
// </form>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
