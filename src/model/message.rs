#[allow(dead_code, unreachable_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageId {
    #[serde(alias = "message_id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartId {
    #[serde(alias = "part_id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBody {
    pub destination: String,
    pub originator: String,
    pub text: String,
    pub time_to_send: String,
    pub validity_period: u32,
    pub callback_url: String,
    pub use_local_time: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDetails {
    #[serde(alias = "message_id")]
    pub id: String,
    pub part_id: String,
    pub state: String,
    pub sender: String,
    pub network: String,
    pub dcs: String,
    pub error_description: String,
    pub time_change_state: String,
    pub cost: f32,
    pub error_id: u32,
    pub ported: bool,
}
