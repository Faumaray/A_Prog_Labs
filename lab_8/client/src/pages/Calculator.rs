use yew::prelude::*;  
use web_sys::HtmlTextAreaElement;
use crate::switch::*;



pub struct Calc {
    f_value: String,
    s_value: String,
    result: String,
}
pub enum Msg {
    Increment,
    Dicrement,
    Division,
    Multiply,
    SaveInMemory,
    ClearMemory,
    GetFromMemoryInFirst,
    GetFromMemoryInSecond,
    GetFirst(String),
    GetSecond(String),
    ReceiveResponse(Result<shared::SendMessageResponseCalculatorBody, reqwasm::Error>),
}

pub async fn fetch(body: String) -> Result<shared::SendMessageResponseCalculatorBody, reqwasm::Error> {
    let res: Result<shared::SendMessageResponseCalculatorBody, reqwasm::Error> = reqwasm::http::Request::post("/calculator")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .json()
        .await;
    res
}
impl Component for Calc {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
            kind: shared::CalculatorRequest::Enter,
            text: String::new()
        }).unwrap();
        ctx.link().send_future(async move{
            let data = fetch(body).await;
                Msg::ReceiveResponse(data)
         });
        Self {
            f_value: String::new(),
            s_value: String::new(),
            result: String::new(),
        }
    }

    

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::Increment,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::Dicrement => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::Dicrement,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data= fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            }
            Msg::Multiply => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::Multiply,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::Division => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::Increment,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::GetFirst(inp) => {
                self.f_value = inp.clone();
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::GetFirst,
                    text: self.f_value.clone()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                false
            },
            Msg::GetSecond(inp) => {
                self.s_value = inp.clone();
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::GetSecond,
                    text: self.s_value.clone()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                false
            },
            Msg::SaveInMemory =>{
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::SaveInMemory,
                    text: self.result.clone()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                false
            },
            Msg::ClearMemory => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::ClearMemory,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                false
            },
            Msg::GetFromMemoryInFirst => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInFirst,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::GetFromMemoryInSecond => {
                let body = serde_json::to_string(&shared::SendMessageRequestCalculatorBody{
                    kind: shared::CalculatorRequest::GetFromMemoryInSecond,
                    text: String::new()
                }).unwrap();
                ctx.link().send_future(async move{
                    let data = fetch(body).await;
                        Msg::ReceiveResponse(data)
                 });
                true
            },
            Msg::ReceiveResponse(response) =>{
                match response
                {
                    Ok(data) => {
                        let kind = data.kind;
                        match kind {
                            shared::CalculatorRequest::Success => {
                                self.result = data.data.clone();
                            },
                            shared::CalculatorRequest::GetFromMemoryInFirst => {
                                self.f_value = data.data.clone();
                            },
                            shared::CalculatorRequest::GetFromMemoryInSecond => {
                                self.s_value = data.data.clone();
                            },
                            shared::CalculatorRequest::Enter => {
                                let mut count = 0;
                                for line in data.data.lines()
                                {
                                    if count == 0
                                    {
                                        self.f_value = line.clone().trim().to_string();
                                    }
                                    else if count == 1{
                                        self.s_value = line.clone().trim().to_string();
                                    }
                                    else {
                                        self.result = line.clone().trim().to_string();
                                    }
                                    count += 1;
                                }
                                return true
                            }
                            _ => {
                                return false;
                            }
                        }
                    },
                    Err(error) => {},
                }
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <center>
                <div>
                    <input type="number"
                        value={self.f_value.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::GetFirst(input.value())
                        })}
                    />
                    <input type="number"
                        value={self.s_value.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            Msg::GetSecond(input.value())
                        })}
                    />
                    <label>{format!("Result: {}", self.result.clone())}</label>
                    <br/><br/>
                    <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Dicrement)}>{ "-" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Multiply)}>{ "*" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Division)}>{ "/" }</button>
                    <br/>
                    <button onclick={ctx.link().callback(|_| Msg::SaveInMemory)}>{ "Save" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::ClearMemory)}>{ "Clear" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::GetFromMemoryInFirst)}>{ "1" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::GetFromMemoryInSecond)}>{ "2" }</button>
                </div>
            </center>
        }
    }
}