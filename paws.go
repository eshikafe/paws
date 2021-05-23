package main

import (
	"encoding/json"
	"fmt"
	"paws/paws"
)

func main() {
	fmt.Println("PAWS protocol")

	// INIT_MSG
	req := paws.Request{}
	msg := req.InitMsg()
	res, _ := json.Marshal(msg)
	fmt.Println(string(res))
}
