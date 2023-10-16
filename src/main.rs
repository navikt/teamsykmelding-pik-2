mod environment_variables;
mod avien_kafka;
mod nais_api;
mod log;
mod prometheus;


use ::log::info;
use serde_derive::{Deserialize, Serialize};
use crate::avien_kafka::avien_kafka;
use crate::environment_variables::get_environment_variables;
use crate::nais_api::register_nais_api;
use tokio::task;
use crate::log::init_log4rs;

#[tokio::main]
async fn main() {
    init_log4rs();

    let environment_variables = get_environment_variables();

    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    task::spawn(register_nais_api(application_state));
    info!("Server has started");

    avien_kafka(environment_variables);
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}
