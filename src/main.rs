mod handle_client;
mod environment_variables;
mod tcp_listener;
mod avien_kafka;


use serde_derive::{Deserialize, Serialize};
use crate::avien_kafka::avien_kafka;
use crate::environment_variables::get_environment_variables;
use crate::tcp_listener::start_tcp_listener;


fn main() {
    let environment_variables = get_environment_variables();

    let application_state = ApplicationState {
        alive: true,
        ready: true,
    };

    start_tcp_listener(application_state);

    println!("Made it past tcp listeneres");

    avien_kafka(environment_variables)
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}
