#[macro_use]
extern crate dotenv;
mod db;
use db::*;
use actix_files::{Files,NamedFile};
use actix_session::{CookieSession, Session};
use std::sync::Mutex;
use names::Generator;
use rand::prelude::*;
use actix_web::{get,post, web, App, HttpServer,HttpResponse,HttpRequest, Responder,Result};
/*
    TODO:
    1) Make Database
    2) Make FrontEnd
    2) Logic for Send chat (Maybe with sync)
    3) Logic for history
*/
#[post("/")]
async fn index_post(data: web::Data<DatabaseState>,session: Session, request_data: web::Json<shared::RequestClientSendMessege>) -> impl Responder 
{
        if !request_data.text.is_empty()
        {   
            match db::add_to_history(&data.connection, session.get("id").unwrap().unwrap(), session.get::<String>("name").unwrap().unwrap(), request_data.text.clone(), request_data.time.clone()).await
            {
                Ok(_) => {
                   return web::Json(shared::ResponseClientSendMessege{
                        callback: shared::TypeOfAnswerOnMessege::Succes(format!("{}: {}",session.get::<String>("name").unwrap().unwrap(),request_data.text))
                    });
                },
                Err(msg) => {
                    return web::Json(shared::ResponseClientSendMessege{
                        callback: shared::TypeOfAnswerOnMessege::Error(format!("{}",msg))
                    });
                }
            }
        } 
        else 
        {
            match db::add_to_history(&data.connection, session.get("id").unwrap().unwrap(), session.get::<String>("name").unwrap().unwrap(), String::from("Wrong type of messege"), request_data.time.clone()).await
            {
                Ok(_) => {
                   return web::Json(shared::ResponseClientSendMessege{
                        callback: shared::TypeOfAnswerOnMessege::Succes(format!(""))
                    });
                },
                Err(msg) => {
                    return web::Json(shared::ResponseClientSendMessege{
                        callback: shared::TypeOfAnswerOnMessege::Error(format!("{}",msg))
                    });
                }
            }
        }
}
#[post("/history")]
async fn index_history(data: web::Data<DatabaseState>,session: Session, request_data: web::Json<shared::RequestHistoryBody>) -> impl Responder 
{
    match db::get_history_by_time(&data.connection, request_data.start, request_data.stop).await
    {
        Ok(resul) =>{
            let mut out: Vec<shared::ClientMesseges> = Vec::new();
            for (el,vec) in resul
            {
                for del in vec
                {
                    out.push(shared::ClientMesseges{
                        name: el.client_name.clone(),
                        content: del.content.clone(),
                        time: del.timeof.clone()
                    });
                }
            };
            return web::Json(shared::ResponseHistoryBody{
                callback: shared::TypeOfAnswerOnHistory::Succes(out)
            });
        },
        Err(msg) =>{
            return web::Json(shared::ResponseHistoryBody{
                callback: shared::TypeOfAnswerOnHistory::Error(format!("{}",msg))
            });
        }
    }
}
#[derive(Debug, Clone)]
struct DatabaseState {
    connection: sea_orm::DatabaseConnection,
}

#[allow(clippy::unused_async)]
async fn index(session: Session,data: web::Data<DatabaseState>) -> Result<NamedFile> {
    if let Some(name) = session.get::<String>("name").unwrap()
    {     
    }
    else
    {
        if let Some(id) = session.get::<i32>("id").unwrap()
        {
        }
        else
        {
            let mut generator = Generator::default();
    let mut rng = thread_rng();
    session.insert("name", generator.next().unwrap());
    let mut id: i32 = rng.gen_range(1..=(i32::MAX));
    loop{
        match db::check_id(&data.connection, id).await
        {
            Ok(option) =>
            {
                if !option.is_some()
                {
                    session.insert("id", id);
                    break;
                }
                else
                {
                    id = rng.gen_range(1..=(i32::MAX));
                }
            },
            Err(msg) =>{
                println!("{}",msg);
            }
        }
    }
    
        }
    }
    Ok(NamedFile::open("./lab_10/client/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {connection: connection};
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(CookieSession::signed(&[2;32]).secure(false))
            .service(index_post)
            .service(index_history)
            .service(Files::new("/pkg", "./lab_10/client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8100")?
    .run()
    .await
}
