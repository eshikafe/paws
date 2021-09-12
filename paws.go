// copyright 2021 TVWS Project
package main

import "paws/paws"

func main() {
	// fmt.Println("PAWS protocol, version " + paws.PAWSVersion)

	// INIT_MSG
	// var request paws.Request
	// msg := request.InitMsg()
	// req, _ := json.Marshal(msg)
	// fmt.Println(string(req))

	// INT_RES
	// var response paws.Response
	// msg2 := response.InitMsg()
	// res, _ := json.MarshalIndent(msg2, "", "    ")
	// fmt.Println(string(res))

	paws.StartServer()
}
