mod models;
mod api;
mod integrations;
mod automations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use tauri::{self, Manager};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = setup_db().await;

    // Spawn automations
    tokio::spawn(automations::run_automations());

    // Tauri GUI
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_chart_data])
        .run(tauri::generate_context!())
        .unwrap();

    // Actix API
    HttpServer::new(move || App::new()
        .service(web::resource("/onboard").route(web::post().to(api::onboard_device)))
        .service(web::resource("/plans").route(web::get().to(api::get_plans))))
        .bind("0.0.0.0:8080").unwrap()
        .run().await.unwrap();
}

async fn setup_db() -> PgPool {
    // Connect and create tables for devices, plans, etc.
    // ...
}

#[tauri::command]
fn get_chart_data() -> String {
    // Query DB for energy data, return JSON for Chart.js
    "{\"labels\": [\"Now\", \"+1h\"], \"data\": [100, 150]}".to_string()
}
