use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::Switch;

use super::recipes;
use super::single_use;
use recipes::recipe_row::RecipeRow;
use single_use::hero_search::HeroSearch;

#[derive(Debug, Switch, PartialEq, Clone, Copy)]
#[to = "/"]
pub struct Home {}

impl Component for Home {
  type Properties = ();
  type Message = ();
  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }
  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
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
