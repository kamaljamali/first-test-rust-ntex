use ntex::web::{self, middleware, App, HttpRequest};
#[path = "controller/ping.rs"] mod ping;
#[path= "controller/json.rs"] mod json;
#[path= "controller/name.rs"] mod name;

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
                name::name_id,
                name::only_name,
                json::json_test,
                json::json_test_post,
                ping::ping,
                web::resource("/index.html").to(|| async { "Hello world!" }),
                web::resource("/").to(index),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
