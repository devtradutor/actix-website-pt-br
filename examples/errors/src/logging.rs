// <logging>
use actix_web::{error, get, middleware::Logger, App, HttpServer, Result};
use derive_more::{Display, Error};
use log::info;

#[derive(Debug, Display, Error)]
#[display(fmt = "meu erro: {}", name)]
pub struct MyError {
    name: &'static str,
}

// Utilize a implementação padrão para o método `error_response()`.
impl error::ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    let err = MyError { name: "teste o erro" };
    info!("{}", err);
    Err(err)
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </logging>
