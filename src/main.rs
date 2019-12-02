// Library files
extern crate yew;

use yew::prelude::App;
use yew::services::FetchService;
use yew::Bridged;
use yew::{
    html, services::ConsoleService, worker::Bridge, Callback, Component, ComponentLink, Html,
    ShouldRender,
};

// Custom files
mod components;
mod store;
use components::navbar::Navbar;
use components::searchbar::Searchbar;
use components::sidebar::Sidebar;
use store::Store;

struct Model {
    console: ConsoleService,
    fetch: FetchService,
    sidebar_open: bool,
    searchbar_open: bool,
    toggle_sidebar: Callback<()>,
    toggle_searchbar: Callback<()>,
    store: Box<dyn Bridge<store::Store>>,
}

enum Msg {
    ToggleSidebar,
    ToggleSearchbar,
    SetState(store::Response),
}

struct AppState {
    test: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Model {
        let toggle_sidebar = link.send_back(|_| Msg::ToggleSidebar);
        let toggle_searchbar = link.send_back(|_| Msg::ToggleSearchbar);
        let get_state = link.send_back(|state: store::Response| Msg::SetState(state));
        Self {
            console: ConsoleService::new(),
            fetch: FetchService::new(),
            sidebar_open: false,
            searchbar_open: false,
            toggle_sidebar,
            toggle_searchbar,
            store: Store::bridge(get_state),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Used by both navbar and sidebar. one for opening and one for closing
            Msg::ToggleSidebar => {
                self.sidebar_open = !self.sidebar_open;
                true
            }
            Msg::ToggleSearchbar => {
                self.searchbar_open = !self.searchbar_open;
                true
            }
            Msg::SetState(res) => {
                self.console.log("Got state!");
                true
            }
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.store
            .send(store::Actions::GetRecipes(String::from("Hello")));
        true
    }

    fn view(&self) -> yew::virtual_dom::VNode<Self> {
        html! {
            <div class="app-wrapper">
                <Navbar
                    toggle_sidebar=&self.toggle_sidebar,
                    toggle_searchbar=&self.toggle_searchbar
                />
                <Sidebar sidebar_open=&self.sidebar_open, toggle_sidebar=&self.toggle_sidebar />
                <Searchbar show_searchbar=&self.searchbar_open />
                <div class="app-container">
                    <components::RouteWrapper />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
