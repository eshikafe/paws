// copyright 2021 Austin Aigbe
package paws

// import "golang.org/x/tools/internal/jsonrpc2"

type Error struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	Data    string `json:"data,omitempty"`
}

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

var (
	ErrVersion         = -101 // "PAWS Error: The database does not support the specified version of the message
	ErrUnsupported     = -102 // "The Database does not support the device
	ErrUnimplemented   = -103
	ErrOutsideCoverage = -104
	ErrDatabaseChange  = -105
	ErrMissing         = -201
	ErrInvalidValue    = -202
	ErrUnAuthorized    = -301 // "The device is not authorized to use the database
	ErrNotRegistered   = -302
)

// PAWS Error Response
// The error JSON-RPC Response for PAWS has the following form:
// {
//   "jsonrpc": "2.0",
//   "error": {
// 		"code": -102,
// 		"message": "An appropriate error message.",
// 		"data": { ... }
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
