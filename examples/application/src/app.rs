#![allow(dead_code)]

// <setup>
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Olá Mundo!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            //  prefixa todos os recursos e rotas anexados a ele...
            web::scope("/app")
                // ...portanto, isso trata solicitações para `GET /app/index.html`.
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </setup>
