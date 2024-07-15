use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct StatusCode(pub u16);

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_code = format!("STATUS CODE: {}", self.status_code.0);
        let mut headers = String::new();
        for (k, v) in &self.headers {
            headers.push_str(format!("{}: {}", k, v).as_str());
            headers.push('\n');
        }
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };
        write!(f, "{}\n\n{}\n\n{}", status_code, headers, body)
    }
}
