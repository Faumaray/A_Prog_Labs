#[macro_use]
extern crate dotenv;
mod db;
use db::*;
use actix_files::{Files,NamedFile};
use actix_session::{CookieSession, Session};
use std::sync::Mutex;
use actix_web::{get,post, web, App, HttpServer,HttpResponse,HttpRequest, Responder,Result};
/*
    TODO:
    1) Make Database
    2) Make FrontEnd
    2) Logic for Send chat (Maybe with sync)
    3) Logic for history
*/

#[derive(Debug, Clone)]
struct DatabaseState {
    connection: sea_orm::DatabaseConnection,
}

#[allow(clippy::unused_async)]
async fn index(session: Session) -> Result<NamedFile> {
    println!("Serving at 127.0.0.1:8100");
    println!("{:?}", session.entries());
    Ok(NamedFile::open("./lab_10/client/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {connection: connection};
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(CookieSession::signed(&[1;32]).secure(false))
            .service(Files::new("/pkg", "./lab_10/client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8100")?
    .run()
    .await
}
