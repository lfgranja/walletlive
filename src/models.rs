use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
}

#[derive(Serialize, Clone)]
pub struct UserAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub quantity: f64,
}

impl UserAsset {
    pub fn total_value(&self) -> f64 {
        self.quantity * self.unit_value
    }
}

pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}