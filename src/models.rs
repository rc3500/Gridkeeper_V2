use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Device {
    pub id: String,
    pub type_: String, // e.g., "fronius", "ev_charger"
    pub metadata: serde_json::Value, // Serial, IP, etc.
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct EnergyPlan {
    pub id: String,
    pub retailer: String,
    pub tariff: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Telemetry {
    pub timestamp: String,
    pub power_pv: f64,
    pub power_battery: f64,
    pub power_load: f64,
}

#[allow(dead_code)]  // Added to fix unused fields warning
#[derive(Deserialize, Debug)]
pub struct AutomationRule {
    pub condition: String, // e.g., "price < 0"
    pub action: String,    // e.g., "charge_battery"
}