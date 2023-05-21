// <either>
use actix_web::{Either, Error, HttpResponse};

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn index() -> RegisterResult {
    if is_a_variant() {
        // escolhe a variante Left
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // escolhe a variante Right
        Either::Right(Ok("Hello!"))
    }
}
// </either>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn is_a_variant() -> bool {
    true
}
