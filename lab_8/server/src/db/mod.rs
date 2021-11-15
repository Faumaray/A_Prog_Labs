//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

pub mod prelude;

pub mod workers;
use dotenv::dotenv;
use std::env;
use sea_orm::*;


pub async fn estabilish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Database::connect(database_url).await.unwrap()
}
pub async fn get_all_data(connection: &DatabaseConnection) -> Result<Vec<workers::Model>, DbErr>{
    workers::Entity::find().all(connection).await
}

pub async fn get_data_by_id(select_id: i32, connection:& DatabaseConnection) -> Result<Option<workers::Model>, DbErr>{
    workers::Entity::find_by_id(select_id).one(connection).await
}

pub async fn get_data_by_name(select_name: String, connection: &DatabaseConnection) -> Result<Vec<workers::Model>, DbErr> {
    workers::Entity::find()    
    .filter(workers::Column::Fname.contains(select_name.as_str()))    
    .order_by_asc(workers::Column::Fname)    
    .all(connection)    
    .await
}
