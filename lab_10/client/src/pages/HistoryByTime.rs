use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub enum Msg {
}
pub struct HistoryByTime{
    name: String
}
impl Component for HistoryByTime {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            name: ctx.props().name.clone()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.name = ctx.props().name.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = yew_router::prelude::BrowserHistory::default();
        html! {
            <div>
                <button onclick={ctx.link().callback(move |_| history.back())}>{ "Go to Back" }</button>
                <center>
                    <section class="hero is-danger is-bold is-large">
                        <div class="hero-body">
                            <div class="container">
                                <h1 class="title">
                                    { format!("Error occured: ") }
                                </h1>
                            </div>
                        </div>
                    </section>
                </center>
            </div>
        }
    }
}