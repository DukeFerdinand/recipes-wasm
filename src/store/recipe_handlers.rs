use serde::{Deserialize, Serialize};
use yew::{services::FetchService, worker::*};

pub struct Api {
  link: AgentLink<Api>,
  fetch: FetchService,
}

#[derive(Serialize, Deserialize)]
pub enum Actions {
  GetRecipes(String),
}

#[derive(Serialize, Deserialize)]
pub enum Response {
  ReturnRecipes(String),
}

impl Agent for Api {
  type Reach = Context;
  type Message = ();
  type Input = Actions;
  type Output = Response;

  fn create(link: AgentLink<Self>) -> Self {
    Self {
      link,
      fetch: FetchService::new(),
    }
  }

  fn update(&mut self, _: Self::Message) {}

  fn handle(&mut self, action: Self::Input, sender: HandlerId) {
    match action {
      Actions::GetRecipes(_query) => self
        .link
        .response(sender, Response::ReturnRecipes("Test success".into())), // Actions::GetRecipes(_) => {}
    }
  }
}
