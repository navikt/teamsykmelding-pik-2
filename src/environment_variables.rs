use std::env;
use serde_derive::{Deserialize, Serialize};

const INTERN_PIK_TOPIC: &str = "teamsykmelding.paragraf-i-kode";
const ETTERLEVELSE_TOPIC: &str = "flex.omrade-helse-etterlevelse";

pub fn get_environment_variables() -> EnvironmentVariables {
    let cluster_name: String = env::var("NAIS_CLUSTER_NAME").unwrap_or("environment variable NAIS_CLUSTER_NAME is missing".to_string());
    let application_name: String = env::var("NAIS_APP_NAME").unwrap_or("environment variable NAIS_APP_NAME is missing".to_string());
    let kafka_brokers: String = env::var("KAFKA_BROKERS").unwrap_or("environment variable KAFKA_BROKERS is missing".to_string());
    let kafka_certificate_path: String = env::var("KAFKA_CERTIFICATE_PATH").unwrap_or("environment variable KAFKA_CERTIFICATE_PATH is missing".to_string());
    let kafka_ca_path: String = env::var("KAFKA_CA_PATH").unwrap_or("environment variable KAFKA_CA_PATH is missing".to_string());
    let kafka_private_key_path: String = env::var("KAFKA_PRIVATE_KEY_PATH").unwrap_or("environment variable KAFKA_PRIVATE_KEY_PATH is missing".to_string());
    let hostname: String = env::var("HOSTNAME").unwrap_or("environment variable HOSTNAME is missing".to_string());

    return EnvironmentVariables {
        intern_pik_topic: INTERN_PIK_TOPIC,
        etterlevelse_topic: ETTERLEVELSE_TOPIC.to_string(),
        cluster_name,
        application_name,
        kafka_brokers,
        kafka_certificate_path,
        kafka_ca_path,
        kafka_private_key_path,
        hostname
    };
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentVariables {
    pub(crate) intern_pik_topic: &'static str,
    pub(crate) etterlevelse_topic: String,
    pub(crate) cluster_name: String,
    pub(crate) application_name: String,
    pub(crate) kafka_brokers: String,
    pub(crate) kafka_certificate_path: String,
    pub(crate) kafka_ca_path: String,
    pub(crate) kafka_private_key_path: String,
    pub(crate) hostname: String
}
