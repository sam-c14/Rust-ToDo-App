use crate::db::{create_task, delete_task, get_all_tasks, get_task, update_task};
use crate::models::{AppResponse, AppState, CreateTaskRequest, UpdateTaskRequest};
use actix_web::{HttpResponse, Responder, delete, get, http::StatusCode, patch, post, web};

pub async fn home(data: web::Data<AppState>) -> impl Responder {
    let app_data = &data.tasks.lock().unwrap();

    let task_count = app_data.len();

    let response = web::Json(AppResponse {
        data: Some(app_data.to_vec()),
        status_code: 200,
        message: format!("Welcome to task manager, You have {task_count} tasks"),
    });

    HttpResponse::Ok().json(response)
}

#[get("")]
pub async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let app_data = data.tasks.lock().unwrap();

    let tasks = get_all_tasks(app_data);

    let message = format!("Tasks retrieved");

    let response = web::Json(AppResponse {
        data: Some(tasks),
        status_code: 200,
        message,
    });

    HttpResponse::Ok().json(response)
}

#[get("/{id}")]
pub async fn get_single_task(data: web::Data<AppState>, path: web::Path<(u32,)>) -> impl Responder {
    let app_data = data.tasks.lock().unwrap();

    let id = path.into_inner().0 as usize;

    let mut response = AppResponse {
        data: None,
        status_code: 200,
        message: String::from("Data retrieved successfully"),
    };

    match get_task(id, app_data) {
        Ok(task) => {
            response.data = Some(vec![task]);
            HttpResponse::Ok().json(web::Json(response))
        }
        Err(e) => {
            response.status_code = 404;
            response.message = e.to_string();
            HttpResponse::build(StatusCode::NOT_FOUND).json(response)
        }
    }
}

#[post("")]
pub async fn insert_task(
    req_body: web::Json<CreateTaskRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let app_data = data.tasks.lock().unwrap();

    let user_data = req_body.into_inner();

    match create_task(user_data.title, user_data.description, app_data) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::build(StatusCode::BAD_REQUEST).body(e.to_string()),
    }
}

#[patch("/{id}")]
pub async fn update_single_task(
    req_body: web::Json<UpdateTaskRequest>,
    data: web::Data<AppState>,
    path: web::Path<(u32,)>,
) -> impl Responder {
    let app_data = data.tasks.lock().unwrap();

    let id = path.into_inner().0;

    let user_data = req_body.into_inner();

    let title = user_data.title;

    let description = user_data.description;

    let status = user_data.status;

    match update_task(id as usize, app_data, title, description, status) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::build(StatusCode::NOT_FOUND).json(web::Json(AppResponse {
            data: None,
            status_code: 404,
            message: e.to_string(),
        })),
    }
}

#[delete("/{id}")]
pub async fn delete_single_task(
    data: web::Data<AppState>,
    path: web::Path<(u32,)>,
) -> impl Responder {
    let app_data = data.tasks.lock().unwrap();

    let id = path.into_inner().0;

    match delete_task(id as usize, app_data) {
        Ok(_status) => HttpResponse::build(StatusCode::NO_CONTENT).finish(),
        Err(e) => HttpResponse::build(StatusCode::NOT_FOUND).json(web::Json(AppResponse {
            data: None,
            status_code: 404,
            message: e.to_string(),
        })),
    }
}

