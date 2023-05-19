use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use actix_web::web::{Either, Json, Form};

// <easy-form-handling>
#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

// O formulário de registro é JSON
async fn json_register(form: web::Json<Register>) -> impl Responder {
    format!("Olá {} do país {}!", form.username, form.country)
}

// O formulário de registro pode ser JSON ou codificado em URL.
async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register { username, country } = form.into_inner();
    format!("Olá {username} do país {country}!")
}
// </easy-form-handling>

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
            .route("/json_register", web::post().to(json_register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
