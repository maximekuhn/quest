use std::str::FromStr;

use crate::{
    request::{HttpRequest, Method},
    response::{HttpResponse, StatusCode},
};

pub struct Executor {
    client: reqwest::blocking::Client,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(
        &self,
        request: HttpRequest,
    ) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        let response = self.client.execute(request.into())?;
        Ok(response.into())
    }
}

impl Into<reqwest::blocking::Request> for HttpRequest {
    fn into(self) -> reqwest::blocking::Request {
        let method = self.method.into();
        reqwest::blocking::Request::new(method, self.url)
    }
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        reqwest::Method::from_str(self.to_string().as_str()).expect("valid method")
    }
}

impl From<reqwest::blocking::Response> for HttpResponse {
    fn from(response: reqwest::blocking::Response) -> Self {
        Self {
            status_code: response.status().into(),
            headers: response
                .headers()
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect(),
            body: Some(String::from_utf8(response.bytes().unwrap().to_vec()).unwrap()),
        }
    }
}

impl From<reqwest::StatusCode> for StatusCode {
    fn from(code: reqwest::StatusCode) -> Self {
        Self(code.as_u16())
    }
}
