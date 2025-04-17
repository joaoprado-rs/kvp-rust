use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use librarius::{error, info};

use super::{handler::get_route_and_execute, request::Request};
use crate::server::{response::Response, state::State};

pub struct Server {
    host: String,
    port: String,
    pub kvp: Arc<Mutex<State>>,
}

impl Server {
    pub fn new(host: &str, port: &str) -> Self {
        Server {
            host: host.to_string(),
            port: port.to_string(),
            kvp: Arc::new(Mutex::new(State::new())),
        }
    }

    pub fn run(&self) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&address).unwrap();
        info!("Server listening on {}", address);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    info!("Connection stablished successfully");
                    Self::handle_connection(stream, Arc::clone(&self.kvp));
                }
                Err(err) => {
                    info!(
                        "Error while opening connection. The error is: '{}'",
                        err.to_string()
                    )
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<State>>) {
        let mut buffer = [0; 1024];
        let mut request = String::new();

        match stream.read(&mut buffer) {
            Ok(size) => {
                request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
                let parsed_request = match Request::build_request(&request) {
                    Some(req) => req,
                    None => {
                        info!("An error has occurred while constructing the request...");
                        let response = Response::new(None, 500).format_response();
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let response = match get_route_and_execute(parsed_request, state) {
                    Some(res) => res,
                    None => {
                        error!("An error has occurred while executing the request...");
                        let response = Response::new(None, 500).format_response();
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                error!(
                    "Error allocating memory for the stream. The error is: '{}'",
                    err.to_string()
                );
            }
        }
    }
}
