use axum::{Json, Router};
use axum::routing::get;
use kameo::remote;

use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::{ WithExportConfig, LogExporter };
use opentelemetry_sdk::{
    Resource,
    logs::SdkLoggerProvider
};
use serde_json::{json, Value};
use tracing::{info};
use tracing_subscriber::{prelude::*, EnvFilter};

#[tokio::main]
async fn main() {

    let provider = otel_config();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    let peer_id = remote::bootstrap().unwrap();

    info!("Starting server on {}", address);
    info!(port,"Peer ID: {}", peer_id);

    let app = Router::new()
        .route("/", get(handle_dummy_get))
        .route("/health", get(|| async { "OK".to_string()}));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let _ = provider.shutdown();
}

fn otel_config() -> SdkLoggerProvider {
    let resource = Resource::builder()
        .with_service_name("api")
        .build();
    let exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317".to_string())
        .with_timeout(std::time::Duration::from_secs(3))
        .build()
        .expect("Failed to create exporter");

    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(exporter)
        .build();

    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());

    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=debug".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();
    provider
}

async fn handle_dummy_get() -> Json<Value> {
    Json(json!("Hello, world!"))
}