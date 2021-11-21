use yew::prelude::*;
use yew_router::prelude::*;
pub enum Msg {
}
pub struct Messenger{
    name: String,
}
impl Component for Messenger {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            name: String::new()//Generate name for cookies
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = yew_router::prelude::BrowserHistory::default();
        let content = self.name.clone();
            html! {
            <div>
                <button onclick={ctx.link().callback(move |_| history.push(crate::switch::AppRoute::HistoryByTime {name: content.clone()}))}>{ "Go to History" }</button>
                <center>
                    <section class="hero is-danger is-bold is-large">
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    { "Lab 10" }
                                </h1>
                            </div>
                        </div>
                    </section>
                </center>
            </div>
        }
    }
}