// <setup>
use actix_web::{get, web, App, HttpServer};

// Está estrutura representa um estado
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- obtem o app_name
    format!("Olá {app_name}!") // <- resposta com o  app_name
}
// </setup>

// <start_app>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </start_app>
