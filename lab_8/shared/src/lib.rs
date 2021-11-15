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
}
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct SendMessageRequestCalculatorBody {
    pub kind: CalculatorRequest,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponseCalculatorBody {
    pub kind: CalculatorRequest,
    pub data: String,
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
#[derive(Debug, Clone,Serialize, Deserialize)]
pub enum CalculatorRequest {
    Enter = 1,
    Increment = 2 ,
    Dicrement = 3,
    Division = 4,
    Multiply = 5,
    SaveInMemory = 6,
    ClearMemory = 7,
    GetFromMemoryInFirst = 8,
    GetFromMemoryInSecond = 9,
    GetFirst = 10,
    GetSecond = 11,
    Success = 0,
    Error = -1,
}
#[derive(Debug, Clone,Serialize, Deserialize, Default)]
pub struct WorkerResponse {
    pub id: i32,
    pub fname: String,
    pub manager: String,
    pub salary: i32,
    pub div_num: i32,
}