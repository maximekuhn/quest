use thiserror::Error;

use crate::parser;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {err}")]
    IoError {
        #[from]
        err: std::io::Error,
    },

    #[error("Parse error: {err}")]
    ParseError {
        #[from]
        err: parser::HttpFileParserError,
    },
}
