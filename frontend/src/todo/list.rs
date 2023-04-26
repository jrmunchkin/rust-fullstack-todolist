use crate::todo;
use crate::API_URL;
use model::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct List {
    fetch_task: Option<FetchTask>,
    todos: Option<Vec<TodoResponse>>,
    todos_displayed: Option<Vec<TodoResponse>>,
    active_class: String,
    link: ComponentLink<Self>,
}

impl List {
    fn render_view(&self) -> Html {
        let all_class = if self.active_class == "all" {
            "active"
        } else {
            ""
        };
        let pending_class = if self.active_class == "pending" {
            "active"
        } else {
            ""
        };
        let complete_class = if self.active_class == "complete" {
            "active"
        } else {
            ""
        };
        let clear_all_class: &str;
        match self.todos.clone() {
            Some(_) => {
                clear_all_class = "active";
            }
            None => {
                clear_all_class = "";
            }
        };
        html! {
            <>
                <todo::create::CreateForm on_create_todo=self.link.callback(move |_| Msg::MakeReq)/>
                <div class=classes!("controls")>
                    <div class=classes!("filters")>
                        <span onclick=self.link.callback(move |_| Msg::Active) class=classes!(all_class) id="all">{ "All"}</span>
                        <span onclick=self.link.callback(move |_| Msg::Pending) class=classes!(pending_class) id="pending">{ "Pending" }</span>
                        <span onclick=self.link.callback(move |_| Msg::Complete) class=classes!(complete_class) id="completed">{ "Completed" }</span>
                    </div>
                    <button onclick=self.link.callback(move |_| Msg::MakeReqDelete) class=classes!("clear-btn", clear_all_class)>{ "Clear All" }</button>
                </div>
                { self.view_list() }
            </>
        }
    }

    fn view_list(&self) -> Html {
        let overflow_class: &str;
        match self.todos.clone() {
            Some(t) => {
                if t.len() > 5 {
                    overflow_class = "overflow";
                } else {
                    overflow_class = ""
                }
            }
            None => {
                overflow_class = "";
            }
        };
        if let Some(t) = &self.todos_displayed {
            html! {
                <ul class=classes!("task-box", overflow_class)>
                    { t.iter().map(|name| self.view_todo(name)).collect::<Html>() }
                </ul>
            }
        } else {
            html! {
                <ul class=classes!("task-box")>
                    <span>{ "You don't have any todo here" }</span>
                </ul>
            }
        }
    }

    fn view_todo(&self, todo: &TodoResponse) -> Html {
        html! {
            <li class=classes!("task")>
                <label for={todo.id.clone()} >
                    <todo::update::UpdateForm id={todo.id.clone()} name={todo.name.clone()} is_complete={todo.is_complete.clone()} on_update_todo=self.link.callback(move |_| Msg::MakeReq)/>
                </label>
                <div class=classes!("settings")>
                    <todo::delete::DeleteForm id={todo.id.clone()} on_delete_todo=self.link.callback(move |_| Msg::MakeReq)/>
                </div>
            </li>
        }
    }
}

pub enum Msg {
    MakeReq,
    Resp(Result<Vec<TodoResponse>, anyhow::Error>),
    MakeReqDelete,
    RespDelete(Response<Json<Result<(), anyhow::Error>>>),
    Active,
    Pending,
    Complete,
}

impl Component for List {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            fetch_task: None,
            link,
            todos: None,
            todos_displayed: None,
            active_class: String::from("all"),
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_view() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                let req = Request::get(API_URL.to_owned() + "/todos")
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<TodoResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let todos = FetchService::fetch(req, cb).expect("can fetch todos");
                self.fetch_task = Some(todos);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = if data.len() > 0 { Some(data) } else { None };
                    if self.active_class == "all".to_string() {
                        self.link.send_message(Msg::Active);
                    }
                    if self.active_class == "pending".to_string() {
                        self.link.send_message(Msg::Pending);
                    }
                    if self.active_class == "complete".to_string() {
                        self.link.send_message(Msg::Complete);
                    }
                }
            }
            Msg::MakeReqDelete => {
                let req = Request::delete(API_URL.to_owned() + "/todos")
                    .body(Nothing)
                    .expect("can make req to backend");

                let cb =
                    self.link
                        .callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                            Msg::RespDelete(response)
                        });

                let todos = FetchService::fetch(req, cb).expect("can delete todos");
                self.fetch_task = Some(todos);
                ()
            }
            Msg::RespDelete(resp) => {
                if resp.status().is_success() {
                    self.link.send_message(Msg::MakeReq);
                }
            }
            Msg::Active => {
                self.todos_displayed = self.todos.clone();
                self.active_class = String::from("all");
            }
            Msg::Pending => {
                let filtered_values: Option<Vec<TodoResponse>> = self.todos.clone().map(|vec| {
                    vec.into_iter()
                        .filter(|value| value.is_complete == false)
                        .collect()
                });
                self.todos_displayed = filtered_values;
                self.active_class = String::from("pending");
            }
            Msg::Complete => {
                let filtered_values: Option<Vec<TodoResponse>> = self.todos.clone().map(|vec| {
                    vec.into_iter()
                        .filter(|value| value.is_complete == true)
                        .collect()
                });
                self.todos_displayed = filtered_values;
                self.active_class = String::from("complete");
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}
