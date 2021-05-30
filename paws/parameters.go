// copyright 2021 Austin Aigbe
package paws

// PAWS Protocol Parameters (RFC 7545)
// All parameter names are case sensitive.

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

// #[derive(Serialize, Deserialize)]
// type struct Any;

type HeightType enum {
    AGL,  // Above Ground Level (default)
    AMSL, // Above Mean Sea Level
}


type AntennaCharacteristics struct {
    height float
    heightType HeightType,
    heightUncertainty float
    characteristics Various
}

type FrequencyRange struct {
	StartHz float `json:"startHz"`
	StopHz  float `json:"stopHz"`
}

type DeviceCapabilities struct {
	frequencyRanges []FrequencyRange `json:"frequencyRanges"`
	other           any
}

// #[derive(Serialize, Deserialize)]
// type struct vCard {
//     // TODO
// // use the vCard crate
// }

// #[derive(Serialize, Deserialize)]
// type struct DeviceOwner {
//     owner: vCard,    // Required
//     operator: vCard, // Optional
// }

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


type SpectrumSpec struct{
    rulesetInfo RuleSetInfo
    spectrumSchedules []SpectrumSchedule
    timeRange EventTime
    frequencyRanges []FrequencyRange
    needsSpectrumReport bool
    maxTotalBwHz float
    maxContiguousBwHz float
}


type SpectrumSchedule struct{
    eventTime EventTime
    spectra []Spectrum
}

type Spectrum struct{
    resolutionBwHz float
    profiles []SpectrumProfile
}

type SpectrumProfile struct{
    list []SpectrumProfilePoint
}

type SpectrumProfilePoint struct{
    hz float
    dbm float
}

type EventTime struct{
    startTime string
    stopTime string
}


type GeoSpectrumSpec struct{
    location GeoLocation
    spectrumSpecs []SpectrumSpec
}

type struct DeviceValidity {
    deviceDesc DeviceDescriptor
    isValid bool
    reason string
}
