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

// #[derive(Serialize, Deserialize)]
// type enum HeightType {
//     AGL,  // Above Ground Level (default)
//     AMSL, // Above Mean Sea Level
// }

// #[derive(Serialize, Deserialize)]
// type struct AntennaCharacteristics {
//     height float,
//     heightType: HeightType,
//     heightUncertainty float,
//     characteristics: Various,
// }

// #[derive(Serialize, Deserialize)]
// type struct FrequencyRange {
//     startHz float,
//     stopHz float,
// }

// #[derive(Serialize, Deserialize)]
// type struct DeviceCapabilities {
//     frequencyRanges: Vec<FrequencyRange>,
//     other: Any,
// }

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

// #[derive(Serialize, Deserialize)]
// type struct SpectrumSpec {
//     rulesetInfo: RuleSetInfo,
//     spectrumSchedules: Vec<SpectrumSchedule>,
//     timeRange: EventTime,
//     frequencyRanges: Vec<FrequencyRange>,
//     needsSpectrumReport: bool,
//     maxTotalBwHz float,
//     maxContiguousBwHz float,
// }

// #[derive(Serialize, Deserialize)]
// type struct SpectrumSchedule {
//     eventTime: EventTime,
//     spectra: Vec<Spectrum>,
// }

// #[derive(Serialize, Deserialize)]
// type struct Spectrum {
//     resolutionBwHz float,
//     profiles: Vec<SpectrumProfile>,
// }

// #[derive(Serialize, Deserialize)]
// type struct SpectrumProfile {
//     list: Vec<SpectrumProfilePoint>,
// }

// #[derive(Serialize, Deserialize)]
// type struct SpectrumProfilePoint {
//     hz float,
//     dbm float,
// }

// #[derive(Serialize, Deserialize)]
// type struct EventTime {
//     startTime: String,
//     stopTime: String,
// }

// #[derive(Serialize, Deserialize)]
// type struct GeoSpectrumSpec {
//     location: GeoLocation,
//     spectrumSpecs: Vec<SpectrumSpec>,
// }

// #[derive(Serialize, Deserialize)]
// type struct DeviceValidity {
//     deviceDesc: DeviceDescriptor,
//     isValid: bool,
//     reason: String,
// }
