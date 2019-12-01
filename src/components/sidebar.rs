use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct Sidebar {
  is_open: bool,
  props: Props,
}

#[derive(Properties)]
pub struct Props {
  #[props(required)]
  pub sidebar_open: bool,
  #[props(required)]
  pub toggle_sidebar: Callback<()>,
}

pub enum Msg {
  ToggleMenu,
}

impl Component for Sidebar {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      is_open: props.sidebar_open,
      props,
    }
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ToggleMenu => {
        self.props.toggle_sidebar.emit(());
        true
      }
    }
  }
  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.is_open = props.sidebar_open;
    true
  }
  fn view(&self) -> Html<Self> {
    html! {
      <div class={if self.is_open {
        "sidebar open"
      } else {
        "sidebar"
      }}>
        <div class="sidebar-title-wrapper vertical-center-content">
          <h2 class="sidebar-title">{"Sidebar"}</h2>
          <i onclick=|_| Msg::ToggleMenu, class="material-icons md-18">{"close"}</i>
        </div>
      </div>
    }
  }
}
