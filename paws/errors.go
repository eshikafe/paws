// copyright 2021 Austin Aigbe
package paws

type Error struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	Data    string `json:"data"`
}

//   -101   VERSION          The Database does not support the specified
//                            version of the message.
//    -102   UNSUPPORTED      The Database does not support the device.
//                            For example, it supports none of the rulesets
//                            specified in the request or does not support
//                            the device, based on its device type, model,
//                            etc.  This error does not use any additional
//                            data.
//    -103   UNIMPLEMENTED    The Database does not implement the optional
//                            request or optional feature.  This error does
//                            not use any additional data.
//    -104   OUTSIDE_COVERAGE The specified geolocation is outside the
//                            coverage area of the Database.  The Database
//                            MAY include a DbUpdateSpec (Section 5.7) to
//                            provide a list of alternate Databases that
//                            might be appropriate for the requested
//                            location.  See OUTSIDE_COVERAGE Error
//                            (Section 5.17.1) for more details.
// -105   DATABASE_CHANGE  The Database has changed its URI.  The
// Database MAY include a DbUpdateSpec (Section
// 5.7) in the error response to provide devices
// with one or more alternate database URIs.
// The device needs to update its preconfigured
// entry for the responding Database with the
// alternate Databases listed in the
// DbUpdateSpec.  See DATABASE_CHANGE Error
// (Section 5.17.2) for more details.
// -200   (reserved)
// -201   MISSING          A required parameter is missing.  The
// Database MUST include a list of the required
// parameter names.  The Database MAY include
// only names of parameters that are missing,
// but MAY include a full list. Including the
// full list of missing parameters may reduce
// the number of re-queries from the device.
// See MISSING Error (Section 5.17.3) for more
// details.
// -202   INVALID_VALUE    A parameter value is invalid in some way.
// The Database SHOULD include a message
// indicating which parameter and why its value
// is invalid.  This error does not use any
// additional data.
// -300   (reserved)
// -301   UNAUTHORIZED     The device is not authorized to used the
// Database.   Authorization may be determined
// by the ruleset or be dependent on prior
// arrangement between the device and Database.
// This error does not use any additional data.
// -302   NOT_REGISTERED   Device registration required, but the device
// is not registered.  This error does not use
// any additional data.
// -32000 (reserved)       Reserved for JSON-RPC error codes.
// to
// -32768
