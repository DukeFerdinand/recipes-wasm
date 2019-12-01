use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct RecipeCard {}

impl Component for RecipeCard {
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
      <div class="recipe-card">
        <div class="card-upper">
          <img src="/assets/main-hero.jpg" class="recipe-image" />
        </div>
        <div class="card-lower">
          <div class="recipe-title">{"Title"}</div>
          <div class="servings">{1}</div>
        </div>
      </div>
    }
  }
}
