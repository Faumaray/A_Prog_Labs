use yew::prelude::*;  
use web_sys::HtmlTextAreaElement;
use crate::switch::*;



pub struct Calc {
    f_value: Option<String>,
    s_value: Option<String>,
    result: String,
}
pub enum Msg {
    Increment,
    Dicrement,
    Division,
    Multiply,
    GetFirst(String),
    GetSecond(String),
}
impl Component for Calc {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            f_value: None,
            s_value: None,
            result: String::new(),
        }
    }

    

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                if self.f_value.is_some() && self.s_value.is_some()
                {
                    self.result = format!("{}",self.f_value.clone().unwrap().parse::<f32>().unwrap() + self.s_value.clone().unwrap().parse::<f32>().unwrap());
                } 
                else
                {

                }
                true
            },
            Msg::Dicrement => {
                if self.f_value.is_some() && self.s_value.is_some()
                {
                    self.result = format!("{}",self.f_value.clone().unwrap().parse::<f32>().unwrap() - self.s_value.clone().unwrap().parse::<f32>().unwrap());
                } 
                else
                {

                }
                true
            }
            Msg::Multiply => {
                if self.f_value.is_some() && self.s_value.is_some()
                {
                    self.result = format!("{}",self.f_value.clone().unwrap().parse::<f32>().unwrap() * self.s_value.clone().unwrap().parse::<f32>().unwrap());
                } 
                else
                {

                }
                true
            },
            Msg::Division => {
                if self.f_value.is_some() && self.s_value.is_some()
                {
                    self.result = format!("{}",self.f_value.clone().unwrap().parse::<f32>().unwrap() / self.s_value.clone().unwrap().parse::<f32>().unwrap());
                } 
                else
                {

                }
                true
            },
            Msg::GetFirst(inp) => {
                match inp.clone().parse::<f32>()
                {
                    Ok(data) => {
                        self.f_value = Some(format!("{}",data));
                    },
                    Err(error) => {}
                }       
                false
            },
            Msg::GetSecond(inp) => {
                match inp.clone().parse::<f32>()
                {
                    Ok(data) => {
                        self.s_value = Some(format!("{}",data));
                    },
                    Err(error) => {}
                }       
                false
            },
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
                    <label>{self.result.clone()}</label>
                    <br/><br/>
                    <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Dicrement)}>{ "-" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Multiply)}>{ "*" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Division)}>{ "/" }</button>
                </div>
            </center>
        }
    }
}