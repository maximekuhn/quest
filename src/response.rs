use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct StatusCode(pub u16);
