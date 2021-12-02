// Copyright (c) 2021 TVWS-Project
//
// PAWS Error Codes (RFC 7545 section 5.17)

package paws

import "encoding/json"

// import "golang.org/x/tools/internal/jsonrpc2"

// 5.17.  Error Element
//    If the Database responds to a PAWS request message with an error, it
//    MUST include an Error element.
type Error struct {
	Code    int       `json:"code"`
	Message string    `json:"message"`
	Data    ErrorData `json:"data,omitempty"`
}

type ErrorData *json.RawMessage

// PAWS predefined and reserved error codes
var (
	ErrVersion         = -101
	ErrUnsupported     = -102
	ErrUnimplemented   = -103
	ErrOutsideCoverage = -104 // ErrorData {spec:DbUpdateSpec} optional
	ErrDatabaseChange  = -105 // ErrorData {spec:DbUpdateSpec}
	ErrMissing         = -201 // ErrorData {paramters: list}
	ErrInvalidValue    = -202
	ErrUnAuthorized    = -301
	ErrNotRegistered   = -302
)

func newError(code int) Error {
	var msg string

	switch code {
	case ErrVersion:
		msg = "Database does not support message version"
	case ErrUnsupported:
		msg = "Database does not support the device"
	case ErrUnAuthorized:
		msg = "The device is not authorized to use this database"
	}
	return Error{
		Code:    code,
		Message: msg,
	}
}

func (err *Error) Error() string {
	return err.Message
}

// PAWS Error Response format
// {
//   "jsonrpc": "2.0",
//   "error": {
// 		"code": ...,
// 		"message": "...",
// 		"data": ErrorData{}
//   },
//   "id": "idString"
// }
type ErrorResponse struct {
	Jsonrpc string `json:"jsonrpc"`
	Error   Error  `json:"error"`
	Id      string `json:"id"`
}

func (err *ErrorResponse) Init(code int) ErrorResponse {
	err.Jsonrpc = JSONVersion
	err.Error = newError(code)
	err.Id = "xxxx"
	return *err
}
