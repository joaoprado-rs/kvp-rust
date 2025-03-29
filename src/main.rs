use std::{
    collections::HashMap,
    hash::Hash,
    io::{BufRead, BufReader, Lines, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct Response {
    content: Option<String>,
    code: u16,
}

#[derive(Debug)]
struct Request {
    path: String,
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    fn new(
        path: String,
        method: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Request {
            path,
            method,
            headers,
            body,
        }
    }
}

impl Response {
    fn new(content: Option<String>, code: u16) -> Self {
        Response { content, code }
    }
    fn format_response(&self) -> String {
        let body = self.content.as_deref().unwrap_or("");
        let status_code = self.get_status_code();
        let str = format!(
            "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            body.len(),
            body
        );
        str
    }

    fn get_status_code(&self) -> &str {
        match self.code {
            200 => "200 OK",
            201 => "201 Created",
            400 => "400 Bad Request",
            404 => "404 Not Found",
            500 => "500 Internal Server Error",
            _ => "500 Internal Server Error",
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

fn build_request(request_string: &String) -> Option<Request> {
    let mut lines = request_string.lines();
    let first = lines.next()?;
    let (method, path) = build_method_route(first)?;
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let parts = line.split_once(":");
        if let Some((mut key, mut value)) = parts {
            key = key.trim();
            value = value.trim();
            headers.insert(key.to_string(), value.to_string());
        } else {
            println!(
                "Skiping line because does not match the pattern: '{}'",
                line.to_string()
            );
        }
    }

    let body = if headers
        .get("Content-Length")
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|&size| size > 0)
        .is_some()
        && headers
            .get("Content-Type")
            .filter(|content| content.as_str() == "application/json")
            .is_some()
    {
        let body_lines: Vec<&str> = lines.collect();
        if !body_lines.is_empty() {
            Some(body_lines.join(""))
        } else {
            None
        }
    } else {
        None
    };

    Some(Request::new(path, method, headers, body))
}

fn build_method_route(first: &str) -> Option<(String, String)> {
    let vec_line: Vec<&str> = first.split_whitespace().collect();
    let method = vec_line.get(0).map(|s| s.to_string())?;
    let route = vec_line.get(1).map(|s| s.to_string())?;
    Some((method, route))
}

fn get_route_and_execute(req: Request) -> Option<String> {
    if req.path == "/list" && req.method == "GET" {
        return Some(Response::new(None, 404).format_response());
    } else if req.path == "/get/{key}" && req.method == "GET" {
        return Some(Response::new(None, 404).format_response());
    } else if req.path == "/set" && req.method == "POST" {
        return Some(Response::new(None, 404).format_response());
    } else if req.path == "/delete/{key}" && req.method == "DELETE" {
        return Some(Response::new(None, 404).format_response());
    } else {
        return Some(Response::new(None, 404).format_response());
    }
}
