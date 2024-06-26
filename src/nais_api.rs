use axum::{
    routing::get,
    http::StatusCode, Router,
};
use axum::extract::State;
use prometheus::{Encoder, TextEncoder};

use crate::ApplicationState;

pub async fn register_nais_api(application_state: ApplicationState) {
    let routes = nais_routes(application_state);

    let listener =
        tokio::net::TcpListener::bind(&"0.0.0.0:8080").await.unwrap();

    axum::serve(listener, routes).await.unwrap();
}

fn nais_routes(application_state: ApplicationState) -> Router {
    Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
        .route("/internal/prometheus", get(prometheus))
        .with_state(application_state)
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

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use axum::http::StatusCode;
    use axum_test::{TestResponse, TestServer};
    use serde_derive::{Deserialize, Serialize};

    use crate::ApplicationState;
    use crate::nais_api::nais_routes;

    #[derive(Serialize, Deserialize)]
    struct Manifest {
        spec: Spec,
    }

    #[derive(Serialize, Deserialize)]
    struct Spec {
        liveness: Actuator,
        readiness: Actuator,
        prometheus: Actuator,
    }

    #[derive(Serialize, Deserialize)]
    struct Actuator {
        path: String,
    }

    fn nais_manifest(filename: &'static str) -> Manifest {
        let dir = std::env::current_dir().unwrap();
        let path = dir.join(filename);
        let content = read_to_string(path.to_str().unwrap()).unwrap();
        serde_yml::from_str(&content).unwrap()
    }

    #[tokio::test]
    async fn configured_liveness_returns_200_when_state_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: true,
                ready: false,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");
        let dev_res = server.get(&dev_manifest.spec.liveness.path).await;
        assert_eq!(dev_res.status_code(), StatusCode::OK);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let prod_res = server.get(&prod_manifest.spec.liveness.path).await;
        assert_eq!(prod_res.status_code(), StatusCode::OK)
    }

    #[tokio::test]
    async fn configured_liveness_returns_500_when_state_not_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");
        let dev_res: TestResponse = server.get(&dev_manifest.spec.liveness.path).await;
        assert_eq!(dev_res.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let prod_res = server.get(&prod_manifest.spec.liveness.path).await;
        assert_eq!(prod_res.status_code(), StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[tokio::test]
    async fn configured_readiness_returns_200_when_state_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: true,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");
        let dev_res = server.get(&dev_manifest.spec.readiness.path).await;
        assert_eq!(dev_res.status_code(), StatusCode::OK);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let prod_res = server.get(&prod_manifest.spec.readiness.path).await;
        assert_eq!(prod_res.status_code(), StatusCode::OK)
    }

    #[tokio::test]
    async fn configured_readiness_returns_500_when_state_not_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");
        let dev_res = server.get(&dev_manifest.spec.readiness.path).await;
        assert_eq!(dev_res.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let prod_res = server.get(&prod_manifest.spec.readiness.path).await;
        assert_eq!(prod_res.status_code(), StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[tokio::test]
    async fn configured_prometheus_scrapable() {
        let routes = nais_routes(
            ApplicationState {
                alive: true,
                ready: true,
            }
        );
        let server = TestServer::new(routes).unwrap();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");
        let dev_res = server.get(&dev_manifest.spec.prometheus.path).await;
        assert_eq!(dev_res.status_code(), StatusCode::OK);
        assert_eq!(dev_res.text().as_str(), String::new());

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let prod_res = server.get(&prod_manifest.spec.prometheus.path).await;
        assert_eq!(prod_res.status_code(), StatusCode::OK);
        assert_eq!(prod_res.text().as_str(), String::new());
    }
}
