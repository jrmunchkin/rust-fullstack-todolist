use crate::API_URL;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::Callback;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
    pub on_delete_todo: Callback<()>,
}

pub struct DeleteForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    props: Props,
}

impl DeleteForm {
    fn render_form(&self) -> Html {
        html! {
            <><i onclick=self.link.callback(move |_| Msg::MakeReq) class=classes!("uil", "uil-trash")></i></>
        }
    }
}

pub enum Msg {
    MakeReq,
    Resp(Response<Json<Result<(), anyhow::Error>>>),
}

impl Component for DeleteForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.render_form() }
            </>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                let req = Request::delete(API_URL.to_owned() + "/todo/" + &self.props.id)
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                            Msg::Resp(response)
                        });

                let todo = FetchService::fetch(req, cb).expect("can delete todo");
                self.fetch_task = Some(todo);
                ()
            }
            Msg::Resp(resp) => {
                if resp.status().is_success() {
                    self.props.on_delete_todo.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
