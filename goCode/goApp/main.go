package main

/*
#cgo LDFLAGS: -L../libs -lrpc -Wl,-rpath=../libs
#include <stdlib.h>

// fetch_transactions je Rust funkcija koja vraca podatke o transakcijama iz poslednjeg bloka
char* fetch_transactions(const char* rpc_url);

// fetch_max_tx_per_last_5_blocks je Rust funkcija koja vraca podatke o transakciji maksimalnog potrosenog gasa za svaki od poslednjih 5 blokova
char* fetch_max_tx_per_last_5_blocks(const char* rpc_url);
*/
import "C"

import (
	"encoding/json"
	"fmt"
	"unsafe"

	"goCode/internal/config"
	"goCode/internal/rpc"
	"goCode/internal/transaction"
	"goCode/internal/utilis"
)

func main() {

	rpcTestnetURL, rpcMainnetURL, recipientAddress, privateKeyHex := config.LoadEnv()

	blockNumber := rpc.GetLatestBlock(rpcTestnetURL)
	fmt.Println(blockNumber)
	fmt.Println()

	fakeBalance := utilis.EthToHex(100) // 100 ETH
	transaction.SetFakeBalance(rpcTestnetURL, recipientAddress, fakeBalance)

	sendEth := 1
	transaction.SendTransaction(rpcTestnetURL, recipientAddress, int64(sendEth), privateKeyHex)

	fmt.Println()

	// === Deo koji zove Rust FFI ===
	cRpcTestnetURL := C.CString(rpcTestnetURL)
	defer C.free(unsafe.Pointer(cRpcTestnetURL))

	fmt.Println("----------Rust pokrece FFI za transakcije iz poslednjeg bloka:----------")
	result := C.fetch_transactions(cRpcTestnetURL)
	defer C.free(unsafe.Pointer(result))
	fmt.Println("----------Rust je zavrsio FFI----------")
	goResult := C.GoString(result)
	fmt.Println("Rust FFI: Podaci o transakcijama iz poslednjeg bloka:\n", goResult)

	fmt.Println()

	cRpcMainnetURL := C.CString(rpcMainnetURL)
	defer C.free(unsafe.Pointer(cRpcMainnetURL))
	fmt.Println("----------Rust pokrece FFI za maksimalne transakcije iz poslednjih 5 blokova:----------")
	resultArray := C.fetch_max_tx_per_last_5_blocks(cRpcMainnetURL)
	defer C.free(unsafe.Pointer(resultArray))
	fmt.Println("----------Rust je zavrsio FFI----------")
	goArray := C.GoString(resultArray)

	var summaries []rpc.TxSummary
	err := json.Unmarshal([]byte(goArray), &summaries)
	if err != nil {
		fmt.Println("Greska pri parsiranju JSON-a iz Rust-a:", err)
		return
	}

	for _, s := range summaries {
		fmt.Printf("Blok %s | MAX TX %s | Gas %d | %.3f%% u bloku\n",
			s.BlockNumber, s.TxHash, s.GasUsed, s.PercentInBlock)
	}
}
