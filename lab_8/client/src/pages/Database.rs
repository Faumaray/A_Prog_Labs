use yew::{       
    prelude::*};
use web_sys::HtmlTextAreaElement;
use yew_router::history::History;

pub enum Msg {
    GetById,
    GetByName,
    GetAll,
    InputName(String),
    InputID(String),
    ReceiveResponse(Result<shared::SendMessageResponseDatabaseBody, reqwasm::Error>)
}
pub struct Db{
    search_id: i32,
    search_name: String,
    data: Vec<Html>
}
pub async fn fetch(body: String) -> Result<shared::SendMessageResponseDatabaseBody, reqwasm::Error> {
    let res: Result<shared::SendMessageResponseDatabaseBody, reqwasm::Error> = reqwasm::http::Request::post("/database")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for Db {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            search_id: -1,
            search_name: String::new(),
            data: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetAll => {
                let body = serde_json::to_string(&shared::SendMessageRequestDatabaseBody{
                    kind: shared::DatabaseRequest::AllData,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                return false
            },
            Msg::GetById => {
                let body = serde_json::to_string(&shared::SendMessageRequestDatabaseBody{
                    kind: shared::DatabaseRequest::DataById,
                    text: self.search_id.clone().to_string()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                return false
            },
            Msg::GetByName => {
                let body = serde_json::to_string(&shared::SendMessageRequestDatabaseBody{
                    kind: shared::DatabaseRequest::DataByName,
                    text: self.search_name.clone().to_string()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                return false
            },
            Msg::InputID(value) => {
                self.search_id = value.parse().unwrap();
                return false
            },
            Msg::InputName(value) => {
                self.search_name = value;
                return false
            },
            Msg::ReceiveResponse(response) => {
                match response
                {
                    Ok(data) => {
                        match data.kind
                        {
                                shared::DatabaseRequest::Success => {
                                    let mut nodes: Vec<Html> = Vec::new();
                                    for value in data.data
                                    {
                                        nodes.push(html!{
                                            <tr>
                                                <td>{value.id.clone()}</td>
                                                <td>{value.fname.clone()}</td>
                                                <td>{value.manager.clone()}</td>
                                                <td>{value.salary.clone()}</td>
                                                <td>{value.div_num.clone()}</td>
                                            </tr>
                                        });
                                    }
                                    self.data = nodes;
                                    return true
                                },
                                _ => {
                                    let history = yew_router::prelude::BrowserHistory::default();
                                    &history.push(crate::switch::AppRoute::Error {msg: format!("Data Recieve Error")});
                                    return false
                                }
                        }
                    },
                    Err(error) => {
                        let history = yew_router::prelude::BrowserHistory::default();
                        &history.push(crate::switch::AppRoute::Error {msg: format!("{}", error)});
                        return false},
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            
                <div>
                    <center>
                    <input type="number"
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::InputID(input.value())
                    })}
                    />
                    <button type="submit" class="submit_search_by_id" onclick={ctx.link().callback(|_| Msg::GetById)}> {"Show by ID"}</button>
                    <br/>
                    <br/>
                    <input type="text"
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::InputName(input.value())
                    })}
                    />
                    <button type="submit" class="submit_search_by_name" onclick={ctx.link().callback(|_| Msg::GetByName)}>{"Show by name"}</button>      
                    <br/>
                    <br/>
                    <button type="submit" class="show_all" onclick={ctx.link().callback(|_| Msg::GetAll)}>{"Show all"}</button>     
                    <hr/>
                    </center>
                    <table id="table" width="100%" border="1px solid black">
                        <thead>
                            <tr>
                                <th>{"ID"}</th>
                                <th>{"Name"}</th>
                                <th>{"Manager"}</th>
                                <th>{"Salary"}</th>
                                <th>{"Division number"}</th>
                            </tr>
                        </thead>
                        { self.data.clone() }
                    </table>
                </div>
            
        }
    }
}