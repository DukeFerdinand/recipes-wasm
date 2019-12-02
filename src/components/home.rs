use yew::{
  html,
  services::{ConsoleService, FetchService},
  Bridge, Bridged, Component, ComponentLink, Html, ShouldRender,
};

use super::recipes;
use super::single_use;
use crate::store;
use recipes::recipe_row::RecipeRow;
use single_use::hero_search::HeroSearch;

pub struct Home {
  fetch: FetchService,
  console: ConsoleService,
  store: Box<dyn Bridge<store::Store>>,
}

pub enum Msg {
  CreateResponse(store::Response),
  ReceiveRecipes(store::Response),
}

impl Component for Home {
  type Properties = ();
  type Message = Msg;
  fn create(_props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let connect_state = link.send_back(|res: store::Response| Msg::CreateResponse(res));
    Self {
      fetch: FetchService::new(),
      store: store::Store::bridge(connect_state),
      console: ConsoleService::new(),
    }
  }
  fn mounted(&mut self) -> ShouldRender {
    true
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::CreateResponse(res) => {
        self.console.log(&format! {"{:?}", res });
        true
      }
      Msg::ReceiveRecipes(res) => {
        self.console.log(&format! {"{:?}", res });
        true
      }
    }
  }
  fn view(&self) -> Html<Self> {
    html! {
      <div class="page home-page">
        <HeroSearch />
        <RecipeRow />
      </div>
    }
  }
}
