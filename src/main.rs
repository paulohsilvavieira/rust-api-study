mod infra;
mod routes;
use infra::database::connection;

use actix_web::{App, HttpServer};
use routes::user_routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    connection::establish();
    println!("Rust API Rest");
    println!("Running PORT:http://localhost:8080");
    return HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
}
