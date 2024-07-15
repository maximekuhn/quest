use std::{collections::HashMap, str::FromStr};

use thiserror::Error;
use url::Url;

pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            _ => Err(format!("unknown HTTP method: '{s}'")),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("empty input")]
    Empty,

    #[error("invalid format: '{reason}'")]
    InvalidFormat { reason: String },

    #[error("invalid HTTP method: '{reason}'")]
    InvalidMethod { reason: String },

    #[error("invalid URL: '{reason}'")]
    InvalidUrl { reason: String },
}

impl FromStr for HttpRequest {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let mut lines_iter = lines.iter();

        // first line: [METHOD] [URI]
        let Some(first_line) = lines_iter.next() else {
            return Err(ParseError::Empty);
        };
        let (method, url) = parse_first_line(first_line)?;

        Ok(Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
        })
    }
}

fn parse_first_line(line: &str) -> Result<(Method, Url), ParseError> {
    let Some((method, url)) = line.split_once(' ') else {
        return Err(ParseError::InvalidFormat {
            reason: "first line should be [METHOD] [URI]".into(),
        });
    };

    let method = method
        .parse()
        .map_err(|err| ParseError::InvalidMethod { reason: err })?;

    let url = Url::from_str(url).map_err(|err| ParseError::InvalidUrl {
        reason: err.to_string(),
    })?;

    Ok((method, url))
}
