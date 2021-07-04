// Copyright 2021 Austin Aigbe
// Copyright 2021 TVWS-Project

use crate::parameters::*;
use crate::errors::*;
use crate::version::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// PAWS method
pub struct Method {
    name: String,
    request: Request,
    response: Response,
}



impl Method{
    // Method Name: spectrum.paws.init
    //  Request: INIT_REQ
    //  Response: INIT_RESP
    pub fn init(&mut self) -> Result<Response, ErrorResponse> {
        let req = Request::new("init");
        let res = Response::new("init");
        Ok(res)

    }

    pub fn register(&mut self) -> Result<Response, ErrorResponse> {

    }

    pub fn get_spectrum(&mut self) -> Result<Response, ErrorResponse> {

    }

    pub fn get_spectrum_batch(&mut self) -> Result<Response, ErrorResponse> {

    }

    pub fn notify_Spectrum_use(&mut self) -> Result<Response, ErrorResponse> {

    }

    pub fn verify_device(&mut self) -> Result<Response, ErrorResponse> {
        
    }
}

// PAWS Request JSON-RPC format:
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

pub struct Request {
    jsonrpc: String, 
    method: String,  // "spectrum.paws.<methodName>"
    params: String,  // PAWS Paramaters
    id: String,
}

impl Request {
    pub fn new(name: &str) -> Self {
        match name {
            "init" => Self {
                jsonrpc: String::from("2.0"),
                method: String::from("spectrum.paws.init"),
                params: InitReq::new(),                                  // INIT_REQ
                id: String::from("xxxxxx")
            }
        }
        
    }
}

// PAWS Response
// The non-error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }

pub struct Response {
    jsonrpc: String,
    result: String,
    id: String,
}

impl Response {
    pub fn new(name: &str) -> Self {
        match name {
            "init" => Self {
                jsonrpc: String::from("2.0"),
                result: 
                id: String::from("xxxxxx"),

            }
        }

    }
}

// INIT_REQ
#[derive(Serialize, Deserialize)]
pub struct InitReq {
    #[serde(rename = "type")]
    mtype: String,
    version: String,

    #[serde(rename = "deviceDesc")]
    device_desc: DeviceDescriptor, // REQUIRED
    location: GeoLocation,         // REQUIRED

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<HashMap<String, Value>>,
}

impl InitReq {
    fn new() -> String{
        let req_type = "INIT_REQ";
        let version = PAWS_VERSION;
        let init_req_msg = InitReq {
            mtype: String::from("INIT_REQ"),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("FCC"),
            location: GeoLocation::new(),
            other: None
        };
        let s = serde_json::to_string_pretty(&init_req_msg).unwrap();
        return s;
    }
}

// INIT_RESP
#[derive(Serialize, Deserialize)]
pub struct InitResp {
    #[serde(rename = "type")]
    mtype: String,
    version: String,

    #[serde(rename = "rulesetInfos")]
    ruleset_infos: Vec<RuleSetInfo<T>>, // REQUIRED for INIT_RESP

    #[serde(rename = "databaseChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    database_change: Option<DbUpdateSpec>,   // OPTIONAL

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<HashMap<String, Value>>,  // OPTIONAL
}
