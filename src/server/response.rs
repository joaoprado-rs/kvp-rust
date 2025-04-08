use serde::{Deserialize, Serialize};

pub struct Response {
    content: Option<String>,
    code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Error {
    reason: String,
}

struct Errors {
    errors: Vec<Error>,
}

impl Response {
    pub fn new(content: Option<String>, code: u16) -> Self {
        Response { content, code }
    }

    pub fn format_response(&self) -> String {
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

    pub fn get_status_code(&self) -> &str {
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

impl Error {
    fn new(reason: String) -> Self {
        Error { reason }
    }
    fn to_string(&self) -> String {
        let str = serde_json::to_string(&self);
        str.unwrap()
    }
}

impl Data {
    fn new(message: String) -> Self {
        Data { message }
    }
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}
