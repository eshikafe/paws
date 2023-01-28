use crate::parameters::*;
use crate::types::*;
use crate::version::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
//use uuid::Uuid;

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
    // pub result: ResponseParams,
    pub id: String,
}

impl<T> Response<T> {
    /// Create a new PAWS response.
    ///
    /// ```rust
    /// use paws::Response;
    /// use paws::{InitResp, RegistrationResp};
    /// Response::new(InitResp::new());
    /// ```
    pub fn new(result: T) -> Self {
        Response {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            result,
            // result: ResponseParams::Init(InitResp::new(ruleset_id)),
            id: String::from("xxxxxx"),
        }
    }
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

impl InitResp {
    pub fn new(ruleset_id: String) -> Self {
        Self {
            mtype: RequestType::InitResp.to_string(),
            version: PAWS_VERSION.to_string(),

            // TODO: Use rule_set_id in DeviceDescriptor to determine RulesetInfo
            ruleset_infos: vec![RulesetInfo::new(ruleset_id)],
            database_change: None,
            other: None,
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
        let ruleset_infos = String::from("NccTVWS-2019");
        Self {
            mtype: RequestType::RegistrationResp.to_string(),
            version: PAWS_VERSION.to_string(),
            ruleset_infos: vec![RulesetInfo::new(ruleset_infos)],
            database_change: None,
            other: None
        }
    }
}

// TODO:
// Implement AvailSpectrumResp
// Implement AvailSpectrumBatchResp
// Implement SpectrumUseResp
// Implement DevValidResp