package main

/*
#cgo LDFLAGS: -L. -lrpc
#include <stdlib.h>

// fetch_transactions je Rust funkcija koja vraca podatke o transakcijama iz poslednjeg bloka
char* fetch_transactions(const char* rpc_url);
*/
import "C"

import (
	"fmt"
	"unsafe"

	"code/internal/config"
	"code/internal/rpc"
	"code/internal/transaction"
	"code/internal/utilis"
)

func main() {

	rpcURL, recipientAddress, privateKeyHex := config.LoadEnv()

	blockNumber := rpc.GetLatestBlock(rpcURL)
	fmt.Println(blockNumber)
	fmt.Println()

	fakeBalance := utilis.EthToHex(100) // 100 ETH
	transaction.SetFakeBalance(rpcURL, recipientAddress, fakeBalance)

	sendEth := 1
	transaction.SendTransaction(rpcURL, recipientAddress, int64(sendEth), privateKeyHex)

	//---------------------------
	fmt.Println()

	cRpcURL := C.CString(rpcURL)
	defer C.free(unsafe.Pointer(cRpcURL))

	result := C.fetch_transactions(cRpcURL)
	defer C.free(unsafe.Pointer(result))

	goResult := C.GoString(result)
	fmt.Println("Podaci o transakcijama iz poslednjeg bloka:\n", goResult)

}
