use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Properties)]
pub struct Props {
  #[props(required)]
  pub show_searchbar: bool,
}

pub struct Searchbar {
  pub show_searchbar: bool,
}

impl Component for Searchbar {
  type Message = ();
  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      show_searchbar: props.show_searchbar,
    }
  }
  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.show_searchbar = props.show_searchbar;
    true
  }

  fn view(&self) -> Html<Self> {
    html! {
      <div class={if self.show_searchbar {
        "searchbar vertical-center-content open"
      } else {
        "searchbar vertical-center-content"
      }}>
        <i class="material-icons md-20 m-r-15">{"search"}</i>
        <input type="text" placeholder="Search recipes" />
      </div>
    }
  }
}
