use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{HistoryByTime::HistoryByTime,Messenger::Messenger, Error::Er, page_not_found::PageNotFound};
use crate::switch::{AppRoute};

pub struct Index {
}
    fn switch(routes: &AppRoute) -> Html {
        match routes {
            AppRoute::Messenger => {
                html! { <Messenger />}
            },
            AppRoute::Error { msg }  => {
                html! { <Er msg={ msg.clone() }/> }
            },
            AppRoute::HistoryByTime => {
                html! { <HistoryByTime /> }
            },
            AppRoute::PageNotFound => {
                html! { <PageNotFound/> }
            }
        }
    }
impl Component for Index {
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
            <BrowserRouter> 
                <main>
                <Switch<AppRoute> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}