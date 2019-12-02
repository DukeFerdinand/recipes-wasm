use serde::{Deserialize, Serialize};
use yew::{services::ConsoleService, services::FetchService, worker::*};

mod recipe_handlers;

pub struct Store {
  link: AgentLink<Store>,
  fetch: FetchService,
  api: Box<dyn Bridge<recipe_handlers::Api>>,
}

pub enum Msg {
  FetchRecipes(String),
  ContextMsg(recipe_handlers::Response),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Actions {
  // String in this case is a query String
  GetRecipes(String),
  GetLatest(u8), // StoreRecipes(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
  GetRecipesResponse(String),
  GetLatestResponse(String),
}

impl Agent for Store {
  type Reach = Context;
  type Message = Msg;
  type Input = Actions;
  type Output = Response;

  fn create(link: AgentLink<Self>) -> Self {
    let get_recipes_callback =
      link.send_back(|res: recipe_handlers::Response| Msg::ContextMsg(res));
    let mut console = ConsoleService::new();
    console.log("State created");
    Self {
      link,
      fetch: FetchService::new(),
      api: recipe_handlers::Api::bridge(get_recipes_callback),
    }
  }

  fn update(&mut self, internal_msg: Self::Message) {
    match internal_msg {
      Msg::FetchRecipes(_query) => {}
      Msg::ContextMsg(res) => {}
    }
  }

  fn handle(&mut self, action: Self::Input, sender: HandlerId) {
    match action {
      Actions::GetRecipes(query) => {
        let mut console = ConsoleService::new();
        console.log(&query);
        self
          .link
          .response(sender, Response::GetRecipesResponse("Test Response".into()))
      }
      Actions::GetLatest(count) => {
        self
          .link
          .response(sender, Response::GetLatestResponse("Got latest".into()));
      }
    }
  }
}
