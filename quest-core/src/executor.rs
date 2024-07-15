use std::str::FromStr;

use reqwest::{
    blocking::Body,
    header::{HeaderName, HeaderValue},
};

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

#[allow(clippy::from_over_into)]
impl Into<reqwest::blocking::Request> for HttpRequest {
    fn into(self) -> reqwest::blocking::Request {
        let method = self.method.into();
        let mut request = reqwest::blocking::Request::new(method, self.url);
        for (header_name, header_value) in self.headers {
            request.headers_mut().append(
                HeaderName::from_str(&header_name).unwrap(),
                HeaderValue::from_str(&header_value).unwrap(),
            );
        }
        if let Some(body) = self.body {
            *request.body_mut() = Some(Body::from(body));
        }
        request
    }
}

#[allow(clippy::from_over_into)]
impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        reqwest::Method::from_str(self.to_string().to_uppercase().as_str()).expect("valid method")
    }
}

#[allow(clippy::from_over_into)]
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

#[allow(clippy::from_over_into)]
impl From<reqwest::StatusCode> for StatusCode {
    fn from(code: reqwest::StatusCode) -> Self {
        Self(code.as_u16())
    }
}
