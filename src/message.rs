// Copyright 2021 TVWS-Project

use crate::parameters::*;
use crate::version::*;
use crate::method::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;


// PAWS Request
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

//  A PAWS request message is carried in the body of an HTTP POST request
#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String,
}

impl<T> Request<T> {
    // Creates a new PAWS Request
    // 
    // # Examples
    // 
    // ```
    // # use paws::{Request, Method};
    // let init_req = InitReq::new()
    // let req = Request::new(init_req);
    // assert_eq!(req.method(), "spectrum.paws.init".to_string());
    // ```
    // 
    pub fn new(param: T) -> Request<T> {
        let method = String::new();
        // if param.mtype == String::from("INIT_REQ") {
        //     method = String::from("spectrum.paws.init");
        // } else if param.mtype == String::from("REGISTRATION_REQ") {
        //    method = String::from("spectrum.paws.register");
        // } else if param.mtype == String::from("AVAIL_SPECTRUM_BATCH_REQ") {
        //     method = String::from("spectrum.paws.getSpectrum");
        // } else if param.mtype == String::from("SPECTRUM_USE_NOTIFY") {
        //     method = String::from("spectrum.paws.notifySpectrumUse");
        // } else if param.mtype == String::from("DEV_VALID_REQ") {
        //     method = String::from("spectrum.paws.verifyDevice")
        // } else {
        //     method = String::from("");
        // }

         Request {
                jsonrpc: String::from("2.0"),
                method: method,
                params: param,
                id: String::from("xxx"),
            }
    }
    // Creates a builder object to produce a `Request`
    // pub fn method(&self)  -> String {
    //     self.method
    // }
}

// PAWS Response
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }

// A PAWS response message is carried in the body of an HTTP response.
// A PAWS response SHOULD include a Content-Length header.

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: String,
}

impl<T> Response<T> {
    pub fn build(result: T) -> Response<T> {
         Response {
                jsonrpc: String::from("2.0"),
                result: result,
                id: String::from("xxxxxx"),
            }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitReq {
    #[serde(rename = "type")]
    pub mtype: String,
    pub version: String,

    #[serde(rename = "deviceDesc")]
    pub device_desc: DeviceDescriptor, // REQUIRED
    pub location: GeoLocation, // REQUIRED

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>,
}

impl InitReq {
    pub fn new() -> Self {
        Self {
            mtype: String::from("INIT_REQ"),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("NCC"),
            location: GeoLocation::new(6.8269, 3.6228),
            other: None,
        }
    }
}

// PAWS `INIT_RESP` message
#[derive(Serialize, Deserialize, Debug)]
pub struct InitResp {
    #[serde(rename = "type")]
    pub mtype: String,
    pub version: String,

    #[serde(rename = "rulesetInfos")]
    pub ruleset_infos: Vec<RulesetInfo>, // REQUIRED for INIT_RESP

    #[serde(rename = "databaseChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_change: Option<DbUpdateSpec>, // OPTIONAL

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>, // OPTIONAL
}

impl InitResp {
    pub fn new() -> Self {
        Self {
            mtype: String::from("INIT_RESP"),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new()],
            database_change: None,
            other: None,
        }
    }
}
