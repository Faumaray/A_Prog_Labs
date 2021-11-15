use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct SendMessageRequestDatabaseBody {
    pub kind: DatabaseRequest,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponseDatabaseBody {
    pub kind: DatabaseRequest,
    pub data: Vec<WorkerResponse>,
    pub error: String,
}
#[derive(Debug, Clone,Serialize, Deserialize)]
pub enum DatabaseRequest
{
    AllData = 1,
    DataById = 2,
    DataByName= 3,
    Success = 0,
    Error = -1
}
#[derive(Debug, Clone,Serialize, Deserialize, Default)]
pub struct WorkerResponse {
    pub id: i32,
    pub fname: String,
    pub manager: String,
    pub salary: i32,
    pub div_num: i32,
}