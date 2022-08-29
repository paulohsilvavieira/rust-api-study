extern crate diesel;
extern crate dotenv;

#[macro_use]

pub mod connection {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use dotenv::dotenv;
    use std::env;
    pub fn establish() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
