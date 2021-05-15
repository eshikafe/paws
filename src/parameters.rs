// PAWS Protocol Parameters

use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Point {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Ellipse {
    center: Point,
    semiMajorAxis: f32,
    semiMinorAxis: f32,
    orientation: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Polygon {
    exterior: Vec<Point>
}

#[derive(Serialize, Deserialize)]
pub struct GeoLocation {
    point: Ellipse,
    region: Polygon,
    confidence: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceDescriptor {
    serialNumber: String,  // Optional. Max length: 64 octets
    manufacturerId: String,
    modelId: String,
    rulesetIds: Vec<String>,
    other: Any,
}

#[derive(Serialize, Deserialize)]
pub struct Any;

#[derive(Serialize, Deserialize)]
pub enum HeightType {
    AGL,     // Above Ground Level (default)
    AMSL,    // Above Mean Sea Level
}

#[derive(Serialize, Deserialize)]
pub struct AntennaCharacteristics {
    height: f32,
    heightType: HeightType,
    heightUncertainty: f32,
    characteristics: Various,
}

#[derive(Serialize, Deserialize)]
pub struct FrequencyRange {
    startHz: f32,
    stopHz: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceCapabilities {
    frequencyRanges: Vec<FrequencyRange>,
    other: Any,
}

#[derive(Serialize, Deserialize)]
pub struct vCard {

}

pub struct DeviceOwner {
    owner: vCard,     // Required
    operator: vCard,  // Optional
}


pub struct RuleSetInfo {
    authority: String,
    ruleset_id: String,
    max_location_change: f32,
    max_polling_secs: f32,
    other: Any,
}



