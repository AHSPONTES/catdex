use actix_files::Files;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use serde_json::json;
use std::env;

use handlebars::Handlebars;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
use self::models::*;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path": "/static/image/british-short-hair.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/ragdoll.jpg"
            }
        ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setting up the handlebar template engine
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    // Setting up the database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
