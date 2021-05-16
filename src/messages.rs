use crate::parameters::*;

// Messages
pub struct InitReqMsg {
    device_desc: DeviceDescriptor, // REQUIRED
    location: GeoLocation,         // REQUIRED
    other: Any                     // OPTIONAL
}

pub struct InitRespMsg {
    ruleset_infos: Vec<RuleSetInfo>,
    database_change: DbUpdateSpec,
    other: Any,
}
