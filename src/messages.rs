use crate::parameters::*;

// PAWS Request
// The JSON-RPC Request for PAWS has the following form:
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

pub struct Request<T> {
	jsonrpc: String,
	method:  String,
    params: T,
    id: String,
}

// PAWS Response
// The non-error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }

pub struct Response<T> {
	jsonrpc: String,
	result: T,
	id: String,
}

// PAWS Messages
// "spectrum.paws.methodName"
pub struct InitReqMsg<T> {
    device_desc: DeviceDescriptor<T>, // REQUIRED
    location: GeoLocation,         // REQUIRED
    other: Option<T>,              // OPTIONAL
}

pub struct InitRespMsg<T> {
    ruleset_infos: Vec<RuleSetInfo>,
    database_change: DbUpdateSpec,
    other: Option<T>,
}
