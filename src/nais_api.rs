use axum::{
    routing::get,
    http::StatusCode, Router,
};
use log::info;

use crate::ApplicationState;

pub async fn register_nais_api(application_state: ApplicationState) {
    let app = Router::new()
        .route("/internal/is_alive", get(is_alive(application_state)))
        .route("/internal/is_ready", get(is_ready(application_state)));

    info!("Server is starting up");
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