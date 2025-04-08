struct Response {
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
