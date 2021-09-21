// Copyright 2021 TVWS-Project

use crate::parameters::*;
use crate::types::*;
use crate::version::*;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// PAWS methods
enum Method {
    Init,
    Register,
    GetSpectrum,
    GetSpectrumBatch,
    NotifySpectrumUse,
    VerifyDevice,
}

impl Method {
    fn to_string(&self) -> String {
        match *self {
            Method::Init => "spectrum.paws.init".to_string(),
            Method::Register => "spectrum.paws.register".to_string(),
            Method::GetSpectrum => "spectrum.paws.getSpectrum".to_string(),
            Method::GetSpectrumBatch => "spectrum.paws.getSpectrumBatch".to_string(),
            Method::NotifySpectrumUse => "spectrum.paws.notifySpectrumUse".to_string(),
            Method::VerifyDevice => "spectrum.paws.verifyDevice".to_string(),
        }
    }
}

enum Message {
    InitReq,
    InitResp,
}

impl Message {
    fn to_string(&self) -> String {
        match *self {
            Message::InitReq => "INIT_REQ".to_string(),
            Message::InitResp => "INIT_RESP".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestParams {
    Init(InitReq),
    // Register(RegistrationReq),
    // GetSpectrum(AvailSpectrumReq),
    // AvailSpectrumBatchReq,
    // SpectrumUseNotify,
    // DevValidReq,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseParams {
    Init(InitResp),
    // Register(RegistrationResp),
    // GetSpectrum(AvailSpectrumResp),
    // AvailSpectrumBatchResp,
    // SpectrumUseResp,
    // DevValidResp,
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
            mtype: Message::InitReq.to_string(),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("ncc"),
            location: GeoLocation::new(6.8269, 3.6228),
            other: None,
        }
    }

    pub fn location(&self) -> (Float, Float) {
        match self.location.loc {
            Loc::Point(Ellipse {
                center:
                    Point {
                        latitude,
                        longitude,
                    },
                ..
            }) => (latitude, longitude),
            _ => (0.0, 0.0),
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: RequestParams,
    pub id: String,
}

impl Request {
    // Creates a new PAWS Request
    pub fn new() -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method: Method::Init.to_string(),
            params: RequestParams::Init(InitReq::new()),
            id: String::from("xxx"),
        }
    }

    // Revisit this implementation
    pub fn ruleset(&self) -> String {
        match &self.params {
            RequestParams::Init(InitReq {
                device_desc: DeviceDescriptor { rulesetIds, .. },
                ..
            }) => {
                if let Some(r) = rulesetIds {
                    return r[0].clone();
                } else {
                    String::from("")
                }
            }
            _ => String::from(""),
        }
    }

    pub fn location(&self) -> (Float, Float) {
        match &self.params {
            RequestParams::Init(InitReq) => InitReq.location(),
            _ => (0.0, 0.0),
        }
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
    pub result: ResponseParams,
    pub id: String,
}

impl Response {
    pub fn new(rulesetId: String) -> Response {
        Response {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result: ResponseParams::Init(InitResp::new(rulesetId)),
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
            mtype: Message::InitResp.to_string(),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new(rulesetId)],
            database_change: None,
            other: None,
        }
    }
}
