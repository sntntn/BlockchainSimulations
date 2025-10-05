package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

func LoadEnv() (string, string, string) {
	err := godotenv.Load("../.env")
	if err != nil {
		log.Fatal("Greska pri ucitavanju .env fajla")
	}

	rpcURL := os.Getenv("RPC_URL")
	if rpcURL == "" {
		log.Fatal("Nedostaje RPC_URL u .env fajlu")
	}

	recipient := os.Getenv("RECIPIENT_ADDRESS")
	if recipient == "" {
		log.Fatal("Nedostaje RECIPIENT_ADDRESS u .env fajlu")
	}

	privateKeyHex := os.Getenv("PRIVATE_KEY")
	if privateKeyHex == "" {
		log.Fatal("Nedostaje PRIVATE_KEY u .env fajlu")
	}

	return rpcURL, recipient, privateKeyHex
}
