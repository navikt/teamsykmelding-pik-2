use std::io::{Read, Write};
use std::net::{TcpStream};
use crate::ApplicationState;


// http constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

pub fn handle_client(mut stream: TcpStream, application_state: ApplicationState) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /internal/is_alive") => handle_get_is_alive_request(application_state),
                r if r.starts_with("GET /internal/is_ready") => handle_get_is_ready_request(application_state),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}



fn handle_get_is_alive_request(application_state: ApplicationState) -> (String, String) {
    match application_state.alive {
        true => {
            (OK_RESPONSE.to_string(), "I'm alive! :)".to_string())
        },
        _ => (INTERNAL_ERROR.to_string(), "I'm dead x_x".to_string())
    }
}

fn handle_get_is_ready_request(application_state: ApplicationState) -> (String, String) {
    match application_state.ready {
        true => {
            (OK_RESPONSE.to_string(), "I'm ready! :)".to_string())
        },
        _ => (INTERNAL_ERROR.to_string(), "Please wait! I'm not ready :(".to_string())
    }
}


