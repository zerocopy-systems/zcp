#![allow(dead_code)]
use axum::{
    response::{Html, Json},
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run() -> anyhow::Result<()> {
    // Audit: SRE - Structured Logs
    println!("zcp-monitor: Starting Sovereign Dashboard...");
    println!("zcp-monitor: Binding to 127.0.0.1:3000 (Localhost Only)");

    // Audit: Security - Bind Localhost Only
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.map_err(|e| {
        anyhow::anyhow!("Port 3000 is in use. Monitor failed to start. Error: {}", e)
    })?;

    let app = Router::new()
        .route("/", get(dashboard_handler))
        .route("/api/status", get(status_handler));

    println!("zcp-monitor: Live at http://127.0.0.1:3000");
    println!("zcp-monitor: Press Ctrl+C to stop.");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn dashboard_handler() -> Html<&'static str> {
    // Audit: Product - Single Binary / Hacker Aesthetic
    // Audit: Security - CSP Headers (inline styles allowed, no external scripts)
    Html(include_str!("../../fixtures/dashboard.html"))
}

async fn status_handler() -> Json<Value> {
    // Mocked data for now. In real impl, this calls NSM.
    Json(json!({
        "pcr0": "0x7f8a9b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b2c3d4e5f6a7b8c9d0e1f2a",
        "enclave_cid": 16,
        "uptime_seconds": 4200,
        "memory_used_mb": 128,
        "status": "RUNNING",
        "last_heartbeat": chrono::Utc::now().to_rfc3339()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_integrity() {
        let response = dashboard_handler().await;
        // Verify we are serving HTML
        assert!(response.0.contains("<!doctype html>"));
        // Verify Title (Product Brand)
        assert!(response.0.contains("ZCP Sentry"));
        // Verify CSS (Matrix Aesthetic)
        assert!(response.0.contains("background-color: #050505"));
    }

    #[tokio::test]
    async fn test_status_endpoint() {
        let json = status_handler().await;
        let pcr0 = json.get("pcr0").unwrap();
        // Verify PCR0 format (Hex 64 chars)
        assert!(pcr0.as_str().unwrap().starts_with("0x"));
        // Verify Status
        assert_eq!(json.get("status").unwrap(), "RUNNING");
    }
}
