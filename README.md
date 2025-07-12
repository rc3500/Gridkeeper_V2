# Gridkeeper: Smart Energy Management Platform

Gridkeeper automates inverters, batteries, and EV chargers based on wholesale prices (NEM/Amber/Localvolts) and forecasts. Supports Fronius, Sungrow, Goodwe, Deye, Tesla, Sigenergy; OCPP EVs; Shelly devices.

## Setup
- Install Rust: https://rustup.rs
- Run locally: `cargo run`
- Build Windows EXE: `cargo tauri build`
- Docker: `docker build -t gridkeeper . && docker run -p 8080:8080 gridkeeper`
- HA Add-on: Add this repo to HA Supervisor > Add-on Store > Repositories.

## Features
- Easy onboarding wizard.
- Charts/forecasts via Solcast/BOM.
- Automations: Charge on cheap/negative prices, export on expensive, storm reserve.
- Multi-device support with EV load balancing.

API Keys: Copy `.env.example` to `.env` and fill in.
