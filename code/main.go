package main

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"math/big"
	"net/http"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/joho/godotenv"
)

type RPCRequest struct {
	Jsonrpc string        `json:"jsonrpc"`
	Method  string        `json:"method"`
	Params  []interface{} `json:"params"`
	ID      int           `json:"id"`
}

type RPCResponse struct {
	Jsonrpc string          `json:"jsonrpc"`
	ID      int             `json:"id"`
	Result  json.RawMessage `json:"result"` // Sirovi JSON za parsiranje
}

type Block struct {
	Number       string        `json:"number"`
	Hash         string        `json:"hash"`
	Timestamp    string        `json:"timestamp"`
	Transactions []Transaction `json:"transactions"`
}

type Transaction struct {
	Hash     string `json:"hash"`
	From     string `json:"from"`
	To       string `json:"to"`
	Value    string `json:"value"`
	Gas      string `json:"gas"`
	GasPrice string `json:"gasPrice"`
}

// --------------------------------------------------------------------

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Greska pri učitavanju .env fajla")
	}

	rpcURL := os.Getenv("RCP_URL")
	if rpcURL == "" {
		log.Fatal("Nedostaje RCP URL u .env fajlu")
	}
	blockNumber := getLatestBlock(rpcURL)
	fmt.Println(blockNumber)
	fmt.Println()

	recipientAddress := os.Getenv("RECIPIENT_ADDRESS")
	if recipientAddress == "" {
		log.Fatal("Nedostaje RECIPIENT_ADDRESS u .env fajlu")
	}
	fakeBalance := ethToHex(100) // 100 ETH
	setFakeBalance(rpcURL, recipientAddress, fakeBalance)

	sendEth := 1
	sendTransaction(rpcURL, recipientAddress, int64(sendEth))

}

// --------------------------------------------------------------------

func ethToWei(eth int64) *big.Int {
	// 1 ETH = 10^18 wei
	weiAmount := new(big.Int).SetInt64(1000000000000000000)
	ethAmount := new(big.Int).SetInt64(eth)

	weiValue := new(big.Int).Mul(ethAmount, weiAmount) //mnozimo

	return weiValue
}

func ethToHex(eth int64) string {
	weiValue := ethToWei(eth)

	weiHex := fmt.Sprintf("0x%s", weiValue.Text(16))

	return weiHex
}

func getLatestBlock(rpcURL string) string {
	reqBody := RPCRequest{
		Jsonrpc: "2.0",
		Method:  "eth_getBlockByNumber",
		Params:  []interface{}{"latest", true}, // "latest" - poslednji blok, true - hocemo i transakcije
		ID:      1,
	}

	// Pretvaramo strukturu u JSON
	reqBytes, err := json.Marshal(reqBody)
	if err != nil {
		log.Fatalf("Greška pri konverziji u JSON: %v", err)
	}

	resp, err := http.Post(rpcURL, "application/json", bytes.NewBuffer(reqBytes))
	if err != nil {
		log.Fatalf("Greška pri slanju RPC zahteva: %v", err)
	}
	defer resp.Body.Close()

	// Citamo telo odgovora
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatalf("Greška pri čitanju odgovora: %v", err)
	}

	// Parsiramo odgovor
	var rpcResp RPCResponse
	if err := json.Unmarshal(body, &rpcResp); err != nil {
		log.Fatalf("Greška pri parsiranju JSON odgovora: %v", err)
	}

	fmt.Println("BLOCK Odgovor:", string(rpcResp.Result))

	// Parsiramo blok
	var block Block
	if err := json.Unmarshal(rpcResp.Result, &block); err != nil {
		log.Fatalf("Greška pri parsiranju podataka bloka: %v", err)
	}

	fmt.Printf("Blok broj: %s\n", block.Number)
	fmt.Printf("Hash bloka: %s\n", block.Hash)
	fmt.Printf("Vreme rudarenja: %s\n", block.Timestamp)
	fmt.Printf("Broj transakcija: %d\n", len(block.Transactions))

	return block.Number
}

// --------------------------------------------------------------------

func setFakeBalance(rpcURL string, address string, balance string) {
	reqBody := RPCRequest{
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

// --------------------------------------------------------------------

func sendTransaction(rpcURL string, recipientAddress string, sendEth int64) {

	privateKeyHex := os.Getenv("PRIVATE_KEY")

	if privateKeyHex == "" {
		log.Fatal("Nedostaje PRIVATE_KEY u .env fajlu")
	}

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
