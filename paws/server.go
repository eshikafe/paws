// copyright 2021 Austin Aigbe
package paws

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	//"golang.org/x/tools/internal/jsonrpc2"
	// "net/rpc/jsonrpc"
)

func StartServer() {
	//pawsHandler := new(PAWSHandler)
	//rpc.Register(pawsHandler)
	port := 3030
	http.HandleFunc("/", pawsHandler)
	//listener, err := net.Listen("tcp", fmt.Sprintf(":%v", port))
	// if err != nil {
	// 	log.Fatal(fmt.Sprintf("Unable to listen on port %s: %s", port, err))
	// }
	log.Printf("PAWS server starting on port %v\n", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf("127.0.0.1:%v", port), nil))
}

func pawsHandler(w http.ResponseWriter, r *http.Request) {
	var response Response
	var request Request
	var paws_err_resp ErrorResponse

	body, err := ioutil.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}
	err = json.Unmarshal(body, &request)
	if err != nil {
		//http.Error(w, "JSON Unmarshal error", http.StatusBadRequest)
		paws_err_resp = paws_err_resp.Init(ErrVersion)
		encoder := json.NewEncoder(w)
		encoder.Encode(&paws_err_resp)
		return
	}
	if request.Method == "spectrum.paws.init" && request.Params.Type == "INIT_REQ" {
		response = response.Init()
	} else {
		response = Response{}
	}
	encoder := json.NewEncoder(w)
	encoder.Encode(&response)

}
