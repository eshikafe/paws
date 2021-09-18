// Copyright 2021 TVWS-Project

package paws

// PAWS Request
// The JSON-RPC Request for PAWS has the following form:
//    {
//      "jsonrpc": "2.0",
//      "method": "spectrum.paws.methodName",
//      "params": <PAWS_REQ>,
//      "id": "idString"
//    }

type PAWSHandler struct {
	request  Request
	response Response
}

type Request struct {
	Jsonrpc string `json:"jsonrpc"`
	Method  string `json:"method"`
	//ApiVersion string  `json:"apiVersion"` // "v1beta"
	Params InitReq `json:"params"`
	Id     string  `json:"id"`
}

// PAWS Response
// The non-error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "result": <PAWS_RESP>,
//   "id": "idString"
// }
type Response struct {
	Jsonrpc string   `json:"jsonrpc"`
	Result  InitResp `json:"result"`
	Id      string   `json:"id"`
}

// Method Name: spectrum.paws.init                                                       |
//    Request: INIT_REQ                                                        |
//    Response: INIT_RESP

// INIT_REQ message
type InitReq struct {
	Type       string           `json:"type"`
	Version    string           `json:"version"`
	DeviceDesc DeviceDescriptor `json:"deviceDesc"`         // REQUIRED
	Location   GeoLocation      `json:"location,omitempty"` // REQUIRED
	//Other Any  // OPTIONAL
}

// INIT_RESP message
type InitResp struct {
	Type           string        `json:"type"`
	Version        string        `json:"version"`
	RulesetInfos   []RulesetInfo `json:"rulesetInfos"`             // REQUIRED
	DatabaseChange *DbUpdateSpec `json:"databaseChange,omitempty"` // OPTIONAL FIX THIS
	//other: Any,
}

func (res *InitReq) init() InitReq {
	res.Type = "INIT_REQ"
	res.Version = PAWSVersion
	res.DeviceDesc.SerialNumber = "XXX"
	res.DeviceDesc.ManufacturerId = "YYY"
	res.DeviceDesc.ModelId = "ZZ"
	res.DeviceDesc.RulesetIds = []string{"NccTvBandWhiteSpace-2010"}

	// TODO: get Lat/Long from GPS module or node config
	res.Location.Point.Center.Latitude = 6.5
	res.Location.Point.Center.Longitude = 3.35
	return *res
}

func (msg *Request) Init() Request {
	var init_req InitReq
	msg.Jsonrpc = JSONVersion
	msg.Method = "spectrum.paws.init"
	msg.Params = init_req.init()
	msg.Id = "xxxxxx"
	return *msg
}

func (msg *Response) Init() Response {
	msg.Jsonrpc = JSONVersion
	msg.Result.Type = "INIT_RESP"
	msg.Result.Version = PAWSVersion
	msg.Result.RulesetInfos = []RulesetInfo{{
		Authority:         "ng",
		RulesetId:         "NccTvBandWhiteSpace-2010",
		MaxLocationChange: 100,
		MaxPollingSecs:    86400}}
	msg.Id = "xxxxxx"
	return *msg
}
