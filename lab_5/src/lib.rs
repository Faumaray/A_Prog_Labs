#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::rc::Rc;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn get_all_data(connection: Rc<PgConnection>) -> std::result::Result<Vec<models::Worker>, diesel::result::Error>
    {
        schema::workers::table
        .load::<models::Worker>(&*connection)
    }

    pub fn get_data_by_id(connection: Rc<PgConnection>, id_select: i32) -> std::result::Result<Vec<models::Worker>, diesel::result::Error>
    {
        schema::workers::table.filter(schema::workers::id.eq(id_select))
        .limit(1)
        .load::<models::Worker>(&*connection)
    }

    pub fn insert_worker<'a>(connection: Rc<PgConnection>, id: &'a i32, fname: &'a str, manager: &'a str, salary: &'a i32, div_num: &'a i32) -> std::result::Result<models::Worker, diesel::result::Error>
    {

        let new_worker = models::NewWorker{
            id,
            fname,
            manager,
            salary,
            div_num
        };
        diesel::insert_into(schema::workers::table)
        .values(&new_worker)
        .get_result(&*connection)
    }

    pub fn delete_worker(connection: Rc<PgConnection>, id_delete: i32) -> std::result::Result<usize, diesel::result::Error>
    {
        diesel::delete(schema::workers::table.filter(schema::workers::id.eq(id_delete)))
        .execute(&*connection)
    }