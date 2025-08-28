package transaction

import (
	"bytes"
	"goCode/internal/rpc"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func SetFakeBalance(rpcURL string, address string, balance string) {
	reqBody := rpc.RPCRequest{
		Jsonrpc: "2.0",
		Method:  "tenderly_setBalance",
		Params:  []interface{}{address, balance},
		ID:      1,
	}

	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		log.Fatalf("Greska pri konverziji u JSON: %v", err)
	}

	resp, err := http.Post(rpcURL, "application/json", bytes.NewBuffer(reqBytes))
	if err != nil {
		log.Fatalf("Greska pri slanju RPC zahteva: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Greska pri citanju odgovora: %v", err)
	}

	fmt.Println("Dodavanje laznog balansa odgovor:", string(body))
}
