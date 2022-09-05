#[path = "../services/mod.rs"]
mod services;
use actix_web::{get, post, web, HttpResponse, Responder};
use services::posts::create_post_service::create_post;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct Info {
    username: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    let result = create_post();
    return HttpResponse::Ok().json(result);
}
