use ntex::web::{self, middleware, App, HttpRequest};
mod errors;
mod models;
mod services;

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
            // Register swagger endpoints
            .configure(services::openapi::ntex_config)
            // Register todo endpoints
            .configure(services::name::ntex_config)
            .configure(services::json::ntex_config)
            .configure(services::ping::ntex_config)
            // Default endpoint for unregisterd endpoints
            .default_service(web::route().to(services::default))
            .service((
                web::resource("/index.html").to(|| async { "Hello world!" }),
                web::resource("/").to(index),
            ))
    })
    .bind("127.0.0.1:8585")?
    .run()
    .await
}
