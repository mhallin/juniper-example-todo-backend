use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().expect("No .env file found");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set, please update the .env file accordingly");
    
    SqliteConnection::establish(&database_url)
        .expect(&format!("Could not create SQLite database connection to: {}", database_url))
}

infer_schema!("dotenv:DATABASE_URL");
