package main

import (
	"encoding/json"
	"fmt"

	"paws"
)

func main() {
	fmt.Println("PAWS protocol")

	// INIT_MSG
	msg := paws.InitMsg()
	res, _ := json.Marshal(string(msg))
	fmt.Println(string(res))
}
