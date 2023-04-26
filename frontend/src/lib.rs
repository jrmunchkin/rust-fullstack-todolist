#![recursion_limit = "256"]

#[macro_use]
extern crate dotenv_codegen;

mod todo;

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

const API_URL: &str = dotenv!("API_URL");

struct TodoList {}

pub enum Msg {}

impl Component for TodoList {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("wrapper")>
                <todo::list::List />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoList>::new().mount_to_body();
}
