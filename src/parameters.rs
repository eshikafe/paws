// Copyright 2021 Austin Aigbe
// Copyright 2021 TVWS-Project

// PAWS Protocol Parameters (RFC 7545)
// All parameter names are case sensitive.

use crate::types::{Float, Int};

use serde::Deserialize;
use serde::Serialize;
use serde::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Point {
    latitude: Float,
    longitude: Float,
}

impl Point {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Point {
            latitude,
            longitude,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Ellipse {
    center: Point,                // REQUIRED

    #[serde(skip_serializing_if = "Option::is_none")]
    semiMajorAxis: Option<Float>, // OPTIONAL

    #[serde(skip_serializing_if = "Option::is_none")]
    semiMinorAxis: Option<Float>, // OPTIONAL

    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Float>,   // OPTIONAL
}

impl Ellipse {
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Ellipse {
            center: Point::new(latitude, longitude),
            semiMajorAxis: None,
            semiMinorAxis: None,
            orientation: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Loc {
    Point(Ellipse),
    Region(Polygon),
}

#[derive(Serialize, Deserialize)]
pub struct GeoLocation {
    #[serde(flatten)]
    loc: Loc,  // point and region are  mutually exclusive

    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<Int>, // OPTIONAL
}

impl GeoLocation {
    fn new(latitude: Float, longitude: Float) -> Self {
        GeoLocation {
            loc: Loc::point(Ellipse::new(latitude, longitude)),
            confidence: None,
        }
    }
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialNumber: Option<String>,   // Optional: PAWS, Required: FCC, ETSI
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturerId: Option<String>, // Optional: PAWS, Required: ETSI
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modelId: Option<String>,        // Optional: PAWS, Required: ETSI
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rulesetIds: Vec<String>,        // Optional

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<HasMap<String, Value>>,
}

impl DeviceDescriptor {
    pub fn new(reg_domain: String) -> Self {
        let regulatory_domains = vec!["NCC", "FCC", "ETSI","PAWS"];

        if reg_domain in regulatory_domains

        DeviceDescriptor {
            serialNumber: Some(String::from("xxxx")),
            manufacturerId: Some(String::from("TVWS project")),
            modelId: None,


        }
    }
}
#[derive(Serialize, Deserialize)]
pub enum HeightType {
    AGL,  // Above Ground Level (default)
    AMSL, // Above Mean Sea Level
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct AntennaCharacteristics<T> {
    height: Float,
    heightType: HeightType,
    heightUncertainty: Float,
    characteristics: Option<T>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FrequencyRange {
    startHz: Float,
    stopHz: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceCapabilities<T> {
    frequencyRanges: Vec<FrequencyRange>,
    other: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct vCard {
    // TODO
// use the vCard crate
}

#[derive(Serialize, Deserialize)]
pub struct DeviceOwner {
    owner: vCard,    // Required
    operator: vCard, // Optional
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RulesetInfo<T> {
    authority: String, // Required
    rulesetId: String, // Required

    #[serde(skip_serializing_if = "Option::is_none")]
    maxLocationChange: Option<Float>, //Required for INIT_RESP, optional otherwise

    #[serde(skip_serializing_if = "Option::is_none")]
    maxPollingSecs: Option<Int>, //Required for INIT_RESP, optional otherwise

    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<T>, // Optional. Depending on the ruleset, other parameters may be required
}

#[derive(Serialize, Deserialize)]
pub struct DbUpdateSpec {
    databases: Vec<DatabaseSpec>,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseSpec {
    name: String, // Required, max length: 64 octets
    uri: String,  // Required, max length: 1024 octets
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpectrumSpec<T> {
    rulesetInfo: RulesetInfo<T>,
    spectrumSchedules: Vec<SpectrumSchedule>,
    timeRange: EventTime,
    frequencyRanges: Vec<FrequencyRange>,
    needsSpectrumReport: bool,
    maxTotalBwHz: Float,
    maxContiguousBwHz: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SpectrumSchedule {
    eventTime: EventTime,
    spectra: Vec<Spectrum>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Spectrum {
    resolutionBwHz: Float,
    profiles: Vec<SpectrumProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct SpectrumProfile {
    list: Vec<SpectrumProfilePoint>,
}

#[derive(Serialize, Deserialize)]
pub struct SpectrumProfilePoint {
    hz: Float,
    dbm: Float,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct EventTime {
    startTime: String,
    stopTime: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GeoSpectrumSpec<T> {
    location: GeoLocation,
    spectrumSpecs: Vec<SpectrumSpec<T>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceValidity<T> {
    deviceDesc: DeviceDescriptor<T>,
    isValid: bool,
    reason: String,
}
