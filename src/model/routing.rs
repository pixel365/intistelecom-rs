use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Routing {
    pub currency: String,
    pub mcc: String,
    pub mnc: String,
    pub price: f32,
}
