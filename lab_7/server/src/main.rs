#[macro_use]
extern crate dotenv;
mod db;
use db::*;
use actix_files::{Files,NamedFile};
use std::sync::Mutex;
use actix_web::{get,post, web, App, HttpServer,HttpResponse,HttpRequest, Responder,Result};

#[derive(Debug, Clone)]
struct DatabaseState {
    connection: sea_orm::DatabaseConnection,
}
#[post("/database")]
async fn database_action(data: web::Data<DatabaseState>, request_data: web::Json<shared::SendMessageRequestDatabaseBody>) -> impl Responder {
    match request_data.kind
    {
        shared::DatabaseRequest::AllData => {
            match get_all_data(&data.connection).await
            {
                Ok(data) =>{
                    let mut out_data: Vec<shared::WorkerResponse> = Vec::new();
                    for value in data{
                        out_data.push(shared::WorkerResponse{
                            id: value.id,
                            fname: value.fname,
                            manager: value.manager,
                            salary: value.salary,
                            div_num: value.div_num
                        });
                    };
                    web::Json(shared::SendMessageResponseDatabaseBody{
                        kind: shared::DatabaseRequest::Success,
                        data: out_data,
                        error: String::new()
                    })
                }
                Err(error)=>{
                    web::Json(shared::SendMessageResponseDatabaseBody{
                        kind: shared::DatabaseRequest::Error,
                        data: Vec::new(),
                        error: format!("{}",error)
                    })
                }
            }
        },
        shared::DatabaseRequest::DataById => {
            match request_data.text.parse()
            {
                Ok(value) => {
                    match get_data_by_id(value, &data.connection).await
                    {
                        Ok(data) =>{
                            if data.is_some()
                            {
                                let worker = data.unwrap();
                                let out_data: Vec<shared::WorkerResponse> = vec![shared::WorkerResponse{
                                    id: worker.id,
                                    fname: worker.fname,
                                    manager: worker.manager,
                                    salary: worker.salary,
                                    div_num: worker.div_num
                                }];
                                return web::Json(shared::SendMessageResponseDatabaseBody{
                                    kind: shared::DatabaseRequest::Success,
                                    data: out_data,
                                    error: String::new()
                                })
                            }
                            else
                            {
                                return web::Json(shared::SendMessageResponseDatabaseBody{
                                    kind: shared::DatabaseRequest::Error,
                                    data: Vec::new(),
                                    error: format!("Not found")
                                })
                            }
                        }
                        Err(error)=>{
                            return web::Json(shared::SendMessageResponseDatabaseBody{
                                kind: shared::DatabaseRequest::Error,
                                data: Vec::new(),
                                error: String::new()
                            })
                        }
                    }
                }
                Err(error) => {web::Json(shared::SendMessageResponseDatabaseBody{
                    kind: shared::DatabaseRequest::Error,
                    data: Vec::new(),
                    error: format!("{}",error)
                })}
            }
        },
        shared::DatabaseRequest::DataByName =>{
            match get_data_by_name(request_data.text.clone(),&data.connection).await
            {
                Ok(data) =>{
                    if data.len() != 0
                    {
                        let mut out_data: Vec<shared::WorkerResponse> = Vec::new();
                        for value in data{
                            out_data.push(shared::WorkerResponse{
                                id: value.id,
                                fname: value.fname,
                                manager: value.manager,
                                salary: value.salary,
                                div_num: value.div_num
                            });
                        };
                        return web::Json(shared::SendMessageResponseDatabaseBody{
                            kind: shared::DatabaseRequest::Success,
                            data: out_data,
                            error: String::new()
                        })
                    }
                    else
                    {
                        return web::Json(shared::SendMessageResponseDatabaseBody{
                            kind: shared::DatabaseRequest::Error,
                            data: Vec::new(),
                            error: format!("Not found")
                        })
                    }   
                }
                Err(error)=>{
                    web::Json(shared::SendMessageResponseDatabaseBody{
                        kind: shared::DatabaseRequest::Error,
                        data: Vec::new(),
                        error: format!("{}",error)
                    })
                }
            }
        },
        _ => {
            web::Json(shared::SendMessageResponseDatabaseBody{
                kind: shared::DatabaseRequest::Error,
                data: Vec::new(),
                error: format!("Unknown Error or Bad Request")
            })
        }
    }
    
}
#[allow(clippy::unused_async)]
async fn index() -> Result<NamedFile> {
    println!("Serving at 127.0.0.1:8090");
    Ok(NamedFile::open("./lab_7/client/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {connection: connection};
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(database_action)
            .service(Files::new("/pkg", "./lab_7/client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
