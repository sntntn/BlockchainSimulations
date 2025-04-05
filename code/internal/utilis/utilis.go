package utilis

import (
	"fmt"
	"math/big"
)

func EthToWei(eth int64) *big.Int {
	// 1 ETH = 10^18 wei
	weiAmount := new(big.Int).SetInt64(1000000000000000000)
	ethAmount := new(big.Int).SetInt64(eth)

	weiValue := new(big.Int).Mul(ethAmount, weiAmount) //mnozimo

	return weiValue
}

func EthToHex(eth int64) string {
	weiValue := EthToWei(eth)

	weiHex := fmt.Sprintf("0x%s", weiValue.Text(16))

	return weiHex
}
