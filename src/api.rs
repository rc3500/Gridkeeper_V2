use actix_web::{web, HttpResponse};
use reqwest::Client;
use sqlx::PgPool;
use crate::models::{Device, EnergyPlan, Telemetry};

pub async fn onboard_device(pool: web::Data<PgPool>, body: web::Json<Device>) -> HttpResponse {
    // Call specific API based on type_ (e.g., Fronius POST /registrations)
    // Store in DB
    sqlx::query("INSERT INTO devices (id, type_, metadata) VALUES ($1, $2, $3)")
        .bind(&body.id).bind(&body.type_).bind(&body.metadata)
        .execute(pool.get_ref()).await.unwrap();
    HttpResponse::Ok().body("Device onboarded")
}

pub async fn get_plans() -> HttpResponse {
    let client = Client::new();
    let plans = client.get("https://cdr.energymadeeasy.gov.au/cds-au/v1/energy/plans")
        .header("x-v", "1").send().await.unwrap()
        .json::<Vec<EnergyPlan>>().await.unwrap();
    HttpResponse::Ok().json(plans)
}

#[allow(dead_code)]  // Added to fix unused function warning
pub async fn get_telemetry(device_id: &str) -> Telemetry {
    let client = Client::new();
    client.post("https://developer-api.isolarcloud.com/v2/device/getDevicePointData")
        .header("Authorization", format!("Bearer {}", std::env::var("SUNGROW_API_KEY").unwrap()))
        .json(&serde_json::json!({"device_id": device_id}))
        .send().await.unwrap()
        .json::<Telemetry>().await.unwrap()
}