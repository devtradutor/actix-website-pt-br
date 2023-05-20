// <setup_mutable>
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex é necessário para realizar mutações com segurança entre threads.
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- obter MutexGuard do contador
    *counter += 1; // <- acessa o counter dentro do  MutexGuard

    format!("Número da solicitação: {counter}") // <- resposta com counter
}
// </setup_mutable>

// <make_app_mutable>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Observação: web::Data criado _fora_ do fechamento HttpServer::new
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        //mova o counter para o fechamento
        App::new()
            .app_data(counter.clone()) // <- registre os dados criados
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </make_app_mutable>
