#[path = "../services/mod.rs"]
mod services;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct Info {
    username: String,
}
struct Result {
    success: bool,
}

#[derive(Deserialize, Debug, Serialize)]

struct ErrorStruct {
    msg: String,
}

fn verify_auth(req: &HttpRequest) -> Result {
    if req
        .headers()
        .get("authorization")
        .unwrap()
        .eq("secret12345")
    {
        return Result { success: true };
    }
    return Result { success: false };
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
async fn echo(req: HttpRequest, info: web::Json<Info>) -> impl Responder {
    let result = verify_auth(&req);
    if result.success {
        return HttpResponse::Ok().json(info);
    }
    let response = ErrorStruct {
        msg: String::from("Unauthorized"),
    };
    return HttpResponse::Unauthorized().json(response);
}
