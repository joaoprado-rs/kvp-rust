pub fn get_route_and_execute(req: Request) -> Option<String> {
    if req.path == "/list" && req.method == "GET" {
        return Some(Response::new(Some("{\"valor\": \"1\"}".to_string()), 200).format_response());
    } else if req.path == "/get/{key}" && req.method == "GET" {
        return Some(Response::new(None, 404).format_response());
    } else if req.path == "/set" && req.method == "POST" {
        return set_pair(req);
    } else if req.path == "/delete/{key}" && req.method == "DELETE" {
        return Some(Response::new(None, 404).format_response());
    } else {
        return Some(Response::new(None, 404).format_response());
    }
}
