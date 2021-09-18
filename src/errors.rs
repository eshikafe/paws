// Copyright (c) 2021 TVWS-Project
//
// PAWS Error Codes (RFC 7545 section 5.17)
// If the Database responds to a PAWS request message with an error, it
// MUST include an Error element.

use env_logger;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

use crate::types::Int;
use crate::version::*;

// PAWS errors
pub enum ErrorCode {
    Version,
    Unsupported,
    Unimplemented,
    OutsideCoverage,
    DatabaseChange,
    Missing,
    InvalidValue,
    Unauthorized,
    NotRegistered,
}

impl ErrorCode {
    fn to_int(&self) -> Int {
        match *self {
            ErrorCode::Version => -101,
            ErrorCode::Unsupported => -102,
            ErrorCode::Unimplemented => -103,
            ErrorCode::OutsideCoverage => -104,
            ErrorCode::DatabaseChange => -105,
            ErrorCode::Missing => -201,
            ErrorCode::InvalidValue => -202,
            ErrorCode::Unauthorized => -301,
            ErrorCode::NotRegistered => -302,
        }
    }

    fn to_message(&self) -> String {
        match *self {
            ErrorCode::Version => String::from("Database does not support message version"),
            ErrorCode::Unsupported => String::from("Database does not support the device"),
            ErrorCode::Unimplemented => String::from(
                "The Database does not implement the optional request or optional feature",
            ),
            ErrorCode::OutsideCoverage => {
                String::from("Specified geolocation is outside the coverage area of the Database")
            }
            ErrorCode::DatabaseChange => String::from("Database has changed its URI"),
            ErrorCode::Missing => String::from("A required parameter is missing"),
            ErrorCode::InvalidValue => String::from("A parameter value is invalid in some way"),
            ErrorCode::Unauthorized => String::from("Device is not authorized to use the database"),
            ErrorCode::NotRegistered => {
                String::from("Device registration required, but the device is not registered")
            }
        }
    }
}

// RFC 7545 Section 5.17.  Error Element
#[derive(Serialize, Deserialize)]
pub struct Error {
    code: Int,
    message: String, // Length: 128 octets

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

impl Error {
    fn new(code: ErrorCode) -> Self {
        match code {
            ErrorCode::Version
            | ErrorCode::Unsupported
            | ErrorCode::Unimplemented
            | ErrorCode::InvalidValue
            | ErrorCode::Unauthorized
            | ErrorCode::NotRegistered => Self {
                code: code.to_int(),
                message: code.to_message(),
                data: None,
            },
            ErrorCode::OutsideCoverage => {
                let data = String::from("");
                Self {
                    code: code.to_int(),
                    message: code.to_message(),
                    data: Some(data),
                }
            }
            ErrorCode::Missing => {
                let data = String::from("");
                Self {
                    code: code.to_int(),
                    message: code.to_message(),
                    data: Some(data),
                }
            }
            ErrorCode::DatabaseChange => {
                let data = String::from("DbUpdateSpec");
                Self {
                    code: code.to_int(),
                    message: code.to_message(),
                    data: Some(data),
                }
            }
            ErrorCode::OutsideCoverage => {
                let data = String::from("alternate database");
                Self {
                    code: code.to_int(),
                    message: code.to_message(),
                    data: Some(data),
                }
            }
        }
    }
}

// PAWS Error Response
// The error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "error": {
// 		"code": -102,
// 		"message": "An appropriate error message.",
// 		"data": { ... }
//   },
//   "id": "idString"
// }

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    jsonrpc: String,
    error: Error,
    id: String,
}

impl ErrorResponse {
    pub fn new(err: ErrorCode) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            error: Error::new(err),
            id: String::from("xxxx"),
        }
    }
}
