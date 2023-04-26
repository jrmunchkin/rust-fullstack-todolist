use crate::API_URL;
use model::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub is_complete: bool,
    pub on_update_todo: Callback<()>,
}

pub struct UpdateForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    props: Props,
}

impl UpdateForm {
    fn render_form(&self) -> Html {
        if self.props.is_complete {
            html! {
                <><input onclick=self.link.callback(move |_| Msg::MakeReq) type="checkbox" id={self.props.id.clone()} checked={true}/>
                <p class=classes!("checked")>{ &self.props.name }</p></>
            }
        } else {
            html! {
                <><input onclick=self.link.callback(move |_| Msg::MakeReq) type="checkbox" id={self.props.id.clone()} />
                <p >{ &self.props.name }</p></>
            }
        }
    }
}

pub enum Msg {
    MakeReq,
    Resp(Response<Json<Result<(), anyhow::Error>>>),
}

impl Component for UpdateForm {
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
                let body = TodoUpdateRequest {
                    is_complete: !self.props.is_complete.clone(),
                };
                let req = Request::put(API_URL.to_owned() + "/todo/" + &self.props.id)
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
                    self.props.on_update_todo.emit(());
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
