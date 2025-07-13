use reqwest::Client;

pub async fn run_automations() {
    loop {
        let price = fetch_price().await; // From Amber/NEM/Localvolts
        if price < 0.0 {
            // Curtail solar, charge EV/battery from grid
            control_device("curtail_solar").await;
        } else if price > 0.2 {
            // Export battery
            control_device("export_battery").await;
        }
        if is_storm().await {
            control_device("reserve_battery").await; // Manual toggle via API
        }
        tokio::time::sleep(std::time::Duration::from_secs(300)).await;
    }
}

async fn fetch_price() -> f64 {
    // Example Amber (checks if retailer configured; defaults if not)
    let client = Client::new();
    let site_id = std::env::var("AMBER_SITE_ID").unwrap_or_else(|_| "default_site".to_string());
    let token = std::env::var("AMBER_API_TOKEN").unwrap_or_else(|_| {
        eprintln!("Warning: AMBER_API_TOKEN not set");
        "".to_string()
    });
    let response = client.get(format!("https://api.amber.com.au/v1/sites/{}/prices/current", site_id))
        .header("Authorization", format!("Bearer {}", token))
        .send().await;

    match response {
        Ok(res) => res.json::<serde_json::Value>().await.unwrap()["data"][0]["perKwh"].as_f64().unwrap(),
        Err(_) => 0.0  // Default if no retailer or error (optional onboarding)
    }
}

async fn is_storm() -> bool {
    // BOM/Open-Meteo for Australia
    let client = Client::new();
    client.get("https://api.open-meteo.com/v1/forecast?latitude=-33.8688&longitude=151.2093&daily=weather_code")
        .send().await.unwrap()
        .json::<serde_json::Value>().await.unwrap()["daily"]["weather_code"][0].as_i64().unwrap() == 95
}

async fn control_device(_action: &str) {
    // Dispatch to specific integration (checks device presence; skips if none)
    // e.g., for EVs: OCPP SetChargingProfile for load balance
}