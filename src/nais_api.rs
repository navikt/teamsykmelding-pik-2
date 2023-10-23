use axum::{
    routing::get,
    http::StatusCode, Router,
};
use axum::extract::State;
use axum::handler::Handler;
use prometheus::{Encoder, TextEncoder};

use crate::ApplicationState;

pub async fn register_nais_api(application_state: ApplicationState) {
    let app = Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
        .route("/internal/prometheus", get(prometheus))
        .with_state(application_state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn is_alive(State(application_state): State<ApplicationState>) -> (StatusCode, &'static str) {
    match application_state.alive {
        true => {
            (StatusCode::OK, "I'm alive! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x")
    }
}


async fn is_ready(State(application_state): State<ApplicationState>) -> (StatusCode, &'static str) {
    match application_state.ready {
        true => {
            (StatusCode::OK, "I'm ready! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Please wait! I'm not ready :(")
    }
}


async fn prometheus() -> (StatusCode, [(&'static str, &'static str); 1], String) {

    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();


    (StatusCode::OK, [("content-type", "text/plain; version=0.0.4")], String::from_utf8(buffer).unwrap())
}

