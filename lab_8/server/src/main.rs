#[macro_use]
extern crate dotenv;
mod db;
use db::*;
use actix_files::{Files,NamedFile};
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
async fn calculation(data: web::Data<CalculatorState>, request_data: web::Json<shared::SendMessageRequestCalculatorBody>) -> impl Responder 
{
    match request_data.kind 
    {
       
        shared::CalculatorRequest::Enter =>{
            let first = data.f_value.lock().unwrap();
            let second = data.s_value.lock().unwrap();
            let result = data.result.lock().unwrap();
            let mut f_res = String::new();
            let mut s_res = String::new();
            let mut res_res = String::new();
            if first.is_some()
            {
                f_res = format!("{}",first.clone().unwrap());
            }
            if first.is_some()
            {
                s_res = format!("{}",second.clone().unwrap());
            }
            if first.is_some()
            {
                res_res = format!("{}",result.clone().unwrap());
            }
            return web::Json(shared::SendMessageResponseCalculatorBody{
                kind: shared::CalculatorRequest::Enter,
                data: format!("{}\n{}\n{}",f_res,s_res,res_res)
            })
        },
        shared::CalculatorRequest::GetFirst =>{
            let mut counter = data.f_value.lock().unwrap();
            match request_data.text.parse::<f32>()
            {
                Ok(value) => {
                    *counter = Some(value);
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
                let mut counter = data.s_value.lock().unwrap();
                match request_data.text.parse::<f32>()
                {
                    Ok(value) => {
                        *counter = Some(value);
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
            let mut counter = data.memory.lock().unwrap();
                match request_data.text.parse::<f32>()
                {
                    Ok(value) => {
                        *counter = Some(value);
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
            let counter = data.memory.lock().unwrap();
            if counter.is_some()
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInFirst,
                    data: format!("{}", counter.clone().unwrap())
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
            let counter = data.memory.lock().unwrap();
            if counter.is_some()
            {
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInSecond,
                    data: format!("{}", counter.clone().unwrap())
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
            let mut counter = data.memory.lock().unwrap();
            *counter = None;
            web::Json(shared::SendMessageResponseCalculatorBody{
                 kind: shared::CalculatorRequest::ClearMemory,
                data: String::new()
            })
        },
        shared::CalculatorRequest::Increment => {
            let first = data.f_value.lock().unwrap();
            let second = data.s_value.lock().unwrap();
            let mut result = data.result.lock().unwrap();
            if first.is_some() && second.is_some()
            {
                *result = Some(first.clone().unwrap() + second.clone().unwrap());
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Success,
                   data: format!("{}",result.clone().unwrap())
               })
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
            let first = data.f_value.lock().unwrap();
            let second = data.s_value.lock().unwrap();
            let mut result = data.result.lock().unwrap();
            if first.is_some() && second.is_some()
            {
                *result = Some(first.clone().unwrap() - second.clone().unwrap());
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Success,
                   data: format!("{}",result.clone().unwrap())
               })
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
            let first = data.f_value.lock().unwrap();
            let second = data.s_value.lock().unwrap();
            let mut result = data.result.lock().unwrap();
            if first.is_some() && second.is_some()
            {
                *result = Some(first.clone().unwrap() * second.clone().unwrap());
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Success,
                   data: format!("{}",result.clone().unwrap())
               })
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
            let first = data.f_value.lock().unwrap();
            let second = data.s_value.lock().unwrap();
            let mut result = data.result.lock().unwrap();
            if first.is_some() && second.is_some()
            {
                *result = Some(first.clone().unwrap() / second.clone().unwrap());
                web::Json(shared::SendMessageResponseCalculatorBody{
                    kind: shared::CalculatorRequest::Success,
                   data: format!("{}",result.clone().unwrap())
               })
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
                        data: out_data
                    })
                }
                Err(error)=>{
                    web::Json(shared::SendMessageResponseDatabaseBody{
                        kind: shared::DatabaseRequest::Error,
                        data: Vec::new()
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
                                data: out_data
                            })
                        }
                        Err(error)=>{
                            return web::Json(shared::SendMessageResponseDatabaseBody{
                                kind: shared::DatabaseRequest::Error,
                                data: Vec::new()
                            })
                        }
                    }
                }
                Err(error) => {web::Json(shared::SendMessageResponseDatabaseBody{
                    kind: shared::DatabaseRequest::Error,
                    data: Vec::new()
                })}
            }
        },
        shared::DatabaseRequest::DataByName =>{
            match get_data_by_name(request_data.text.clone(),&data.connection).await
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
                        data: out_data
                    })
                }
                Err(error)=>{
                    web::Json(shared::SendMessageResponseDatabaseBody{
                        kind: shared::DatabaseRequest::Error,
                        data: Vec::new()
                    })
                }
            }
        },
        _ => {
            web::Json(shared::SendMessageResponseDatabaseBody{
                kind: shared::DatabaseRequest::Error,
                data: Vec::new()
            })
        }
    }
    
}
#[allow(clippy::unused_async)]
async fn index() -> Result<NamedFile> {
    println!("Serving at 127.0.0.1:8090");
    Ok(NamedFile::open("./lab_8/client/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = estabilish_connection().await;
    let state = DatabaseState {connection: connection};
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new( CalculatorState {
                f_value: Mutex::new(None),
                s_value: Mutex::new(None),
                result: Mutex::new(None),
                memory: Mutex::new(None)
            }))
            .data(state.clone())
            .service(database_action)
            .service(calculation)
            .service(Files::new("/pkg", "./lab_8/client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
