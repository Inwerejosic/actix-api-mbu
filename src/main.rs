mod database;
mod handlers;
mod model;
mod schema;

use  actix_web::{web, App, HttpServer};
use crate::handlers::{create_member, delete_member, get_member, update_member, get_member_by_id};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("This Server is up");
    HttpServer::new(|| {
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