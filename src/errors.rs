// PAWS Error Codes (RFC 7545 section 5.17)
// If the Database responds to a PAWS request message with an error, it
// MUST include an Error element.

use std::fmt;
use serde::Deserialize;
use serde::Serialize;
use log::error;
use env_logger;

use crate::types::Int;

#[derive(Serialize, Deserialize)]
pub struct Error {
    code: Int,        // range -32768 to 32767
    message: String,  // Length: 128 octets
    data: String,
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
pub struct ErrorResponse{
	jsonrpc: String,
	error: Error,
	id: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            -101 => "[VERSION] Database does not support the specified version of the message",
            -102 => "[UNSUPPORTED] Database does not support the device",
            -103 => "[UNIMPLEMENTED] Database does not implement the optional request or optional feature",
            -104 => "[OUTSIDE_COVERAGE] Specified geolocation is outside the coverage area of the Database",
            -105 => "[DATABASE_CHANGE] Database has changed its URI",
            -201 => "[MISSING]  Required parameter is missing",
            -202 => "[INVALID_VALUE] A parameter value is invalid in some way",
            -301 => "[UNAUTHORIZED] The device is not authorized to use the Database.",
            -302 => "NOT_REGISTERED Device registration required, but the device is not registered",
            _ => "[Unknown]",

        };

        write!(f, "{}", err_msg)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}