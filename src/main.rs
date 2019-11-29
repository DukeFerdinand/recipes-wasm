// Library files
extern crate yew;

use yew::prelude::App;
use yew::services::{ConsoleService, FetchService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

// Custom files
mod components;
use components::navbar;

struct Model {
    console: ConsoleService,
    fetch: FetchService,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Model {
        Self {
            console: ConsoleService::new(),
            fetch: FetchService::new(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.console.log("Howdy");
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <navbar::Navbar />
                <button onclick=|_| Msg::DoIt>{"Test the button"}</button>
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
