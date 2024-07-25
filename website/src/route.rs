use yew_router::Routable;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/shadow-realm")]
    ShadowRealm,
    #[at("/")]
    NormalRealm,
    #[not_found]
    #[at("/page-not-found")]
    NotFound,
}
