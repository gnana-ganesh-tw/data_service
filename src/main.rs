use actix_web::{App, HttpServer, post};
use message::MessageData;


mod message;
mod csv_writer;
mod db_writer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting data-writer-service at 8080");
    HttpServer::new(|| App::new()
        .service(csv_writer::write).service(db_writer::db_write))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

