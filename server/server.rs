use crate::state::State;

struct Server {
    host: String,
    port: String,
    kvp: Arc<Mutex<State>>
}

impl Server {
    fn new(host: &str, port: &str) -> Self {
        Server {
            host.to_string(),
            port.to_string(),
            kvp: Arc::new(Mutex::new(State::new())),
        }
    }

    fn run(&self) {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(address).unwrap();
        println!("Server listening on {}", address);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connection stablished successfully");
                    self.handle_connection(stream);
                }
                Err(err) => {
                    println!(
                        "Error while opening connection. The error is: '{}'",
                        err.to_string()
                    )
                }
            }
        }

    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let mut request = String::new();
    
        match stream.read(&mut buffer) {
            Ok(size) => {
                request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());
                let parsed_request = match build_request(&request) {
                    Some(req) => req,
                    None => {
                        println!("An error has occurred while constructing the request...");
                        let response = Response::new(None, 500).format_response();
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        return;
                    }
                };
                let response = match get_route_and_execute(parsed_request) {
                    Some(res) => res,
                    None => {
                        println!("An error has occurred while executing the request...");
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
                println!(
                    "Error allocating memory for the stream. The error is: '{}'",
                    err.to_string()
                );
            }
        }
    }
}