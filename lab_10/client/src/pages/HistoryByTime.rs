use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlTextAreaElement;
pub enum Msg {
    BackToChat,
    GetHistory,
    GetStart(String),
    GetEnd(String),
    ReceiveResponse(Result<shared::ResponseHistoryBody, reqwasm::Error>),
}
pub struct HistoryByTime{
    start: String,
    end: String,
    content: String
}
pub async fn fetch(body: String) -> Result<shared::ResponseHistoryBody, reqwasm::Error> {
    let res: Result<shared::ResponseHistoryBody, reqwasm::Error> = reqwasm::http::Request::post("/history")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for HistoryByTime {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            start: String::new(),
            end: String::new(),
            content: String::new()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg
        {
            Msg::GetStart(stri) => {
                self.start = stri.clone();
                false
            },
            Msg::GetEnd(stri) => {
                self.end = stri.clone();
                false
            },
            Msg::GetHistory =>{
                if self.start.is_empty() || self.end.is_empty()
                {

                }
                else
                {
                    let body = serde_json::to_string(&shared::RequestHistoryBody{
                        start: chrono::NaiveDateTime::parse_from_str(self.start.clone().as_str(), "%Y-%m-%dT%H:%M").unwrap(),
                        stop: chrono::NaiveDateTime::parse_from_str(self.end.clone().as_str(), "%Y-%m-%dT%H:%M").unwrap()
                    }).unwrap();
                    ctx.link().send_future(async move{
                        let data= fetch(body).await;
                        Msg::ReceiveResponse(data)
                     });
                }
                
                false
            },
            Msg::BackToChat => {
                let history = yew_router::prelude::BrowserHistory::default();
                history.back();
                false
            },
            Msg::ReceiveResponse(response) =>{
                match response
                {
                    Ok(body)=>
                    {
                        match body.callback
                        {
                            shared::TypeOfAnswerOnHistory::Succes(tex) =>{
                                self.content = String::from("");
                                if tex.len() > 0
                                {
                                    for el in tex
                                    {
                                        self.content.push_str(format!("{}||{}: {}\n", el.time,el.name,el.content).as_str());
                                    }
                                    return true;
                                }
                                else
                                {
                                    return false;
                                }
                            },
                            shared::TypeOfAnswerOnHistory::Error(error) =>{
                                let history = yew_router::prelude::BrowserHistory::default();
                                history.push(crate::switch::AppRoute::Error {msg: format!("{}",error).to_string().clone()});
                                return true;
                            }
                        };
                    },
                    Err(error)=>
                    {
                        let history = yew_router::prelude::BrowserHistory::default();
                        history.push(crate::switch::AppRoute::Error {msg: format!("{}",error).to_string().clone()});
                        return true;
                    }
                };
            },
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div>
                <button onclick={ctx.link().callback(move |_| Msg::BackToChat)}>{ "Go to Back" }</button>
                <center>
                <textarea readonly=true rows=30 cols=60 value={self.content.clone()}/>
                <br/>
                <input type="datetime-local"
                        value={self.start.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::GetStart(input.value())
                        })}
                    />
                <input type="datetime-local"
                    value={self.end.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::GetEnd(input.value())
                    })}
                />
                <br/>
                <button onclick={ctx.link().callback(|_| Msg::GetHistory)}>{"Show History"}</button>
                </center>
            </div>
        }
    }
}