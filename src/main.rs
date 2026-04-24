pub mod controllers;
pub mod db;
pub mod models;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use controllers::{
    delete_single_task, get_single_task, get_tasks, home, insert_task, update_single_task,
};
use models::{AppState, DB};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    println!("Server starting on port {PORT}");

    let tasks: DB = Mutex::new(Vec::new());

    let task_db = AppState { tasks };

    let app_data = web::Data::new(task_db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .service(
                web::scope("/tasks")
                    .service(get_tasks)
                    .service(insert_task)
                    .service(get_single_task)
                    .service(update_single_task)
                    .service(delete_single_task),
            )
            .route("/", web::get().to(home))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
