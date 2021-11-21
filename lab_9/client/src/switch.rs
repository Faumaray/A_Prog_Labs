use yew_router::Routable;
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/error/:msg")]
    Error {msg: String},
    #[at("/calculator")]
    Calculator,
    #[at("/database")]
    Database ,
    #[not_found]
    #[at("/404")]
    PageNotFound,
    #[at("/")]
    Home,
}
// type aliases to make life just a bit easier