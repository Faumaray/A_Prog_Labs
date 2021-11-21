#[macro_use]
extern crate dotenv;
mod db;
use db::*;
use actix_files::{Files,NamedFile};
use actix_session::{CookieSession, Session};
use std::sync::Mutex;
use actix_web::{get,post, web, App, HttpServer,HttpResponse,HttpRequest, Responder,Result};

// This struct represents state
struct CalculatorState {
    f_value: Mutex<Option<f32>>,
    s_value: Mutex<Option<f32>>,
    result: Mutex<Option<f32>>,
    memory: Mutex<Option<f32>>
}

#[derive(Debug, Clone)]
struct DatabaseState {
    connection: sea_orm::DatabaseConnection,
}
#[post("/calculator")]
async fn calculation(session: Session, request_data: web::Json<shared::SendMessageRequestCalculatorBody>) -> impl Responder 
{
    match request_data.kind 
    {
        shared::CalculatorRequest::Enter =>{
            let mut f_res = String::new();
            let mut s_res = String::new();
            let mut res_res = String::new();
            if let Some(first) = session.get::<f32>("f_value").unwrap()
            {
                f_res = format!("{}",first.clone());
            }
            if let Some(second) = session.get::<f32>("s_value").unwrap()
            {
                s_res = format!("{}",second.clone());
            }
            if let Some(result) = session.get::<f32>("result").unwrap()
            {
                res_res = format!("{}",result.clone());
            }
            return web::Json(shared::SendMessageResponseCalculatorBody{
                kind: shared::CalculatorRequest::Enter,
                data: format!("{}\n{}\n{}",f_res,s_res,res_res)
            })
        },
        shared::CalculatorRequest::GetFirst =>{
            match request_data.text.parse::<f32>()
            {
                Ok(value) => {
                    session.insert("f_value", value).unwrap();
                    return web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::GetFirst,
                        data: String::new()
                    })
                },
                Err(error) => {
                    return web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Error,
                        data: format!("{}", error)
                })
                },
            }
        },
        shared::CalculatorRequest::GetSecond =>{
                match request_data.text.parse::<f32>()
                {
                    Ok(value) => {
                        session.insert("s_value", value).unwrap();
                        return web::Json(shared::SendMessageResponseCalculatorBody{
                            kind: shared::CalculatorRequest::GetSecond,
                            data: String::new()
                        })
                    },
                    Err(error) => {
                        return web::Json(shared::SendMessageResponseCalculatorBody{
                            kind: shared::CalculatorRequest::Error,
                            data: format!("{}", error)})
                    },
                }
                
        },
        shared::CalculatorRequest::SaveInMemory =>{
                match request_data.text.parse::<f32>()
                {
                    Ok(value) => {
                        session.insert("memory", value).unwrap();
                        return web::Json(shared::SendMessageResponseCalculatorBody{
                            kind: shared::CalculatorRequest::SaveInMemory,
                            data: String::new()
                        })
                    },
                    Err(error) => {
                        return web::Json(shared::SendMessageResponseCalculatorBody{
                            kind: shared::CalculatorRequest::Error,
                            data: format!("{}", error)})
                    },
                }
        },
        shared::CalculatorRequest::GetFromMemoryInFirst =>{
            if let Some(counter) = session.get::<f32>("memory").unwrap()
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInFirst,
                    data: format!("{}", counter.clone())
                })
            }
            else {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                    data: format!("Memory is empty")
                })
            }
        },
        shared::CalculatorRequest::GetFromMemoryInSecond =>{
            if let Some(counter) = session.get::<f32>("memory").unwrap()
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInSecond,
                    data: format!("{}", counter.clone())
                })
            }
            else {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                    data: format!("Memory is empty")
                })
            }
        },
        shared::CalculatorRequest::ClearMemory =>{
            session.remove("memory");
            web::Json(shared::SendMessageResponseCalculatorBody{
                 kind: shared::CalculatorRequest::ClearMemory,
                data: String::new()
            })
        },
        shared::CalculatorRequest::Increment => {
            let first = session.get::<f32>("f_value");
            let second = session.get::<f32>("s_value");
            if first.is_ok() && second.is_ok()
            {
                let first = first.unwrap();
                let second = second.unwrap();
                if first.is_some() && second.is_some()
                {
                    session.insert("result", first.unwrap()+second.unwrap());
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Success,
                    data: format!("{}",session.get::<f32>("result").unwrap().unwrap())
                })
                }
                else
                {
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Error,
                       data: String::new()
                   })
                }
            }
            else
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                   data: String::new()
               })
            }
        },
        shared::CalculatorRequest::Dicrement =>{
            let first = session.get::<f32>("f_value");
            let second = session.get::<f32>("s_value");
            if first.is_ok() && second.is_ok()
            {
                let first = first.unwrap();
                let second = second.unwrap();
                if first.is_some() && second.is_some()
                {
                    session.insert("result", first.unwrap()-second.unwrap());
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Success,
                    data: format!("{}",session.get::<f32>("result").unwrap().unwrap())
                })
                }
                else
                {
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Error,
                       data: String::new()
                   })
                }
            }
            else
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                   data: String::new()
               })
            }
        },
        shared::CalculatorRequest::Multiply =>{
            let first = session.get::<f32>("f_value");
            let second = session.get::<f32>("s_value");
            if first.is_ok() && second.is_ok()
            {
                let first = first.unwrap();
                let second = second.unwrap();
                if first.is_some() && second.is_some()
                {
                    session.insert("result", first.unwrap()*second.unwrap());
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Success,
                    data: format!("{}",session.get::<f32>("result").unwrap().unwrap())
                })
                }
                else
                {
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Error,
                       data: String::new()
                   })
                }
            }
            else
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                   data: String::new()
               })
            }
        },
        shared::CalculatorRequest::Division =>{
            let first = session.get::<f32>("f_value");
            let second = session.get::<f32>("s_value");
            if first.is_ok() && second.is_ok()
            {
                let first = first.unwrap();
                let second = second.unwrap();
                if first.is_some() && second.is_some()
                {
                    session.insert("result", first.unwrap()/second.unwrap());
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Success,
                    data: format!("{}",session.get::<f32>("result").unwrap().unwrap())
                })
                }
                else
                {
                    web::Json(shared::SendMessageResponseCalculatorBody{
                        kind: shared::CalculatorRequest::Error,
                       data: String::new()
                   })
                }
            }
            else
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Error,
                   data: String::new()
               })
            }
        },
        _ =>{
            return web::Json(shared::SendMessageResponseCalculatorBody{
                kind: shared::CalculatorRequest::Error,
                data: format!("Bad request")
            })
        }
    }
    
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
async fn index(session: Session) -> Result<NamedFile> {
    println!("Serving at 127.0.0.1:8090");
    println!("{:?}",session.entries());
    Ok(NamedFile::open("./lab_9/client/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {connection: connection};
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(CookieSession::signed(&[0;32]).secure(false))
            .service(database_action)
            .service(calculation)
            .service(Files::new("/pkg", "./lab_9/client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
