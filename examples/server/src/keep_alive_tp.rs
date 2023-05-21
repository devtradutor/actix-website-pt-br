#![allow(dead_code)]

// <example>
use actix_web::{http, HttpRequest, HttpResponse};

async fn index(_req: HttpRequest) -> HttpResponse {
    let mut resp = HttpResponse::Ok()
        .force_close() // <- Encerre a conexão na HttpResponseBuilder.
        .finish();

    // Alternativamente, encerre a conexão na struct HttpResponse.
    resp.head_mut().set_connection_type(http::ConnectionType::Close);

    resp
}
// </example>

// ConnectionType::Close
// ConnectionType::KeepAlive
// ConnectionType::Upgrade
