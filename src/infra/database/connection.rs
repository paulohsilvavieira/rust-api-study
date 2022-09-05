extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url);

    if connection.is_ok() {
        println!("Connection Established!")
    }

    return connection.expect("Error Ocurred on establish connection Database");
}
