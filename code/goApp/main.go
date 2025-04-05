package main

import (
	"fmt"

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

}
