use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDateTime;


/* Region for Requests of Client Wants History */
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct RequestHistoryBody {
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHistoryBody {
    pub callback: TypeOfAnswerOnHistory
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMesseges {
    pub name: String,
    pub content: String,
    pub time: NaiveDateTime,   
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum TypeOfAnswerOnHistory
{
    Succes(Vec<ClientMesseges>),
    Error(String)
}


/* Region for Requests of Client Sends Messege */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestClientSendMessege {
    pub text: String,
    pub time: NaiveDateTime
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseClientSendMessege
{
    pub callback: TypeOfAnswerOnMessege,
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum TypeOfAnswerOnMessege
{
    Succes(String),
    Error(String)
}