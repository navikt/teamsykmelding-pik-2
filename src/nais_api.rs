use axum::{
    routing::get,
    http::StatusCode, Router,
};
use axum::extract::State;
use log::info;
use prometheus::{Encoder, TextEncoder};

use crate::ApplicationState;

pub async fn register_nais_api(application_state: ApplicationState) {
    let app = nais_routes(application_state);

    let listener =
        tokio::net::TcpListener::bind(&"0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    info!("Server has started");

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
    use axum::body::Body;

    use axum::http::{Request, StatusCode};
    use serde_derive::{Deserialize, Serialize};
    use tokio::net::TcpListener;

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
        serde_yaml::from_str(&content).unwrap()
    }

    #[tokio::test]
    async fn configured_liveness_returns_200_when_state_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: true,
                ready: false,
            }
        );

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();


        let dev_manifest = nais_manifest("naiserator-dev.yaml");

        let liveness_path_dev = dev_manifest.spec.liveness.path;

        let dev_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{liveness_path_dev}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(dev_res.status(), StatusCode::OK);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");
        let liveness_path_prod = prod_manifest.spec.liveness.path;


        let prod_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{liveness_path_prod}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();
        assert_eq!(prod_res.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn configured_liveness_returns_500_when_state_not_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();


        let dev_manifest = nais_manifest("naiserator-dev.yaml");

        let liveness_path_dev = dev_manifest.spec.liveness.path;

        let dev_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{liveness_path_dev}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(dev_res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");

        let liveness_path_prod = prod_manifest.spec.liveness.path;

        let prod_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{liveness_path_prod}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(prod_res.status(), StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[tokio::test]
    async fn configured_readiness_returns_200_when_state_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: true,
            }
        );

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();


        let dev_manifest = nais_manifest("naiserator-dev.yaml");

        let readiness_path_dev = dev_manifest.spec.readiness.path;

        let dev_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{readiness_path_dev}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();


        assert_eq!(dev_res.status(), StatusCode::OK);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");

        let readiness_path_prod = prod_manifest.spec.readiness.path;

        let prod_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{readiness_path_prod}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(prod_res.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn configured_readiness_returns_500_when_state_not_alive() {
        let routes = nais_routes(
            ApplicationState {
                alive: false,
                ready: false,
            }
        );
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");

        let readiness_path_dev = dev_manifest.spec.readiness.path;

        let dev_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{readiness_path_dev}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();


        assert_eq!(dev_res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let prod_manifest = nais_manifest("naiserator-prod.yaml");

        let readiness_path_prod = prod_manifest.spec.readiness.path;

        let prod_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{readiness_path_prod}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(prod_res.status(), StatusCode::INTERNAL_SERVER_ERROR)
    }

    #[tokio::test]
    async fn configured_prometheus_scrapable() {
        let routes = nais_routes(
            ApplicationState {
                alive: true,
                ready: true,
            }
        );
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        let dev_manifest = nais_manifest("naiserator-dev.yaml");

        let prometheus_path_dev = dev_manifest.spec.prometheus.path;

        let dev_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{prometheus_path_dev}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(dev_res.status(), StatusCode::OK);
        assert_eq!(dev_res.body(), String::new());

        let prod_manifest = nais_manifest("naiserator-prod.yaml");

        let prometheus_path_prod = prod_manifest.spec.prometheus.path;

        let prod_res = client.request(
            Request::builder()
                .uri(format!("http://{addr}{prometheus_path_prod}"))
                .header("Host", "localhost")
                .body(Body::empty())
                .unwrap(),
        )
            .await
            .unwrap();

        assert_eq!(prod_res.status(), StatusCode::OK);
        assert_eq!(prod_res.body(), String::new());
    }
}
