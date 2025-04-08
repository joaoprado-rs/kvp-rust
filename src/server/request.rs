pub struct Request {
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

    pub fn build_request(request_string: &String) -> Option<Request> {
        let mut lines = request_strin.lines();
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
                    "Skipping line because does not match the pattern: '{}'",
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
}
