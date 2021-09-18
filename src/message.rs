// Copyright 2021 TVWS-Project

use crate::parameters::*;
use crate::version::*;
use crate::method::*;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

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
            device_desc: DeviceDescriptor::new("ncc"),
            location: GeoLocation::new(6.8269, 3.6228),
            other: None,
        }
    }
}


// PAWS Request
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

//  A PAWS request message is carried in the body of an HTTP POST request
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: InitReq,
    pub id: String,
}

impl Request {
    // Creates a new PAWS Request
    // 
    // # Example
    // 
    // ```
    // # use paws::{Request, Method};
    // let init_req = InitReq::new()
    // let req = Request::new(init_req);
    // assert_eq!(req.method(), "spectrum.paws.init".to_string());
    // ```
    // 
    pub fn new() -> Self {

         Self {
                jsonrpc: JSON_RPC_VERSION.to_string(),
                method: String::from("spectrum.paws.init"),
                params: InitReq::new(),
                id: String::from("xxx"),
            }
    }

    // Revisit this implementation
     pub fn ruleset(&self) -> String {
        let ruleset_id = match &self.params.device_desc.rulesetIds {
            Some(r) => r[0].clone(),
            None => String::from("")
        };
        return ruleset_id;
    }
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
pub struct Response {
    pub jsonrpc: String,
    pub result: InitResp,
    pub id: String,
}

impl Response {
    pub fn new(rulesetId: String) -> Response {
         Response {
                jsonrpc: JSON_RPC_VERSION.to_string(),
                result: InitResp::new(rulesetId),
                id: String::from("xxxxxx"),
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
    pub fn new(rulesetId: String) -> Self {
        Self {
            mtype: String::from("INIT_RESP"),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new(rulesetId)],
            database_change: None,
            other: None,
        }
    }
}
