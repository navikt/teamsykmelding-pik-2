use std::env;
use crate::EnvironmentVariables;

const INTERN_PIK_TOPIC: &str = "teamsykmelding.paragraf-i-kode";
const ETTERLEVELSE_TOPIC: &str = "flex.omrade-helse-etterlevelse";

pub fn get_environment_variables()  -> EnvironmentVariables {
    let cluster_name: String = env::var("NAIS_CLUSTER_NAME").unwrap_or("localhost".to_string());

    return EnvironmentVariables {
        intern_pik_topic: INTERN_PIK_TOPIC.to_string(),
        etterlevelse_topic: ETTERLEVELSE_TOPIC.to_string(),
        cluster_name,
    };
}
