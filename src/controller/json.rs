use ntex::web::{self,  HttpResponse};
#[path = "../data-types/ping-type.rs"] mod my_obj;
/// json_test handler
#[web::get("/json_test")]
async fn json_test() -> HttpResponse {
    let t = &my_obj::MyObj {
        first_name: "Kamal".to_owned(),
        last_name: "Jamali".to_owned(),
        id: 112,
    };

    HttpResponse::Ok().json(&t) // <- send response
}
/// json_test handler
#[web::post("/json_test_post")]
async fn json_test_post(item: web::types::Json<my_obj::MyObj>) -> HttpResponse {
    println!("model: {:?}", &item.first_name);
    HttpResponse::Ok().json(&item.0) // <- send response
}
