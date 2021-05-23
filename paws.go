// copyright 2021 Austin Aigbe
package main

import (
	"encoding/json"
	"fmt"
	"paws/paws"
)

func main() {
	fmt.Println("PAWS protocol")

	// INIT_MSG
	request := paws.Request{}
	msg := request.InitMsg()
	req, _ := json.Marshal(msg)
	fmt.Println(string(req))

	// INT_RES
	response := paws.Response{}
	msg2 := response.InitMsg()
	res, _ := json.Marshal(msg2)
	fmt.Println(string(res))
}
