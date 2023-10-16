use axum::{
    routing::get,
    http::StatusCode, Router,
};
use prometheus::{Encoder, TextEncoder};

use crate::ApplicationState;

pub async fn register_nais_api(application_state: ApplicationState) {
    let app = Router::new()
        .route("/internal/is_alive", get(is_alive(application_state)))
        .route("/internal/is_ready", get(is_ready(application_state)))
        .route("/internal/prometheus", get(prometheus()));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn is_alive(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.alive {
        true => {
            (StatusCode::OK, "I'm alive! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x")
    }
}


fn is_ready(application_state: ApplicationState) -> (StatusCode, &'static str) {
    match application_state.ready {
        true => {
            (StatusCode::OK, "I'm ready! :)")
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Please wait! I'm not ready :(")
    }
}

fn prometheus() -> (StatusCode, [(&'static str, &'static str); 1], String) {

    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let output_string = String::from_utf8(buffer.clone()).unwrap();

    (StatusCode::OK, [("content-type", "text/plain; version=0.0.4; charset=utf-8")], output_string)
}
