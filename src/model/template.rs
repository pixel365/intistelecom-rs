use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseTemplate {
    pub template: String,
    pub title: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub created_at: String,
    pub updated_at: String,
    pub template: String,
    pub title: String,
    pub id: u32,
}
