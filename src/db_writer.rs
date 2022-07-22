use actix_web::{HttpResponse, web};
use rusqlite::Connection;
use crate::{MessageData, post};

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
    println!("{:#?}", message);
    println!("============================================");
    HttpResponse::Ok().body("Write to DB successful")
}
