mod database;
mod handlers;
mod model;
mod schema;

// Importing the handlers to be called in the route
use  actix_cors::Cors;
use  actix_web::{web, App, HttpServer};
use crate::handlers::{create_member, get_member, get_member_by_id, update_member, delete_member};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("This Server is up");
    HttpServer::new(|| {
        // Added this to allow my frontend app to interact effectively with this Rust Actix Backend
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .route("/member", web::post().to(create_member))
            .route("/members", web::get().to(get_member))
            .route("/member/{id}", web::get().to(get_member_by_id))
            .route("/member/{id}", web::put().to(update_member))
            .route("/member/delete/{id}", web::delete().to(delete_member))
    })
        .bind("0.0.0.0:7070")?
        .run()
        .await
}