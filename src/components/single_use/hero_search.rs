use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct HeroSearch {}

impl Component for HeroSearch {
  type Properties = ();
  type Message = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }
  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
  fn view(&self) -> Html<Self> {
    html! {
      <div class="hero-search">
        <img src="/assets/main-hero.jpg" class="hero-banner m-b-50" />
        <div class="hero-content">
          <input
            type="text"
            class="hero-searchbar"
            placeholder="Search recipes"
          />
        </div>
      </div>
    }
  }
}
