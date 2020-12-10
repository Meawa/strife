use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder, Result};

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;

use serde::Deserialize;

pub mod schema;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct NewPost {
    title: String,
    body: String,
}

pub fn establish_connection() -> SqliteConnection {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// extract `Info` using serde
// #[post("/subjects/")]
async fn index(post: web::Json<NewPost>) -> Result<impl Responder> {
    Ok(format!("You typed: {:?}", post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
        // .route("/subjects/{school}/posts", route)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
