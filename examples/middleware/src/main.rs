pub mod default_headers;
pub mod errorhandler;
pub mod logger;
pub mod user_sessions;
pub mod wrap_fn;

// <simple>
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// Existem duas etapas no processamento de middlewares.
// 1. Inicialização do middleware, a fábrica de middleware é chamada 
// com o próximo serviço na cadeia como parâmetro.
// 2. O método de chamada do middleware é chamado com a requisição normal.
pub struct SayHi;

// A fábrica de Middleware que é uma trait chamada `Transform`
// `S` - O tipo do próximo serviço
// `B` - O tipo do corpo da resposta
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Oi do início. Você solicitou: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Oi da resposta");
            Ok(res)
        })
    }
}
// </simple>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new().wrap(SayHi).service(
            web::resource("/").to(|| async {
                "Olá, middleware! Verifique o console onde o servidor está sendo executado."
            }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
