use std::collections::HashMap;

pub struct Message {
    protocol: String,
    status: String,
    headers: HashMap<String, String>,
    body: String
}

impl Message {
    pub fn new(protocol: String, status: String, headers: HashMap<String, String>, body: String) -> Message {
        Message {
            protocol,
            status,
            headers,
            body
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("{} {}\r\n", &self.protocol, &self.status);

        if !&self.headers.is_empty() {
            for (key, value) in &self.headers {
                response.push_str(&format!("{}: {}\r\n", &key, &value));
            }
            response.push_str("\r\n");
        }

        if !&self.body.is_empty() {
            response.push_str(&self.body);
        }

        response
    }
}
