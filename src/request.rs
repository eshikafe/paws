use crate::parameters::*;
use crate::types::*;
use crate::version::*;
use crate::method::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// use uuid::Uuid;

// PAWS Request
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }
#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: String,
}

impl<T> Request<T> {
    /// Create a new PAWS request.
    ///
    /// ```rust
    /// use paws::Request;
    /// use paws::{InitReq, RegistrationReq};
    
    /// Request::new(Method::Init, InitReq::new());
    /// Request::new(Method::Register, RegistrationReq::new());
    /// ```
    pub fn new(method: Method, params: T) -> Self {
        // let id = Uuid::new_v4()?;
        Self {
                    jsonrpc: JSON_RPC_VERSION.to_string(),
                    method: method.to_string(),
                    params,
                    id: String::from("xxx"),
                }
    }

    /// Get a reference to the message
    pub fn get_ref(&self) -> &T {
        &self.params
    }

    /// Get a mutable reference to the message
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.params
    }

    // TODO
    // Revisit this implementation
    pub fn ruleset(&self) -> String {
        let req_type = self.get_ref();

        if let Some(r) = req_type.device_desc.rulesetIds {
            return r[0].clone();
        } else {
            String::from("")
        }
        
    }

    pub fn method(&self) -> Option<Method> {
        match self.method.as_str() {
            "spectrum.paws.init" => Some(Method::Init),
            "spectrum.paws.register" => Some(Method::Register),
            "spectrum.paws.getSpectrum" => Some(Method::GetSpectrum),
            "spectrum.paws.getSpectrumBatch" => Some(Method::GetSpectrumBatch),
            "spectrum.paws.notifySpectrumUse" => Some(Method::NotifySpectrumUse),
            "spectrum.paws.verifyDevice" => Some(Method::VerifyDevice),
            _ => None
        }
    }

    // pub fn mtype(&self) -> String {
    //     match &self.params {
    //         RequestParams::Init(init_req) => init_req.mtype(),
    //         _ => "".to_string(),
    //     }
    // }

    // pub fn location(&self) -> (Float, Float) {
    //     match &self.params {
    //         RequestParams::Init(init_req) => init_req.location(),
    //         _ => (0.0, 0.0),
    //     }
    // }
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

impl InitReq {
    pub fn new() -> Self {
        Self {
            mtype: RequestType::InitReq.to_string(),
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
            mtype: RequestType::RegistrationReq.to_string(),
            version: PAWS_VERSION.to_string(),
            device_desc: DeviceDescriptor::new("ncc"),
            location: GeoLocation::new(6.45, 3.75),
            device_owner: None,
            antenna: None,
            other: None
        }
    }
}

// TODO
// Implement AvailSpectrumReq
// Implement AvailSpectrumBatchReq
// Implement SpectrumUseNotify
// Implement DevValidReq