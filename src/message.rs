// Copyright 2021 TVWS-Project


use crate::parameters::*;
use crate::types::*;
use crate::version::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
//use uuid::Uuid;

// PAWS methods
pub enum Method {
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

pub enum Message {
    InitReq,
    InitResp,
    RegistrationReq,
    RegistrationResp,
}

impl Message {
    fn to_string(&self) -> String {
        match *self {
            Message::InitReq => "INIT_REQ".to_string(),
            Message::InitResp => "INIT_RESP".to_string(),
            Message::RegistrationReq => "REGISTRATION_REQ".to_string(),
            Message::RegistrationResp => "REGISTRATION_RESP".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestParams {
    Init(InitReq),
    Register(RegistrationReq),
    // GetSpectrum(AvailSpectrumReq),
    // AvailSpectrumBatchReq,
    // SpectrumUseNotify,
    // DevValidReq,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseParams {
    Init(InitResp),
    Register(RegistrationResp),
    // GetSpectrum(AvailSpectrumResp),
    // AvailSpectrumBatchResp,
    // SpectrumUseResp,
    // DevValidResp,
}

// INIT_REQ message
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

// INIT_RESP message
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

// REGISTRATION_REQ message
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationReq {
    #[serde(rename = "type")]
    pub mtype: String,
    pub version: String,

    #[serde(rename = "deviceDesc")]
    pub device_desc: DeviceDescriptor, // REQUIRED
    pub location: GeoLocation, // REQUIRED

    #[serde(rename = "deviceOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_owner: Option<DeviceOwner>, // OPTIONAL

    #[serde(skip_serializing_if = "Option::is_none")]
    pub antenna: Option<AntennaCharacteristics>, // OPTIONAL

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>, // OPTIONAL
}

impl RegistrationReq {
    pub fn new() -> Self {
        Self {
            mtype: Message::RegistrationReq.to_string(),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("ncc"),
            location: GeoLocation::new(6.45, 3.75),
            device_owner: None,
            antenna: None,
            other: None
        }
    }
}

// REGISTRATION_RESP message
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationResp {
    #[serde(rename = "type")]
    pub mtype: String,
    pub version: String,

    #[serde(rename = "rulesetInfos")]
    pub ruleset_infos: Vec<RulesetInfo>, // REQUIRED

    #[serde(rename = "databaseChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_change: Option<DbUpdateSpec>, // OPTIONAL

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>, // OPTIONAL
}

impl RegistrationResp {
    pub fn new() -> Self {
        // Fix this later. rulesetinfos should be retrieved from the DB, not hardcoded
        let ruleset_infos = String::from("NccTvBandWhiteSpace-2019");
        Self {
            mtype: Message::RegistrationResp.to_string(),
            version: PAWS_VERSION.to_string(),
            ruleset_infos: vec![RulesetInfo::new(ruleset_infos)],
            database_change: None,
            other: None
        }
    }
}

impl InitReq {
    pub fn new() -> Self {
        Self {
            mtype: Message::InitReq.to_string(),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("ncc"),

            // TODO: Get location from GPS module or from configuration file
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

    pub fn mtype(&self) -> String {
        self.mtype.clone()
    }
}

impl InitResp {
    pub fn new(ruleset_id: String) -> Self {
        Self {
            mtype: Message::InitResp.to_string(),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new(ruleset_id)],
            database_change: None,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: RequestParams,
    pub id: String,
}

impl Request {
    // Creates a new PAWS Request
    pub fn new(method: Method) -> Self {

       // let id = Uuid::new_v4()?;
        match method {
            Method::Init => Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method: Method::Init.to_string(),
            params: RequestParams::Init(InitReq::new()),
            id: String::from("xxx"),
        },
        Method::Register => Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method: Method::Register.to_string(),
            params: RequestParams::Register(RegistrationReq::new()),
            id: String::from("xxx"),
        },
        _ => Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            method: Method::Init.to_string(),
            params: RequestParams::Init(InitReq::new()),
            id: String::from("xxx"),
        }
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

    pub fn method(&self) -> Option<Method> {
        match self.method.as_str() {
            "spectrum.paws.init" => Some(Method::Init),
            "spectrum.paws.register" => Some(Method::Register),
            _ => None
        }
    }

    pub fn mtype(&self) -> String {
        match &self.params {
            RequestParams::Init(init_req) => init_req.mtype(),
            _ => "".to_string(),
        }
    }

    pub fn location(&self) -> (Float, Float) {
        match &self.params {
            RequestParams::Init(init_req) => init_req.location(),
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
    pub fn new(ruleset_id: String) -> Response {
        Response {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result: ResponseParams::Init(InitResp::new(ruleset_id)),
            id: String::from("xxxxxx"),
        }
    }
}

