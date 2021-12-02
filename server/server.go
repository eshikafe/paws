// Copyright (c) 2021 TVWS-Project
// PAWS server

package server

import (
	"errors"
	"fmt"
	"log"
	"net/http"

	"github.com/eshikafe/paws/paws"
	"github.com/eshikafe/rpc/v2"
	"github.com/eshikafe/rpc/v2/json2"
)

type SpectrumPaws struct{}

func Start() {
	address := "127.0.0.1"
	port := 3030
	rpcServer := rpc.NewServer()
	json2Codec := json2.NewCodec()
	rpcServer.RegisterCodec(json2Codec, "application/json")
	rpcServer.RegisterService(new(SpectrumPaws), "SpectrumPaws")
	http.Handle("/", rpcServer)
	log.Printf("SpectrumDB started ["+address+":%v]\n", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(address+":%v", port), nil))
}

func (service *SpectrumPaws) Init(r *http.Request, reqParams *paws.InitReq, reply *paws.InitResp) error {
	log.Println("src_ip=" + r.RemoteAddr)
	if reqParams.Type != "INIT_REQ" {
		return errors.New("INIT_REQ type required, received " + reqParams.Type)
	}
	reply.Type = "INIT_RESP"
	reply.Version = paws.PAWSVersion
	reply.RulesetInfos = []paws.RulesetInfo{{
		Authority:         "ng",
		RulesetId:         "NccTvBandWhiteSpace-2010",
		MaxLocationChange: 100,
		MaxPollingSecs:    86400}}
	return nil
}

func (service *SpectrumPaws) Register(r *http.Request, reqParams *paws.RegistrationReq, reply *paws.RegistrationResp) error {
	log.Println("src_ip=" + r.RemoteAddr)
	if reqParams.Type != "REGISTRATION_REQ" {
		return errors.New("REGISTRATION_REQ type required, received " + reqParams.Type)
	}
	return nil
}
