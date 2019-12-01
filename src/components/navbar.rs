use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties)]
pub struct Props {
  #[props(required)]
  pub toggle_sidebar: Callback<()>,
  #[props(required)]
  pub toggle_searchbar: Callback<()>,
}

pub struct Navbar {
  props: Props,
}

pub enum Msg {
  ToggleMenu,
  ToggleSearchbar,
}

impl Component for Navbar {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self { props }
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ToggleMenu => {
        self.props.toggle_sidebar.emit(());
        true
      }
      Msg::ToggleSearchbar => {
        self.props.toggle_searchbar.emit(());
        true
      }
    }
  }
  fn view(&self) -> Html<Self> {
    html! {
      <div class="navbar vertical-center-content">
        <h2 class="navbar-title">{"Our Recipes"}</h2>
        <p
          class="search-toggle vertical-center-content align-self-end m-r-15 pointer"
          onclick=|_| Msg::ToggleSearchbar
        >
          <i class="material-icons md-18 m-r-5">{"search"}</i>{"Search"}
        </p>
        <p
          class="menu-toggle vertical-center-content pointer"
          onclick=|_| Msg::ToggleMenu
        >
          <i class="material-icons md-18 m-r-5">{"menu"}</i>{"Menu"}
        </p>
      </div>
    }
  }
}
