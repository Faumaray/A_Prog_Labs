use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub msg: String,
}

pub enum Msg {
}
pub struct Er{
    content: String
}
impl Component for Er {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            content: ctx.props().msg.clone()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.content = ctx.props().msg.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = yew_router::prelude::BrowserHistory::default();
        html! {
            <center>
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { format!("Error occured: ") }
                        </h1>
                    </div>
                </div>
                <button onclick={ctx.link().callback(move |_| history.back())}>{ "Go back" }</button>
            </section>
            </center>
        }
    }
}