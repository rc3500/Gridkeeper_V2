mod models;
mod api;
mod integrations;
mod automations;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use tauri::{self};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _pool = setup_db().await;  // Prefix with _ to fix unused warning

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
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&db_url).await.expect("Failed to connect to DB")
}

#[tauri::command]
fn get_chart_data() -> String {
    // Query DB for energy data, return JSON for Chart.js
    "{\"labels\": [\"Now\", \"+1h\"], \"data\": [100, 150]}".to_string()
}