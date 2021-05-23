package paws

// PAWS Request
// The JSON-RPC Request for PAWS has the following form:
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

type Request struct {
	Jsonrpc string     `json:"jsonrpc"`
	Method  string     `json:"method"`
	Params  InitReqMsg `json:"params"`
	Id      string     `json:"id"`
}

// PAWS Response
// The non-error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }
type Response struct {
	Jsonrpc string      `json:"jsonrpc"`
	Result  InitRespMsg `json:"result"`
	Id      string      `json:"id"`
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
type ErrorResponse struct {
	Jsonrpc string `json:"jsonrpc"`
	Error   Error  `json:"error"`
	Id      string `json:"id"`
}

// Method Name: spectrum.paws.init                                                       |
//    Request: INIT_REQ                                                        |
//    Response: INIT_RESP

// INIT_REQ message
type InitReqMsg struct {
	Type       string           `json:"type"`
	Version    string           `json:"version"`
	DeviceDesc DeviceDescriptor `json:"deviceDesc"` // REQUIRED
	Location   GeoLocation      `json:"location"`   // REQUIRED
	//Other Any  // OPTIONAL
}

// INIT_RES message
type InitRespMsg struct {
	Type           string        `json:"type"`
	Version        string        `json:"version"`
	RulesetInfos   []RuleSetInfo `json:"rulesetInfos"`   // REQUIRED
	DatabaseChange DbUpdateSpec  `json:"databaseChange"` // OPTIONAL
	//other: Any,
}

func (msg *Request) InitMsg() Request {
	msg.Jsonrpc = "2.0"
	msg.Method = "spectrum.paws.init"
	msg.Params.Type = "INIT_REQ"
	msg.Params.Version = "1.0"
	msg.Params.DeviceDesc.SerialNumber = "XXX"
	msg.Params.DeviceDesc.ManufacturerId = "YYY"
	msg.Params.DeviceDesc.ModelId = "ZZ"
	msg.Params.DeviceDesc.RulesetIds = []string{"NccTvBandWhiteSpace-2010"}
	msg.Params.Location.Point.Center.Latitude = 6.5000
	msg.Params.Location.Point.Center.Longitude = 3.3500
	msg.Id = "xxxxxx"
	return *msg
}
