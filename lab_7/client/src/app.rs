use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{Database::Db,Calculator::Calc, Error::Er, page_not_found::PageNotFound, Home::Home};
use crate::switch::{AppRoute};

pub struct Index {
}
    fn switch(routes: &AppRoute) -> Html {
        match routes {
            AppRoute::Calculator => {
                html! { <Calc />}
            },
            AppRoute::Error  => {
                html! { <Er /> }
            },
            AppRoute::Database => {
                html! { <Db /> }
            },
            AppRoute::Home => {
                html! { <Home /> }
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
                <header class="footer">         
                    <Link<AppRoute>  to={AppRoute::Calculator}>
                        { "Calculator" }
                    </Link<AppRoute>>
                    <br/>
                    <Link<AppRoute>  to={AppRoute::Database}>
                        { "Database" }
                    </Link<AppRoute>>
                </header>
                <main>
                <Switch<AppRoute> render={Switch::render(switch)} />
                </main>
                
            </BrowserRouter>
        }
    }
}