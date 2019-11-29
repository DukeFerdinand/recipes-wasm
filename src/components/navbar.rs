use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Navbar {}

impl Component for Navbar {
  type Message = ();
  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }
  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
  fn view(&self) -> Html<Self> {
    html! {
      <div class="navbar">
        <h2 class="navbar-title">{"Recipes"}</h2>
      </div>
    }
  }
}
