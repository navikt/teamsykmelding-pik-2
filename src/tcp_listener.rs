use std::net::TcpListener;
use crate::ApplicationState;
use crate::handle_client::handle_client;

pub fn start_tcp_listener(application_state: ApplicationState) {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Starting lister on port 8080");

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, application_state);
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }
}
