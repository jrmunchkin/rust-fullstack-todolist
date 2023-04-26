use crate::API_URL;
use model::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_create_todo: Callback<()>,
}

pub struct CreateForm {
    link: ComponentLink<Self>,
    props: Props,
    name: String,
    is_complete: bool,
    fetch_task: Option<FetchTask>,
}

impl CreateForm {
    fn render_form(&self) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));

        html! {
            <div class=classes!("task-input")>
                <img src="img/bars-icon.svg" alt="icon"/>
                <input type="text" placeholder="Add a new todo" value={self.name.clone()} oninput={edit_name} />
                <button class=classes!("add-btn", "active") onclick=self.link.callback(move |_| Msg::MakeReq)>{"Add"}</button>
            </div>
        }
    }
}

pub enum Msg {
    MakeReq,
    Resp(Response<Json<Result<(), anyhow::Error>>>),
    EditName(String),
}

impl Component for CreateForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
            is_complete: false,
            props,
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                let body = TodoCreateRequest {
                    name: self.name.clone(),
                    is_complete: self.is_complete.clone(),
                };
                let req = Request::post(API_URL.to_owned() + "/todo")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make req to backend");

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                            Msg::Resp(response)
                        });

                let task = FetchService::fetch(req, cb).expect("can create todo");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if resp.status().is_success() {
                    self.props.on_create_todo.emit(());
                    self.name = String::new();
                }
            }
            Msg::EditName(input) => {
                self.name = input;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}
