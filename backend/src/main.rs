#[macro_use]
extern crate dotenv_codegen;

mod cron;
mod db;
mod error;
mod handler;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use handler::{create_todo, delete_all_todos, delete_todo, get_todos, update_todo};
use std::process;

const DB_NAME: &str = dotenv!("DB_NAME");
const PORT: &str = dotenv!("PORT");
const MONGO_URL: &str = dotenv!("MONGO_URL");
const ENABLE_CRON: &str = dotenv!("ENABLE_CRON");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if ENABLE_CRON == "true" {
        match cron::start_cron().await {
            Ok(()) => (),
            Err(e) => println!("Cron went wrong : {}", e),
        }
    }

    let db = match db::DB::init().await {
        Ok(db) => {
            println!("Connected to database successfuly");
            db
        }
        Err(e) => {
            println!("Error connecting database: {}", e);
            process::exit(1);
        }
    };

    let todo = Data::new(db);

    println!("Starting server on port {}", PORT);

    match HttpServer::new(move || {
        App::new()
            .app_data(todo.clone())
            .wrap(Cors::permissive())
            .service(create_todo)
            .service(update_todo)
            .service(delete_all_todos)
            .service(delete_todo)
            .service(get_todos)
    })
    .bind(("127.0.0.1", PORT.parse().unwrap()))?
    .run()
    .await
    {
        Ok(db) => db,
        Err(e) => {
            println!("Error starting server: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}
