mod handle_client;
mod environment_variables;

use std::net::{TcpListener};
use std::string::ToString;
use serde_derive::{Deserialize, Serialize};
use crate::environment_variables::get_environment_variables;
use crate::handle_client::handle_client;



fn main() {
    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    let environment_variables = get_environment_variables();

    println!("cluster_name is : {}", environment_variables.cluster_name.to_string());


    let application_state = ApplicationState {
        alive: true,
        ready: true
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, application_state);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentVariables {
    intern_pik_topic: String,
    etterlevelse_topic: String,
    cluster_name: String
}