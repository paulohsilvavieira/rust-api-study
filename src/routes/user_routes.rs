use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct Info {
    username: String,
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    return HttpResponse::Ok().body(info.username.clone());
}
