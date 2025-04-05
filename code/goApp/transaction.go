package main

import (
	"context"
	"crypto/ecdsa"
	"fmt"
	"log"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
)

func sendTransaction(rpcURL string, recipientAddress string, sendEth int64, privateKeyHex string) {
	client, err := ethclient.Dial(rpcURL)
	if err != nil {
		log.Fatalf("Neuspesna konekcija: %v", err)
	}
	defer client.Close()

	privateKey, err := crypto.HexToECDSA(privateKeyHex[2:]) //uklanja "0x"
	if err != nil {
		log.Fatalf("Neuspesna konverzija privatnog kljuca: %v", err)
	}

	// Adresa pošiljaoca
	publicKey := privateKey.Public().(*ecdsa.PublicKey)
	fromAddress := crypto.PubkeyToAddress(*publicKey)

	// Adresa primaoca
	toAddress := common.HexToAddress(recipientAddress)

	// Dohvati nonce (broj transakcija sa ove adrese)
	nonce, err := client.PendingNonceAt(context.Background(), fromAddress)
	if err != nil {
		log.Fatalf("Greska pri dohvatanju nonce-a: %v", err)
	}

	gasLimit := uint64(21000)
	gasPrice, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatalf("Greska pri dohvatanju gas cene: %v", err)
	}

	value := ethToWei(sendEth)

	tx := types.NewTransaction(nonce, toAddress, value, gasLimit, gasPrice, nil)

	// Potpisivanje transakcije
	chainID, err := client.NetworkID(context.Background())
	if err != nil {
		log.Fatalf("Greska pri dohvatanju ID mreže: %v", err)
	}
	signedTx, err := types.SignTx(tx, types.NewEIP155Signer(chainID), privateKey)
	if err != nil {
		log.Fatalf("Greska pri potpisivanju transakcije: %v", err)
	}
	// --------

	err = client.SendTransaction(context.Background(), signedTx)
	if err != nil {
		log.Fatalf("Greska pri slanju transakcije: %v", err)
	}

	fmt.Printf("USPESNO poslata transakcija! TX Hash: %s\n", signedTx.Hash().Hex())
}
