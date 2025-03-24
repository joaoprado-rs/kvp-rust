use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Lines, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct Request {
    path: String,
    method: String,
    header: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    fn new(
        path: String,
        method: String,
        header: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Request {
            path,
            method,
            header,
            body,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection stablished successfully");
                handle_connection(stream);
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
            let parsed_request = build_request(&request).or_else(|| {
                println!("An error has occurred while processing the request...");
                return;
            });
        }
        Err(err) => {
            println!(
                "Error allocating memory for the stream. The error is: '{}'",
                err.to_string()
            );
        }
    }
}

fn build_request(request_string: &String) -> Option<Request> {
    let mut lines = request_string.lines();
    let first = lines.next()?;
    let (method, route) = build_method_route(first)?;

    None
}

fn build_method_route(first: &str) -> Option<(String, String)> {
    let vec_line: Vec<&str> = first.split_whitespace().collect();
    let method = vec_line.get(0).map(|s| s.to_string())?;
    let route = vec_line.get(1).map(|s| s.to_string())?;
    Some((method, route))
}

fn build_headers()
