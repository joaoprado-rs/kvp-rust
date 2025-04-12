use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    message: String,
    success: bool,
    error: Option<Error>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>, // novo campo opcional
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    message: String,
    reason: String,
}

impl Error {
    pub fn new(reason: String, message: String) -> Self {
        Error { reason, message }
    }
}

impl Data {
    pub fn new(
        message: String,
        success: bool,
        error: Option<Error>,
        data: Option<serde_json::Value>,
    ) -> Self {
        Data {
            message,
            success,
            error,
            data,
        }
    }
    fn to_string(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
}

pub struct Response {
    content: Option<String>,
    code: u16,
}

impl Response {
    pub fn new(content: Option<String>, code: u16) -> Self {
        Response { content, code }
    }

    pub fn new_from_data(data: Option<Data>, code: u16) -> Self {
        let content = data.map(|d| d.to_string());
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

            409 => "409 Conflict",

            500 => "500 Internal Server Error",

            _ => "500 Internal Server Error",
        }
    }
}
