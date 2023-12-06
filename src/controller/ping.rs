use ntex::web::{self,HttpRequest, HttpResponse};
use chrono::{Local};

#[path = "../data-types/ping-type.rs"] mod ping_object;
/// json_test handler
#[web::get("/ping")]
pub async fn ping(req: HttpRequest) -> HttpResponse {
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };

    let t = &ping_object::PingObject {
        message: "Wellcome to my project rust ntex".to_owned(),
        date_time: Local::now(),
        ip: req.peer_addr(),
        url: req.uri().to_string(),
    };

    HttpResponse::Ok().json(&t) // <- send response
}
