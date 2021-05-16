// PAWS Protocol Parameters (RFC 7545)
// All parameter names are case sensitive.
use crate::types::{Float, Int};

use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct Point {
    latitude: Float,
    longitude: Float,
}

#[derive(Serialize, Deserialize)]
pub struct Ellipse {
    center: Point,
    semiMajorAxis: Float,
    semiMinorAxis: Float,
    orientation: Float,
}

#[derive(Serialize, Deserialize)]
pub struct Polygon {
    exterior: Vec<Point>,
}

#[derive(Serialize, Deserialize)]
pub struct GeoLocation {
    point: Ellipse,
    region: Polygon,
    confidence: Int,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceDescriptor {
    serialNumber: String, // Optional. Max length: 64 octets
    manufacturerId: String,
    modelId: String,
    rulesetIds: Vec<String>,
    other: Any,
}

#[derive(Serialize, Deserialize)]
pub struct Any;

#[derive(Serialize, Deserialize)]
pub enum HeightType {
    AGL,  // Above Ground Level (default)
    AMSL, // Above Mean Sea Level
}

#[derive(Serialize, Deserialize)]
pub struct AntennaCharacteristics {
    height: Float,
    heightType: HeightType,
    heightUncertainty: Float,
    characteristics: Various,
}

#[derive(Serialize, Deserialize)]
pub struct FrequencyRange {
    startHz: Float,
    stopHz: Float,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceCapabilities {
    frequencyRanges: Vec<FrequencyRange>,
    other: Any,
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

#[derive(Serialize, Deserialize)]
pub struct RuleSetInfo {
    authority: String,
    rulesetId: String,
    maxLocationChange: Float,
    maxPollingSecs: Float,
    other: Any,
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

#[derive(Serialize, Deserialize)]
pub struct SpectrumSpec {
    rulesetInfo: RuleSetInfo,
    spectrumSchedules: Vec<SpectrumSchedule>,
    timeRange: EventTime,
    frequencyRanges: Vec<FrequencyRange>,
    needsSpectrumReport: bool,
    maxTotalBwHz: Float,
    maxContiguousBwHz: Float,
}

#[derive(Serialize, Deserialize)]
pub struct SpectrumSchedule {
    eventTime: EventTime,
    spectra: Vec<Spectrum>,
}

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

#[derive(Serialize, Deserialize)]
pub struct EventTime {
    startTime: String,
    stopTime: String,
}

#[derive(Serialize, Deserialize)]
pub struct GeoSpectrumSpec {
    location: GeoLocation,
    spectrumSpecs: Vec<SpectrumSpec>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceValidity {
    deviceDesc: DeviceDescriptor,
    isValid: bool,
    reason: String,
}
