// copyright 2021 TVWS Project
package main

import (
	"encoding/json"
	"fmt"
	"paws/paws"
)

func main() {
	fmt.Println("PAWS protocol, version " + paws.PAWSVersion)

	// INIT_MSG
	request := paws.Request{}
	msg := request.InitMsg()
	req, _ := json.MarshalIndent(msg, "", "    ")
	fmt.Println("PAWS Request:")
	fmt.Println(string(req))

	// INT_RES
	response := paws.Response{}
	msg2 := response.InitMsg()
	res, _ := json.MarshalIndent(msg2, "", "    ")
	fmt.Println("PAWS Response:")
	fmt.Println(string(res))
}
