mod handle_client;

use std::net::{TcpListener};
use serde_derive::{Deserialize, Serialize};
use crate::handle_client::handle_client;

fn main() {
    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");
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

//Model: Application State struct
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationState {
    alive: bool,
    ready: bool,
}