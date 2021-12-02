// Copyright 2021 TVWS-Project

package paws

// spectrum.paws.init                                                       |
//    Request: INIT_REQ                                                        |
//    Response: INIT_RESP

// INIT_REQ message
type InitReq struct {
	Type       string           `json:"type"`
	Version    string           `json:"version"`
	DeviceDesc DeviceDescriptor `json:"deviceDesc"` // REQUIRED
	Location   GeoLocation      `json:"location"`   // REQUIRED
	//Other Any  // OPTIONAL
}

// INIT_RESP message
type InitResp struct {
	Type           string        `json:"type"`
	Version        string        `json:"version"`
	RulesetInfos   []RulesetInfo `json:"rulesetInfos"`             // REQUIRED
	DatabaseChange *DbUpdateSpec `json:"databaseChange,omitempty"` // OPTIONAL FIX THIS
	//	other       *any
}

// spectrum.paws.register
//    REGISTRATION_REQ
//    REGISTRATION_RESP

type RegistrationReq struct {
	Type        string                 `json:"type"`
	Version     string                 `json:"version"`
	DeviceDesc  DeviceDescriptor       `json:"deviceDesc"`
	Location    GeoLocation            `json:"location"`
	DeviceOwner DeviceOwner            `json:"deviceOwner,omitempty"`
	Antenna     AntennaCharacteristics `json:"antenna,omitempty"`
	//	other       *any
}

type RegistrationResp struct {
	Type           string        `json:"type"`
	Version        string        `json:"version"`
	RulesetInfos   []RulesetInfo `json:"rulesetInfos"`
	DatabaseChange DbUpdateSpec  `json:"databaseChange,omitempty"`
}

// spectrum.paws.getSpectrum
//    AVAIL_SPECTRUM_REQ
//    AVAIL_SPECTRUM_RESP

type AvailSpectrumReq struct {
	Type                 string                 `json:"type"`
	Version              string                 `json:"version"`
	DeviceDesc           DeviceDescriptor       `json:"deviceDesc"`
	Location             GeoLocation            `json:"location"`
	Owner                DeviceOwner            `json:"owner"`
	Antenna              AntennaCharacteristics `json:"antenna,omitempty"`
	Capabilities         DeviceCapabilities     `json:"capabilities,omitempty"`
	MsterDeviceDesc      DeviceDescriptor       `json:"masterDeviceDesc,omitempty"`
	MasterDeviceLocation GeoLocation            `json:"masterDeviceLocation,omitempty"`
	RequestType          string                 `json:"requestType,omitempty"`
	// other *any
}

type AvailSpectrumResp struct {
	Type           string           `json:"type"` // AVAIL_SPECTRUM_RESP
	Version        string           `json:"version"`
	Timestamp      string           `json:"timestamp"` // UTC YYYY-MM-DDThh:mm:ssZ
	DeviceDesc     DeviceDescriptor `json:"deviceDesc"`
	SpectrumSpecs  []SpectrumSpec   `json:"spectrumSpecs"`
	DatabaseChange DbUpdateSpec     `json:"databaseChange,omitempty"`
	// other *any
}

type AvailSpectrumBatchReq struct {
	Type       string           `json:"type"` // AVAIL_SPECTRUM_BATCH_REQ
	Version    string           `json:"version"`
	DeviceDesc DeviceDescriptor `json:"deviceDesc"`
	Locations  []GeoLocation    `json:"locations"`
	Owner      DeviceOwner      `json:"owner,omitempty"`
}
