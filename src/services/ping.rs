use chrono::Local;
use ntex::web::{self, HttpRequest, HttpResponse};
#[path = "../models/ping_type.rs"]
mod ping_type;

/// ping handler
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Ping Pong", body = PingObject),
    ),
)]
#[web::get("/ping")]
pub async fn ping(req: HttpRequest) -> HttpResponse {
    let mut t = String::from("No Ip");

    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
        t = val.ip().to_string();
    };

    let t = &ping_type::PingObject {
        message: "Wellcome to my project rust ntex".to_owned(),
        date_time: Local::now().to_string(),
        ip: Some(t),
        url: req.uri().to_string(),
    };

    HttpResponse::Ok().json(&t) // <- send response
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
