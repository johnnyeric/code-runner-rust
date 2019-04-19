extern crate serde_json;

use types::Payload;
use serde_json::Result;
use std::result;
use std::io::{Error, ErrorKind};
use languages::is_supported;

pub fn parse_payload(json: &str) -> result::Result<Payload, Error> {
    let result: Result<Payload> = serde_json::from_str(&json);

    match result {
        Ok(payload) => {
            if payload.files.len() == 0 {
                return Err(Error::new(ErrorKind::InvalidInput, "No files were given\n"));
            }

            if !is_supported(&payload.language) {
                return Err(Error::new(ErrorKind::InvalidInput, format!("Language '{}' is not supported\n", payload.language)));
            }
            
            Ok(payload)
        },
        Err(error) => {
            return Err(Error::new(ErrorKind::InvalidInput, format!("Failed to parse input JSON ({:?})\n", error)));
        }
    }
}