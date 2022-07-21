use actix_web::{App, HttpServer, post};
use message::MessageData;


mod message;
mod csv_writer;
mod db_writer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(csv_writer::write).service(db_writer::db_write))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

