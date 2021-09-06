// (c) 2021 TVWS-Project


use crate::parameters::*;
//use crate::errors::*;
use crate::version::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// PAWS method
// pub struct Method<T, U> {
//    pub name: String,
//    pub request: Request<T>,
//    pub response: Response<U>,
// }


// impl<T, U> Method<T, U>{
    
//     pub fn init(&mut self) -> Self {
//         // Method Name: spectrum.paws.init
//         //  Request: INIT_REQ
//         //  Response: INIT_RESP
//         Self {
//             name: String::from("init"),
//             request: Request<InitReq>,
//             response: Response<InitResp>,
//         }
        

//     }

    // pub fn register(&mut self) -> Result<Response, ErrorResponse> {

    // }

    // pub fn get_spectrum(&mut self) -> Result<Response, ErrorResponse> {

    // }

    // pub fn get_spectrum_batch(&mut self) -> Result<Response, ErrorResponse> {

    // }

    // pub fn notify_Spectrum_use(&mut self) -> Result<Response, ErrorResponse> {

    // }

    // pub fn verify_device(&mut self) -> Result<Response, ErrorResponse> {
        
    // }
// }

// PAWS Request
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

#[derive(Serialize, Deserialize)]
pub struct Request<T> {
    pub jsonrpc: String, 
    pub method: String,
    pub params: T,
    pub id: String,
}


impl<T> Request<T> {
    pub fn new(name: &str) -> Self {
        match name {
            "init" => Self {
                jsonrpc: String::from("2.0"),
                method: String::from("spectrum.paws.init"),
                params: T::new(),
                id: String::from("xxxxxx")
            }
        }
        
    }
}

// PAWS Response
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: String,
}

impl<T> Response<T> {
    pub fn new(name: &str) -> Self {
        match name {
            "init" => Self {
                    jsonrpc: String::from("2.0"),
                    result: T::new(),
                    id: String::from("xxxxxx"),
                }
        }
    }
}

// INIT_REQ
#[derive(Serialize, Deserialize)]
pub struct InitReq {
    #[serde(rename = "type")]
    pub mtype: String,
    pub version: String,

    #[serde(rename = "deviceDesc")]
    pub device_desc: DeviceDescriptor, // REQUIRED
    pub location: GeoLocation,         // REQUIRED

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>,
}

impl InitReq {
    pub fn new() -> Self{
        Self {
            mtype: String::from("INIT_REQ"),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("NCC"),
            location: GeoLocation::new(6.8269, 3.6228),
            other: None
        };
        // let s = serde_json::to_string_pretty(&init_req_msg).unwrap();
        // return s;
    }
}

// INIT_RESP
#[derive(Serialize, Deserialize)]
pub struct InitResp {
    #[serde(rename = "type")]
   pub mtype: String,
   pub version: String,

    #[serde(rename = "rulesetInfos")]
   pub ruleset_infos: Vec<RulesetInfo>, // REQUIRED for INIT_RESP

    #[serde(rename = "databaseChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
   pub database_change: Option<DbUpdateSpec>,   // OPTIONAL

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
   pub other: Option<HashMap<String, Value>>,  // OPTIONAL
}

impl InitResp {
    pub fn new() -> Self {
        Self {
            mtype: String::from("INIT_REQ"),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new()],
            database_change: None,
            other: None,
        }
    }
}