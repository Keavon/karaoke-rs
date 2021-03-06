use crate::{agents::api, components::toast_container::ToastContainer, pages::*};

use log::trace;
use yew::prelude::*;
use yew_router::{prelude::*, switch::Permissive, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Index,
    #[to = "/songs"]
    Songs,
    #[to = "/artists"]
    Artists,
    #[to = "/artist/{artist_id}"]
    Artist(u64),
    #[to = "/queue"]
    Queue,
    #[to = "/player"]
    Player,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}

#[allow(dead_code)]
pub struct Model {
    link: ComponentLink<Self>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    current_route: Option<String>,
    api_agent: Box<dyn Bridge<api::ApiAgent>>,
    player_active: bool,
}

pub enum Msg {
    UpdateHeader(String),
    ApiResponse(api::Response),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|route: Route| Msg::UpdateHeader(route.route));
        let router_agent = RouteAgent::bridge(callback);

        let callback = link.callback(Msg::ApiResponse);
        let api_agent = api::ApiAgent::bridge(callback);

        Model {
            link,
            router_agent,
            current_route: None,
            api_agent,
            player_active: false,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.api_agent.send(api::Request::PlayerActive);
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateHeader(route) => {
                self.current_route = Some(route);
            }
            Msg::ApiResponse(response) => {
                if let api::Response::Success(api::ResponseData::PlayerActive(active)) = response {
                    self.player_active = active;
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <ToastContainer />
                { self.view_header() }
                <main class="content" role="main">
                    { self.view_page() }
                </main>
            </div>
        }
    }
}

impl Model {
    fn view_header(&self) -> Html {
        let current_route = self.current_route.clone().unwrap_or_else(|| "/".into());

        trace!("Current route is: {}", current_route);

        html! {
            <div class="header">
                <img src="/logo.png" class="header__logo" width="64" />
                <nav class="header__navigation">
                    <RouterAnchor<AppRoute> route=AppRoute::Index
                        classes={ if current_route=="/" { "header__navigation-item--active" } else { "header__navigation-item" }}>
                            { "Home" }</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Songs
                        classes={ if current_route=="/songs" { "header__navigation-item--active" } else { "header__navigation-item" }}>
                            { "Songs" }</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Artists
                        classes={ if current_route=="/artists" { "header__navigation-item--active" } else { "header__navigation-item" }}>
                            { "Artists" }</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Queue
                        classes={ if current_route=="/queue" { "header__navigation-item--active" } else { "header__navigation-item" }}>
                            { "Queue" }</RouterAnchor<AppRoute>>
                    {
                        if self.player_active {
                            html! {
                                <RouterAnchor<AppRoute> route=AppRoute::Player
                                    classes={ if current_route=="/player" { "header__navigation-item--active" } else { "header__navigation-item" }}>
                                        { "Player" }</RouterAnchor<AppRoute>>
                            }
                        } else {
                            html! {}
                        }
                    }
                </nav>
            </div>
        }
    }

    fn view_page(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<IndexPage />},
                        AppRoute::Songs => html! {<SongsPage />},
                        AppRoute::Artist(id) => html!{<ArtistPage artist_id=id />},
                        AppRoute::Artists => html!{<ArtistsPage />},
                        AppRoute::Queue => html!{<QueuePage />},
                        AppRoute::Player => html!{<PlayerPage />},
                        AppRoute::NotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoute::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                        _ => html!{"Page not found"},
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}
