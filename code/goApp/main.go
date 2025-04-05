package main

import (
	"fmt"
)

// --------------------------------------------------------------------

func main() {

	rpcURL, recipientAddress, privateKeyHex := LoadEnv()

	blockNumber := getLatestBlock(rpcURL)
	fmt.Println(blockNumber)
	fmt.Println()

	fakeBalance := ethToHex(100) // 100 ETH
	setFakeBalance(rpcURL, recipientAddress, fakeBalance)

	sendEth := 1
	sendTransaction(rpcURL, recipientAddress, int64(sendEth), privateKeyHex)

}
