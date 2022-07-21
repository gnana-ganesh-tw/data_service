use std::fs::OpenOptions;

use actix_web::{App, HttpServer, post, Result, web};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MessageData {
    #[serde(alias = "RF_ID")]
    rf_id: String,
    #[serde(alias = "ChargePointID")]
    charge_point_id: String,
    #[serde(alias = "MessageType")]
    message_type: String,
    #[serde(alias = "TimeStamp")]
    time_stamp: String,
    #[serde(alias = "Status")]
    status: String,
}

#[post("/write")]
async fn write(message: web::Json<MessageData>) -> Result<String> {
    let path = "data.csv";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);    
writer.write_record(&[
        &message.rf_id,
        &message.charge_point_id,
        &message.message_type,
        &message.time_stamp,
        &message.status,
    ]).unwrap();
	println!("{:#?}", message);
	println!("==========================================");
    Ok(format!("{:#?}", message))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(write)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
