use std::{
    collections::hash_map::Entry,
    sync::{Arc, Mutex},
};

use super::{
    request::Request,
    response::{Data, Error, Response},
    schema::KeyValue,
    state::State,
};
/**
 * Testing
 */
pub fn get_route_and_execute(req: Request, state: Arc<Mutex<State>>) -> Option<String> {
    if req.path == "/lsist" && req.method == "GET" {
        return list_kvp(state);
    } else if req.path.contains("/get/") && req.method == "GET" {
        return get_single_kvp(req, state);
    } else if req.path == "/set" && req.method == "POST" {
        return set_kvp(req, state);
    } else if req.path == "/delete/{key}" && req.method == "DELETE" {
        return Some(Response::new(None, 404).format_response());
    } else {
        return Some(Response::new(None, 404).format_response());
    }
}

fn list_kvp(state: Arc<Mutex<State>>) -> Option<String> {
    let state_guard = state.lock().ok()?;
    let mut list_items: Vec<KeyValue> = Vec::new();
    state_guard.kvp.iter().for_each(|it| {
        list_items.push(KeyValue {
            key: it.0.clone(),
            value: it.1.clone(),
        });
    });
    let serialized = serde_json::to_value(&list_items).ok()?;
    Some(
        Response::new_from_data(
            Some(Data::new(String::new(), true, None, Some(serialized))),
            200,
        )
        .format_response(),
    )
}
fn set_kvp(request: Request, state: Arc<Mutex<State>>) -> Option<String> {
    match serde_json::from_str::<KeyValue>(request.body.unwrap().as_str()) {
        Ok(parsed_body) => {
            let key = parsed_body.key.clone();
            let value = &parsed_body.value.clone();
            let mut state_guard = state.lock().ok()?;

            match state_guard.kvp.entry(parsed_body.key) {
                Entry::Vacant(entry) => {
                    entry.insert(parsed_body.value);
                    let message =
                        format!("'{}' => '{}' register inserted successfully.", key, value);
                    Some(
                        Response::new_from_data(Some(Data::new(message, true, None, None)), 201)
                            .format_response(),
                    )
                }
                Entry::Occupied(_) => {
                    let message =
                        format!("The {} item is already stored. Use the update route.", key);
                    Some(
                        Response::new_from_data(
                            Some(Data::new(
                                "".to_string(),
                                false,
                                Some(Error::new("Item already exist.".to_string(), message)),
                                None,
                            )),
                            409,
                        )
                        .format_response(),
                    )
                }
            }
        }
        Err(err) => {
            let reason = "The request is not valid.";
            let err_msg = format!(
                "Something went wrong while parsing the request. Error: '{}'",
                err.to_string()
            );
            return Some(
                Response::new_from_data(
                    Some(Data::new(
                        "".to_string(),
                        false,
                        Some(Error::new(reason.to_string(), err_msg)),
                        None,
                    )),
                    400,
                )
                .format_response(),
            );
        }
    }
}
fn get_single_kvp(request: Request, state: Arc<Mutex<State>>) -> Option<String> {
    if let Some(param) = request.param {
        let state_guard = state.lock().ok()?;

        if let Some(item) = state_guard.kvp.iter().find(|item| item.0 == &param) {
            let kvp = KeyValue {
                key: item.0.clone(),
                value: item.1.clone(),
            };
            let serialized = serde_json::to_value(&kvp).ok()?;
            Some(
                Response::new_from_data(
                    Some(Data::new(String::new(), true, None, Some(serialized))),
                    200,
                )
                .format_response(),
            )
        } else {
            return Some(
                Response::new_from_data(
                    Some(Data::new(
                        "".to_string(),
                        false,
                        Some(Error::new(
                            String::from("Item not found"),
                            String::from("The item requested was not found."),
                        )),
                        None,
                    )),
                    200,
                )
                .format_response(),
            );
        }
    } else {
        return Some(
            Response::new_from_data(
                Some(Data::new(
                    "".to_string(),
                    false,
                    Some(Error::new(
                        String::from("Invalid route."),
                        String::from(
                            "You must insert the key in the parameter to retrieve it. /get/{key}",
                        ),
                    )),
                    None,
                )),
                400,
            )
            .format_response(),
        );
    }
}
