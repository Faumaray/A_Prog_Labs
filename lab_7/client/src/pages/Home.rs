use yew::prelude::*;
use yew_router::prelude::*;
pub enum Msg {
}
pub struct Home{
}
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <center>
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Lab 8" }
                        </h1>
                    </div>
                </div>
            </section>
            </center>
        }
    }
}