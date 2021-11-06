use super::schema::workers;

#[derive(Queryable)]
pub struct Worker {
    pub id: i32,
    pub fname: String,
    pub manager: String,
    pub salary: i32,
    pub div_num: i32
}


#[derive(Insertable)]
#[table_name="workers"]
pub struct NewWorker<'a> {
    pub id: &'a i32,
    pub fname: &'a str,
    pub manager: &'a str,
    pub salary: &'a i32,
    pub div_num: &'a i32
}