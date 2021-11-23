use yew::prelude::*;
use yew_router::prelude::*;
use chrono::NaiveDateTime;
pub enum Msg {
    Send(String),
    ReceiveResponse(Result<shared::ResponseClientSendMessege, reqwasm::Error>),
    MoveToHist
}

pub struct Messenger{
    content: String,
}
pub async fn fetch(body: String) -> Result<shared::ResponseClientSendMessege, reqwasm::Error> {
    let res: Result<shared::ResponseClientSendMessege, reqwasm::Error> = reqwasm::http::Request::post("/")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for Messenger {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            content: String::new()
        }

    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::Send(text) =>{
                let time = chrono::Utc::now().naive_utc();
                let body = serde_json::to_string(&shared::RequestClientSendMessege{
                    text: text.clone(),
                    time: time.clone()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data= fetch(body).await;
                    Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::ReceiveResponse(response) =>{
                match response
                {
                    Ok(body)=>
                    {
                        match body.callback
                        {
                            shared::TypeOfAnswerOnMessege::Succes(messege) =>{
                                if !messege.is_empty()
                                {
                                    self.content.push_str(format!("{}\n",messege.clone()).as_str());
                                    return true;
                                }
                                else
                                {
                                    return false;
                                }
                            },
                            shared::TypeOfAnswerOnMessege::Error(error) =>{
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
            Msg::MoveToHist =>{
                let history = yew_router::prelude::BrowserHistory::default();
                history.push(crate::switch::AppRoute::HistoryByTime);
                return false;
            }
            _ =>{false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Send(value))
            } else {
                None
            }
        });    
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::MoveToHist)}>{ "Go to History" }</button>
                <center>
                    <textarea readonly=true rows=30 cols=60 value={self.content.clone()}/>
                    <br/>
                    <input type="text" {onkeypress}/>
                </center>
            </div>
        }
    }
}