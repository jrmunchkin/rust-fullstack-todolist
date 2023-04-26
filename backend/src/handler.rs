use std::collections::HashMap;

use crate::db::DB;
use crate::error::Error::*;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use model::*;

#[get("/todos")]
pub async fn get_todos(db: Data<DB>) -> HttpResponse {
    match db.fetch_todos().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/todo")]
pub async fn create_todo(body: Json<TodoCreateRequest>, db: Data<DB>) -> HttpResponse {
    match db.create_todo(&body).await {
        Ok(_todo) => {
            HttpResponse::Ok().json(HashMap::from([("message", "Todo successfully created!")]))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/todo/{id}")]
pub async fn update_todo(
    path: Path<String>,
    body: Json<TodoUpdateRequest>,
    db: Data<DB>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(HashMap::from([("error", "invalid ID")]));
    };
    match db.edit_todo(&id, &body).await {
        Ok(_update) => {
            HttpResponse::Ok().json(HashMap::from([("message", "Todo successfully updated!")]))
        }
        Err(err) => match err {
            InvalidIDError(_id) => HttpResponse::NotFound().json(HashMap::from([(
                "error",
                "No todo found with specified ID",
            )])),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}

#[delete("/todos")]
pub async fn delete_all_todos(db: Data<DB>) -> HttpResponse {
    match db.delete_all_todos().await {
        Ok(_res) => HttpResponse::Ok().json(HashMap::from([(
            "message",
            "All todos successfully deleted!",
        )])),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/todo/{id}")]
pub async fn delete_todo(path: Path<String>, db: Data<DB>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(HashMap::from([("error", "invalid ID")]));
    };
    match db.delete_todo(&id).await {
        Ok(_res) => {
            HttpResponse::Ok().json(HashMap::from([("message", "Todo successfully deleted!")]))
        }
        Err(err) => match err {
            InvalidIDError(_id) => HttpResponse::NotFound().json(HashMap::from([(
                "error",
                "No todo found with specified ID",
            )])),
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}
