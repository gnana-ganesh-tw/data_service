use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageData {
    #[serde(alias = "RF_ID")]
    pub rf_id: String,
    #[serde(alias = "ChargePointID")]
    pub charge_point_id: String,
    #[serde(alias = "MessageType")]
    pub message_type: String,
    #[serde(alias = "TimeStamp")]
    pub time_stamp: String,
    #[serde(alias = "Status")]
    pub status: String,
}