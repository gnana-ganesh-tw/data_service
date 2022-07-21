use actix_web::{App, HttpResponse, HttpServer, post, web};
use message::MessageData;
use rusqlite::{Connection};


mod message;
mod csv_writer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(csv_writer::write).service(db_write))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


#[post("/write")]
pub async fn db_write(message: web::Json<MessageData>) -> HttpResponse {
    let conn = Connection::open("data.db").unwrap();
    conn.execute("INSERT INTO ocpp_message_store \
        (RF_ID , ChargePointID, MessageType , TimeStamp , Status)\
         VALUES (?1,?2,?3,?4,?5)",
                 (&message.rf_id,
                  &message.charge_point_id,
                  &message.message_type,
                  &message.time_stamp,
                  &message.status),
    ).unwrap();
    conn.close().expect("Unable to close the DB");
    HttpResponse::Ok().body("Write to DB successful")
}

