use serde::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{
  router::{Router, RouterState},
  Switch,
};

pub mod home;
pub mod navbar;
pub mod recipes;
pub mod searchbar;
pub mod sidebar;
pub mod single_use;

#[derive(Switch, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AppRoute {
  #[to = "/"]
  Home,
}

pub struct RouteWrapper {}

impl Component for RouteWrapper {
  type Properties = ();
  type Message = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn view(&self) -> yew::virtual_dom::VNode<Self> {
    html! {
      <Router<(), AppRoute, ()>
          render = Router::render(|switch: AppRoute| {
              match switch {
                AppRoute::Home => html! { <home::Home /> },
              }
          })
      />
    }
  }
}
