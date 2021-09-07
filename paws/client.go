// copyright 2021 TVWS Project
package paws

// A PAWS request message is carried in the body of an HTTP POST
// request.  A PAWS response message is carried in the body of an HTTP
// response.  A PAWS response SHOULD include a Content-Length header.

// The POST method is the only method REQUIRED for PAWS.  If a Database
// chooses to support GET, it MUST be an escaped URI, but the encoding
// of the URI is outside the scope of this document.  The Database MAY
// refuse to support the GET request by returning an HTTP error code,
// such as 405 (method not allowed).

// func Client() {
// 	u := User{Id: "US123", Balance: 8}
// 	b := new(bytes.Buffer)
// 	json.NewEncoder(b).Encode(u)
// 	res, _ := http.Post("http://localhost:3030", "application/json", b)
// 	io.Copy(os.Stdout, res.Body)
// }
