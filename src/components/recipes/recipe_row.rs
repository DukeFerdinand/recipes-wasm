use yew::{html, Component, ComponentLink, Html, ShouldRender};

use super::recipe_card;

pub struct RecipeRow {}

impl Component for RecipeRow {
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
      <div class="recipe-row">
        <p class="row-title">{"Most recent"}</p>
        <div class="row-content">
          <recipe_card::RecipeCard />
        </div>
      </div>
    }
  }
}
