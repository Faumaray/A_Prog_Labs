use yew_router::Routable;
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/error/:msg")]
    Error {msg: String},
    #[at("/")]
    Messenger,
    #[at("/history")]
    HistoryByTime ,
    #[not_found]
    #[at("/404")]
    PageNotFound,
}
// type aliases to make life just a bit easier