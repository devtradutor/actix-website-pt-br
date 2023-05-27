use std::io;

use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, r2d2, Insertable, Queryable, SqliteConnection};
use serde::Serialize;

mod schema {
    diesel::table! {
        users {
            id -> VarChar,
            name -> VarChar,
        }
    }
}

#[derive(Debug, Serialize, Queryable)]
struct User {
    id: String,
    name: String,
}

// <handler>
#[derive(Debug, Insertable)]
#[diesel(table_name = self::schema::users)]
struct NewUser<'a> {
    id: &'a str,
    name: &'a str,
}

fn insert_new_user(
    conn: &mut SqliteConnection,
    user_name: String,
) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::*;

    // Criar modelo de inserção
    let uid = format!("{}", uuid::Uuid::new_v4());
    let new_user = NewUser {
        id: &uid,
        name: &user_name,
    };

    // Operações normais do Diesel
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Erro ao inserir pessoa");

    let user = users
        .filter(id.eq(&uid))
        .first::<User>(conn)
        .expect("Erro ao carregar a pessoa que foi inserida agora");

    Ok(user)
}
// </handler>

// <main>
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // conecta ao banco de dados SQLite
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("app.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // Inicia o servidor HTTP na porta 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/{name}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </main>

// <index>
async fn index(
    pool: web::Data<DbPool>,
    name: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (name,) = name.into_inner();

    let user = web::block(move || {
        // Obter uma conexão do pool também é uma operação potencialmente bloqueante.
        // Portanto, deve ser chamada dentro do fechamento `web::block`.
        let mut conn = pool.get().expect("Não foi possível obter uma conexão do banco de dados a partir do pool.");

        insert_new_user(&mut conn, name)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
// </index>
