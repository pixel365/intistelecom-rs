#[allow(dead_code, unreachable_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseOriginator {
    pub originator: String,
    pub default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Originator {
    pub created_at: String,
    pub last_used_at: String,
    pub originator: String,
    pub state: String,
    pub id: u32,
    pub default: bool,
}
