use ntex::web::{self, middleware, App, Error, HttpRequest, HttpResponse};
use ntex::{channel::mpsc, util::Bytes};
use chrono::{DateTime,Local};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr};
#[derive(Deserialize,Serialize)]
struct MyObj {
    first_name: String,
    last_name: String,
    id: i32,
}

#[derive(Deserialize,Serialize)]
struct PingObject{
    message:String,
    date_time:DateTime<Local>,
    ip:Option<SocketAddr>,
    url:String,
}
/// json_test handler
#[web::get("/ping")]
async fn ping(req: HttpRequest) -> HttpResponse {
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };

    let t = &PingObject {
        message:"Wellcome to my project rust ntex".to_owned(),
        date_time:Local::now(),
        ip:req.peer_addr(),
        url:req.uri().to_string(),    
    };

    HttpResponse::Ok().json(&t) // <- send response
}
/// json_test handler
#[web::get("/json_test")]
async fn json_test() -> HttpResponse {
    let t = &MyObj {
        first_name: "Kamal".to_owned(),
        last_name: "Jamali".to_owned(),
        id: 112,
    };

    HttpResponse::Ok().json(&t) // <- send response
}
/// json_test handler
#[web::post("/json_test_post")]
async fn json_test_post(item: web::types::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item.first_name);
    HttpResponse::Ok().json(&item.0) // <- send response
}

/// only_name handler
#[web::get("/only_name/{name}")]
async fn only_name(path: web::types::Path<String>) -> HttpResponse {
    println!("Your path is: {:?}", path);
    let name = &path;
    let text = format!("Hello dear {}!", *name);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// name_id handler
#[web::get("/name_id/{name}/{id}")]
async fn name_id(path: web::types::Path<(String, String)>) -> HttpResponse {
    println!("Your path is: {:?}", path);
    let (name, id) = &path.into_inner();
    println!("Your name is: {:?} and id {:?}", name, id);
    let text = format!("Hello dear {} with Id {}!", *name, *id);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service((
                name_id,
                only_name,
                json_test,
                json_test_post,
                ping,
                web::resource("/index.html").to(|| async { "Hello world!" }),
                web::resource("/").to(index),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex::util::Bytes;
    use ntex::web::{test, App, Error};
    use ntex::Service;
    use ntex::{http, web};

    #[ntex::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let bytes = test::read_body(resp).await;

        assert_eq!(bytes, Bytes::from(r##"Hello world!"##));

        Ok(())
    }
}
