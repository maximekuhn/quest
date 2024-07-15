use thiserror::Error;

use crate::request::{self, HttpRequest};

const SECTIONS_SEP: &str = "###";

#[derive(Debug, Error)]
pub enum HttpFileParserError {
    #[error("failed to parse request: '{err}'")]
    FailedToParseRequest {
        #[from]
        err: request::ParseError,
    },
}

pub fn parse_http_file(content: String) -> Result<Vec<HttpRequest>, HttpFileParserError> {
    let raw_sections: Vec<_> = content.split(SECTIONS_SEP).collect();

    // remove SECTIONS_SEP of each sections
    let mut sections: Vec<String> = Vec::with_capacity(raw_sections.len());
    for section in raw_sections {
        if section.is_empty() {
            continue;
        }
        let mut section_lines = section.lines();
        section_lines.next();
        let section_content = section_lines.collect();
        sections.push(section_content);
    }

    // try to parse HTTP request(s)
    let mut http_requests = Vec::with_capacity(sections.len());
    for section in sections {
        let request = section.parse()?;
        http_requests.push(request);
    }

    Ok(http_requests)
}
