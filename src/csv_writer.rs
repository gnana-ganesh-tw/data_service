use actix_web::{HttpResponse, web};
use std::fs::OpenOptions;
use crate::{MessageData, post};

#[post("/write-csv")]
pub async fn write(message: web::Json<MessageData>) -> HttpResponse {
    let path = "data.csv";
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);
    writer.write_record(&[&message.rf_id, &message.charge_point_id, &message.message_type, &message.time_stamp, &message.status, ]).unwrap();
    println!("{:#?}", message);
    println!("==========================================");
    HttpResponse::Ok().finish()
}
