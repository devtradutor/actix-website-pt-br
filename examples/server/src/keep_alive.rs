use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App, Error,
};

#[allow(dead_code)]
fn app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
}

// <keep-alive>
use actix_web::{http::KeepAlive, HttpServer};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Defina o tempo de keep-alive para 75 segundos.
    let _one = HttpServer::new(app).keep_alive(Duration::from_secs(75));

    // Utilize o keep-alive do sistema operacional (geralmente com um tempo bastante longo).
    let _two = HttpServer::new(app).keep_alive(KeepAlive::Os);

    // Desativar keep-alive
    let _three = HttpServer::new(app).keep_alive(None);

    Ok(())
}
// </keep-alive>
