// PAWS Protocol Parameters (RFC 7545)
// All parameter names are case sensitive.

use crate::types::{Float, Int};

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Point {
    latitude: Float,
    longitude: Float,
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceDescriptor<T> {
    serialNumber: String, // Optional. Max length: 64 octets
    manufacturerId: String,
    modelId: String,
    rulesetIds: Vec<String>,
    other: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Any;

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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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
pub struct GeoSpectrumSpec {
    location: GeoLocation,
    spectrumSpecs: Vec<SpectrumSpec>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct DeviceValidity<T> {
    deviceDesc: DeviceDescriptor<T>,
    isValid: bool,
    reason: String,
}
