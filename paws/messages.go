package paws

// {
//     "jsonrpc": "2.0",
//     "method": "spectrum.paws.init",
//     "params": {
//      "type": "INIT_REQ",
//      "version": "1.0",
//      "deviceDesc": {
//       "serialNumber": "XXX",
//       "fccId": "YYY",
//       "rulesetIds": ["FccTvBandWhiteSpace-2010"]
//      },
//      "location": {
//       "point": {
//        "center": {"latitude": 37.0, "longitude": -101.3}
//       }
//      }
//     },
//     "id": "xxxxxx"
//    }

// PAWS Request
// The JSON-RPC Request for PAWS has the following form:
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

type PAWSRequest struct {
	Jsonrpc string  `json:"jsonrpc"`
	Method  string  `json:"method"`
	Params  InitReq `json:"params"`
	Id      string  `json:"id"`
}

// PAWS Response
// The non-error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }
type PAWSResponse struct {
	Jsonrpc string   `json:"jsonrpc"`
	Result  InitResp `json:"result"`
	Id      string   `json:"id"`
}

// PAWS Error Response
// The error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "error": {
// 	"code": -102,
// 	"message": "An appropriate error message.",
// 	"data": { ... }
//   },
//   "id": "idString"
// }
type PAWSErrorResponse struct {
	Jsonrpc string `json:"jsonrpc"`
	Error   Error  `json:"error"`
	Id      string `json:"id"`
}

// Method Name: spectrum.paws.init                                                       |
//    Request: INIT_REQ                                                        |
//    Response: INIT_RESP

// INIT_REQ message
// Note: Point and region are mutually exclusive.  Exactly one must be present.
type InitReq struct {
	Type       string           `json:"type"`
	Version    string           `json:"version"`
	DeviceDesc DeviceDescriptor `json:"deviceDesc"` // REQUIRED
	Location   GeoLocation      `json:"location"`   // REQUIRED
	//Other Any  // OPTIONAL
}

// INIT_RES message
type InitResp struct {
	Type           string        `json:"type"`
	Version        string        `json:"version"`
	RulesetInfos   []RuleSetInfo `json:"rulesetInfos"`   // REQUIRED
	DatabaseChange DbUpdateSpec  `json:"databaseChange"` // OPTIONAL
	//other: Any,
}

func (msg *PAWSRequest) InitMsg() PAWSRequest {
	msg.Jsonrpc = "2.0"
	msg.Method = "spectrum.paws.init"
	msg.Params.Type = "INIT_REQ"
	msg.Params.Version = "1.0"
	msg.Params.DeviceDesc.SerialNumber = "XXXXX"
	msg.Params.DeviceDesc.ManufacturerId = "YY"
	msg.Params.DeviceDesc.ModelId = "ZZ"
	msg.Params.DeviceDesc.RulesetIds = []string{"NccTvBandWhiteSpace-2010"}
	msg.Params.Location.Point.Center.Latitude = 37.0
	msg.Params.Location.Point.Center.Longitude = -101.3
	return *msg
}
