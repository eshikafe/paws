// (c) 2021 TVWS-Project

// PAWS Protocol Parameters (RFC 7545)
// All parameter names are case sensitive.

use crate::types::{Float, Int};

use mac_address::get_mac_address;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub latitude: Float,
    pub longitude: Float,
}

impl Point {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ellipse {
    pub center: Point, // REQUIRED

    #[serde(skip_serializing_if = "Option::is_none")]
    pub semiMajorAxis: Option<Float>, // OPTIONAL

    #[serde(skip_serializing_if = "Option::is_none")]
    pub semiMinorAxis: Option<Float>, // OPTIONAL

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Float>, // OPTIONAL
}

impl Ellipse {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            center: Point::new(latitude, longitude),
            semiMajorAxis: None,
            semiMinorAxis: None,
            orientation: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Polygon {
    exterior: Vec<Point>, // REQUIRED
}

impl Polygon {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Polygon {
            exterior: vec![Point::new(latitude, longitude)],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Loc {
    Point(Ellipse),
    Region(Polygon),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocation {
    #[serde(flatten)]
    pub loc: Loc, // point and region are  mutually exclusive

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Int>, // OPTIONAL
}

impl GeoLocation {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        GeoLocation {
            loc: Loc::Point(Ellipse::new(latitude, longitude)),
            confidence: None,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialNumber: Option<String>, // Optional: PAWS, Required: FCC, ETSI

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturerId: Option<String>, // Optional: PAWS, Required: ETSI

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modelId: Option<String>, // Optional: PAWS, Required: ETSI

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesetIds: Option<Vec<String>>, // Optional

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>,
}

impl DeviceDescriptor {
    pub fn new(regulator: &str) -> Self {
        //let regulatory_domains = vec!["ncc", "fcc", "etsi", "paws", "ofcom"];
        let mut other_params = HashMap::new();
        let mut rule_set_id = String::new();

        match regulator {
            "ncc" => {
                other_params.insert(String::from("NccId"), json!("YYY"));
                rule_set_id = String::from("NccTvBandWhiteSpace-2019");
            }
            "fcc" => {
                other_params.insert(String::from("FccId"), json!("YYY"));
                other_params.insert(String::from("fccTvbdDeviceType"), json!("FIXED"));
                rule_set_id = String::from("FccTvBandWhiteSpace-2010");
            }
            _ => {
                other_params.insert(String::from("etsiEnDeviceType"), json!("A"));
                other_params.insert(String::from("etsiEnDeviceEmissionsClass"), json!("1"));
                other_params.insert(
                    String::from("etsiEnTechnologyId"),
                    json!("ETSI-EN-301-598-2.1.1-2018-01"),
                );
                other_params.insert(String::from("etsiEnDeviceCategory"), json!("master"));
                rule_set_id = String::from("ETSI-EN-301-598-2.1.1");
            }
        };

        Self {
            serialNumber: Some(get_mac_addr()),
            manufacturerId: Some(String::from("tvws-project-team")),
            modelId: Some(String::from("model-sw-beta")),
            rulesetIds: Some(vec![rule_set_id]),
            other: Some(other_params),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum HeightType {
    AGL,  // Above Ground Level (default)
    AMSL, // Above Mean Sea Level
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AntennaCharacteristics {
    height: Float,
    heightType: HeightType, // default: AGL
    heightUncertainty: Float,

    #[serde(skip_serializing_if = "Option::is_none")]
    characteristics: Option<HashMap<String, Value>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FrequencyRange {
    startHz: Float,
    stopHz: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceCapabilities {
    frequencyRanges: Vec<FrequencyRange>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct vCard {
    // TODO
// use the vCard crate
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceOwner {
    owner: vCard, // Required

    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<vCard>, // Optional
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RulesetInfo {
    pub authority: String, // Required
    pub rulesetId: String, // Required

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxLocationChange: Option<Float>, //Required for INIT_RESP, optional otherwise

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxPollingSecs: Option<Int>, //Required for INIT_RESP, optional otherwise

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HashMap<String, Value>>, // Optional. Depending on the ruleset, other parameters may be required
}

impl RulesetInfo {
    pub fn new(ruleset_id: String) -> Self {

       if ruleset_id == String::from("NccTvBandWhiteSpace-2019") {
           Self {
                // TODO: Use rule_set_id in DeviceDescriptor to determine rulesetId and authority
                authority: String::from("ng"),
                rulesetId: String::from("NccTvBandWhiteSpace-2019"),
                maxLocationChange: Some(100.0),
                maxPollingSecs: Some(86400),
                other: None,
            }
        } else {
            Self {
                // TODO: Use rule_set_id in DeviceDescriptor to determine rulesetId and authority
                authority: String::from("ng"),
                rulesetId: String::from("NccTvBandWhiteSpace-2019"),
                maxLocationChange: Some(100.0),
                maxPollingSecs: Some(86400),
                other: None,
            }
        }

    }
        
 }

#[derive(Serialize, Deserialize, Debug)]
pub struct DbUpdateSpec {
    databases: Vec<DatabaseSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseSpec {
    name: String, // Required, max length: 64 octets
    uri: String,  // Required, max length: 1024 octets
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SpectrumSpec {
    rulesetInfo: RulesetInfo,
    spectrumSchedules: Vec<SpectrumSchedule>,
    timeRange: EventTime,
    frequencyRanges: Vec<FrequencyRange>,
    needsSpectrumReport: bool,
    maxTotalBwHz: Float,
    maxContiguousBwHz: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct SpectrumSchedule {
    eventTime: EventTime,
    spectra: Vec<Spectrum>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Spectrum {
    resolutionBwHz: Float,
    profiles: Vec<SpectrumProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpectrumProfile {
    list: Vec<SpectrumProfilePoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpectrumProfilePoint {
    hz: Float,
    dbm: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EventTime {
    startTime: String,
    stopTime: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoSpectrumSpec {
    location: GeoLocation,
    spectrumSpecs: Vec<SpectrumSpec>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceValidity {
    deviceDesc: DeviceDescriptor,
    isValid: bool,
    reason: String,
}

// Common functions

pub fn get_mac_addr() -> String {
    let mut mac_addr = String::new();
    match get_mac_address() {
        Ok(Some(ma)) => {
            let b = ma.bytes();
            let _result = match write!(
                mac_addr,
                "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                b[0], b[1], b[2], b[3], b[4], b[5]
            ) {
                Ok(_) => return mac_addr,
                _ => return String::from(""),
            };
        }
        _ => String::from(""),
    }
}
