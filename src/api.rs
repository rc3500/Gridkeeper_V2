use actix_web::{web, HttpResponse};
use reqwest::Client;
use sqlx::PgPool;
use crate::models::{Device, EnergyPlan, Telemetry};

pub async fn onboard_device(pool: web::Data<PgPool>, body: web::Json<Device>) -> HttpResponse {
    // Optional: Skip if type_ is not provided or unknown
    if body.type_.is_empty() {
        return HttpResponse::BadRequest().body("Device type required");
    }
    // Call specific API based on type_ (e.g., Fronius POST /registrations for inverters)
    // Store in DB (allows multiple of same type)
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

#[allow(dead_code)]
pub async fn get_telemetry(device_id: &str) -> Telemetry {
    let client = Client::new();
    client.post("https://developer-api.isolarcloud.com/v2/device/getDevicePointData")
        .header("Authorization", format!("Bearer {}", std::env::var("SUNGROW_API_KEY").unwrap()))
        .json(&serde_json::json!({"device_id": device_id}))
        .send().await.unwrap()
        .json::<Telemetry>().await.unwrap()
}

// New: Onboard retailer (enforces one active)
pub async fn onboard_retailer(pool: web::Data<PgPool>, body: web::Json<EnergyPlan>) -> HttpResponse {
    // Deactivate all others
    sqlx::query("UPDATE energy_plans SET active = false").execute(pool.get_ref()).await.unwrap();
    // Insert new as active
    sqlx::query("INSERT INTO energy_plans (id, retailer, tariff, active) VALUES ($1, $2, $3, true)")
        .bind(&body.id).bind(&body.retailer).bind(body.tariff)
        .execute(pool.get_ref()).await.unwrap();
    HttpResponse::Ok().body("Retailer onboarded and activated")
}