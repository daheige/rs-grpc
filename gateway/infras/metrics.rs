use crate::infras::shutdown::graceful_shutdown;
use autometrics::objectives::{Objective, ObjectiveLatency, ObjectivePercentile};
use autometrics::prometheus_exporter;
use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

// Add autometrics Service-Level Objectives (SLOs)
// https://docs.autometrics.dev/rust/adding-alerts-and-slos
// Define SLO service level targets for grpc requests, such as success rate, request time.
pub const API_SLO: Objective = Objective::new("grpc")
    // We expect 99.9% of all requests to succeed.
    .success_rate(ObjectivePercentile::P99_9)
    // We expect 99% of all latencies to be below 750ms.
    .latency(ObjectiveLatency::Ms750, ObjectivePercentile::P99);

// prometheus init
pub async fn prometheus_init(port: u16) {
    // Set up prometheus metrics exporter
    prometheus_exporter::init();

    // Build http /metrics endpoint
    let router = Router::new().route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let address: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = TcpListener::bind(address).await.unwrap();
    println!("prometheus at:{}/metrics", address);

    // start http service
    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown(Duration::from_secs(5)))
        .await
        .expect("prometheus metrics init failed");
}
