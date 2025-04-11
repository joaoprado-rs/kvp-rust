use std::{
    collections::hash_map::Entry,
    fmt::format,
    sync::{Arc, Mutex},
};

use super::{request::Request, response::Response, schema::KeyValue, state::State};

pub fn get_route_and_execute(req: Request, state: Arc<Mutex<State>>) -> Option<String> {
    if req.path == "/list" && req.method == "GET" {
        return Some(Response::new(Some("{\"valor\": \"1\"}".to_string()), 200).format_response());
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

fn set_kvp(request: Request, state: Arc<Mutex<State>>) -> Option<Response> {
    match serde_json::from_str::<KeyValue>(request.body.unwrap().as_str()) {
        Ok(parsed_body) => {
            let key = parsed_body.key.clone();
            let value = &parsed_body.value.clone();
            let mut state_guard = state.lock().ok()?;

            match state_guard.kvp.entry(parsed_body.key) {
                Entry::Vacant(entry) => {
                    entry.insert(parsed_body.value);
                    let message = format!("{}->{} register inserted successfully.", key, value);
                    Some(Response::new(Some(message.into()), 201))
                }
                Entry::Occupied(_) => Some(Response::new(Some("Key already exists".into()), 409)),
            }
        }
        Err(err) => {
            let err_msg = "";
            return Some(Response::new(None, 404));
        }
    }
}
