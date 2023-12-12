pub mod openapi;
pub mod name;
pub mod json;
pub mod ping;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}