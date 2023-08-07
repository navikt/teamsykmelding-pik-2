use std::net::TcpListener;
use crate::ApplicationState;
use crate::handle_client::handle_client;

pub fn start_tcp_listener(application_state: ApplicationState) {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();

    for stream in tcp_listener.incoming() {
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
