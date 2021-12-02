// Copyright (c) 2021 TVWS-Project

package paws

const JSONVersion string = "2.0"
const PAWSVersion string = "1.0"

type float float64

type Point struct {
	Latitude  float `json:"latitude"`  //REQUIRED
	Longitude float `json:"longitude"` // REQUIRED
}

type Ellipse struct {
	Center        Point `json:"center"`                  //REQUIRED
	SemiMajorAxis float `json:"semiMajorAxis,omitempty"` // OPTIONAL
	SemiMinorAxis float `json:"semiMinorAxis,omitempty"` // OPTIONAL
	Orientation   float `json:"orientation,omitempty"`   // OPTIONAL
}

type Polygon struct {
	Exterior []Point `json:"exterior,omitempty"` // REQUIRED
}

// Note: Point and region are mutually exclusive.  Exactly one must be present.
type GeoLocation struct {
	Point      Ellipse  `json:"point"`
	Region     *Polygon `json:"region,omitempty"`     // FIX THIS
	Confidence int      `json:"confidence,omitempty"` // OPTIONAL
}

type DeviceDescriptor struct {
	SerialNumber   string   `json:"serialNumber,omitempty"`   // Optional
	ManufacturerId string   `json:"manufacturerId,omitempty"` // Optional
	ModelId        string   `json:"modelId,omitempty"`        // Optional
	RulesetIds     []string `json:"rulesetIds,omitempty"`     // Optional
	// Other Any
}

// type struct Any;

type HeightType int

const (
	AGL  HeightType = iota // Above Ground Level (default)
	AMSL                   // Above Mean Sea Level
)

type AntennaCharacteristics struct {
	Height            float      `json:"height"`
	HeightType        HeightType `json:"heightType"`
	HeightUncertainty float      `json:"heightUncertainty"`
	//Characteristics   Various    `json:"characteristics"`
}

type FrequencyRange struct {
	StartHz float `json:"startHz"`
	StopHz  float `json:"stopHz"`
}

type DeviceCapabilities struct {
	FrequencyRanges []FrequencyRange `json:"frequencyRanges"`
	//Other           any
}

type vCard struct {
	// TODO
	// check for available vcard golang package
}

type DeviceOwner struct {
	Owner    vCard `json:"owner"`    // Required
	Operator vCard `json:"operator"` // Optional
}

// Rulesets
// 	NCC
//  FCC
//  ETSI
//  ICASA Ruleset (ICASA-TVWS-2018).

type RulesetInfo struct {
	Authority         string `json:"authority"`
	RulesetId         string `json:"rulesetId"`
	MaxLocationChange float  `json:"maxLocationChange"`
	MaxPollingSecs    float  `json:"maxPollingSecs"`
	//Other: Any,
}

type DbUpdateSpec struct {
	Databases []DatabaseSpec `json:"databases,omitempty"`
}

type DatabaseSpec struct {
	Name string `json:"name"` // Required, max length: 64 octets
	Uri  string `json:"uri"`  // Required, max length: 1024 octets
}

type SpectrumSpec struct {
	RulesetInfo         RulesetInfo        `json:"rulesetInfo"`
	SpectrumSchedules   []SpectrumSchedule `json:"spectrumSchedules"`
	TimeRange           EventTime          `json:"timeRange"`
	FrequencyRanges     []FrequencyRange   `json:"frequencyRanges"`
	NeedsSpectrumReport bool               `json:"needsSpectrumReport"`
	MaxTotalBwHz        float              `json:"maxTotalBwHz"`
	MaxContiguousBwHz   float              `json:"maxContiguousBwHz"`
}

type SpectrumSchedule struct {
	EventTime EventTime  `json:"eventTime"`
	Spectra   []Spectrum `json:"spectra"`
}

type Spectrum struct {
	ResolutionBwHz float             `json:"resolutionBwHz"`
	Profiles       []SpectrumProfile `json:"profiles"`
}

type SpectrumProfile struct {
	List []SpectrumProfilePoint `json:"list"` //Check
}

type SpectrumProfilePoint struct {
	Hz  float `json:"hz"`
	Dbm float `json:"dbm"`
}

type EventTime struct {
	StartTime string `json:"startTime"`
	StopTime  string `json:"stopTime"`
}

type GeoSpectrumSpec struct {
	Location      GeoLocation    `json:"location"`
	SpectrumSpecs []SpectrumSpec `json:"spectrumSpecs"`
}

type DeviceValidity struct {
	DeviceDesc DeviceDescriptor `json:"deviceDesc"`
	IsValid    bool             `json:"isValid"`
	Reason     string           `json:"reason"`
}
