package rpc

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func GetLatestBlock(rpcURL string) string {
	reqBody := RPCRequest{
		Jsonrpc: "2.0",
		Method:  "eth_getBlockByNumber",
		Params:  []interface{}{"latest", true}, // "latest" - poslednji blok, true - hocemo i transakcije
		ID:      1,
	}

	// Pretvaramo strukturu u JSON
	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		log.Fatalf("Greska pri konverziji u JSON: %v", err)
	}

	resp, err := http.Post(rpcURL, "application/json", bytes.NewBuffer(reqBytes))
	if err != nil {
		log.Fatalf("Greska pri slanju RPC zahteva: %v", err)
	}
	defer resp.Body.Close()

	// Citamo telo odgovora
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Greska pri čitanju odgovora: %v", err)
	}

	// Parsiramo odgovor
	var rpcResp RPCResponse
	if err := json.Unmarshal(body, &rpcResp); err != nil {
		log.Fatalf("Greska pri parsiranju JSON odgovora: %v", err)
	}

	fmt.Println("BLOCK Odgovor:", string(rpcResp.Result))

	// Parsiramo blok
	var block Block
	if err := json.Unmarshal(rpcResp.Result, &block); err != nil {
		log.Fatalf("Greska pri parsiranju podataka bloka: %v", err)
	}

	fmt.Printf("Blok broj: %s\n", block.Number)
	fmt.Printf("Hash bloka: %s\n", block.Hash)
	fmt.Printf("Vreme rudarenja: %s\n", block.Timestamp)
	fmt.Printf("Broj transakcija: %d\n", len(block.Transactions))

	return block.Number
}
