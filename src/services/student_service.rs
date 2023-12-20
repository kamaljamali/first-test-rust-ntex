use crate::{
    db_postgres::*,
    models::students_model::{Fields, StudentModel},
};
use chrono::Local;
use ntex::web::{self, HttpRequest, HttpResponse};

/// student handler
#[utoipa::path(
    get,
    path = "/student",
    responses(
        (status = 200, description = "student data", body = Vec<StudentModel>),
        (status = 500, description = "internal server error"),
    ),
)]
#[web::get("/student")]
pub async fn get_students_test(req: HttpRequest) -> HttpResponse {
    let db_pool = &mut establish_connection();
    match get_all_students2(db_pool).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// json_test handler
#[utoipa::path(
    post,
    path = "/student",
    request_body = StudentModel,
    responses(
      (status = 201, description = "Student created", body = StudentModel),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::post("/student")]
async fn insert_student(item: web::types::Json<StudentModel>) -> HttpResponse {
    let db_pool = &mut establish_connection();
    match inserts_students(db_pool, item).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}
/// json_test handler
#[utoipa::path(
    patch,
    path = "/student",
    request_body = StudentModel,
    responses(
      (status = 201, description = "Student updated", body = StudentModel),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::patch("/student")]
async fn update_student(item: web::types::Json<StudentModel>) -> HttpResponse {
    let db_pool = &mut establish_connection();
    match update_students(db_pool, item).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// json_test handler
#[utoipa::path(
    patch,
    path = "/student/{name}",
    params(
      ("name" = String, Path, description = "Get name for sample"),
    ),
    request_body = Fields,
    responses(
      (status = 201, description = "Student updated", body = bool),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::patch("/student/{name}")]
async fn insert_field_student(
    path: web::types::Path<String>,
    item: web::types::Json<Fields>,
) -> HttpResponse {
    let db_pool = &mut establish_connection();
    let name = &path;
    match insert_field_students(db_pool, name.to_string(), item).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_students_test);
    cfg.service(insert_student);
    cfg.service(update_student);
    cfg.service(insert_field_student);
}
