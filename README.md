# BlockchainSimulations
This repository is dedicated to experimenting with blockchain technologies using a hybrid architecture that involves both Go and Rust. It focuses on cross-language communication via FFI (Foreign Function Interface) to fetch blockchain data, analyze and simulate transactions.

## Project Structure
``` bash
.
├── .github
│   └── workflows
│       └── check-env.yml
├── .gitignore
├── goCode
│   ├── .env        # .env file
│   ├── goApp
│   │   └── main.go
│   ├── go.mod
│   ├── go.sum
│   ├── internal
│   │   ├── config
│   │   │   └── config.go
│   │   ├── rpc
│   │   │   ├── rpc.go
│   │   │   └── types.go
│   │   ├── transaction
│   │   │   ├── balance.go
│   │   │   └── transaction.go
│   │   └── utilis
│   │       └── utilis.go
│   └── libs
│       └── (librpc.so)  or   (rpc.dll & librpc.a)       # Rust library used in Go via FFI
├── Makefile
├── README.md
└── rustCode
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── config
        │   └── mod.rs
        ├── lib.rs
        ├── main.rs
        ├── models
        │   └── mod.rs
        ├── rpc
        │   ├── fetch.rs
        │   └── mod.rs
        └── utils
            ├── functions.rs
            ├── mod.rs
            └── prints.rs
```
## Setup Instructions

### Prerequisites

Before you can run the program, ensure you have the following installed:

1. **Go**: Version 1.18 or higher. You can download it from the official [Go website](https://golang.org/dl/).
   
2. **Rust**: Ensure you have the Rust toolchain installed, which includes `cargo` and `rustc`. You can install it from the official [Rust website](https://www.rust-lang.org/learn/get-started).
   
3. **.env file**: Ensure the `.env` file is properly configured in the `goCode` folder with the following variables:

```env
PRIVATE_KEY = your_private_key
RECIPIENT_ADDRESS = recipient_address
RPC_TESTNET_URL = rpc_url_of_your_choice (Tenderly - recommended)
RPC_MAINNET_URL = rpc_url_of_your_choice (Infura recommended)
```
- **[Tenderly](https://tenderly.co/about-us)** is used for testnet interactions, providing a simulated blockchain environment ideal for testing and experimentation.
- **[Infura](https://www.infura.io/)** is used for mainnet data, enabling access to real-world blockchain data via a robust API service.
- **[Geth](https://geth.ethereum.org/downloads/)**. (if using locally): If running a local Geth node, set the RPC URL to your configured port. Ensure Geth is synced and running with RPC enabled.

## Building and Running the Program

You can build and run the program for both Linux and Windows using the Makefile. This automates the process of building the Rust FFI library, copying the appropriate library files, and running the Go application.

1. Navigate to the root of the project.
2. Run the following command to build and run the program:
    - **Build** and **run** for **Linux**
        ```
        make allLinux
        ```
        > This will:
        > - Build the Rust library (librpc.so)
        > - Copy the librpc.so file to the goCode/libs folder
        > - Run the Go application

    - **Build** and **run** for **Windows**
        ```
        make allWindows
        ```
        >This will:
        > - Build the Rust library (rpc.dll and rpc.dll.lib)
        > - Copy the rpc.dll to goCode/libs/rpc.dll and rpc.dll.lib to goCode/libs/librpc.a
        > - Run the Go application

## Makefile Commands
- `make allLinux`: Builds the Rust library for Linux, copies the library to the Go app's libs folder, and runs the Go application.
- `make allWindows`: Builds the Rust library for Windows, copies the library to the Go app's libs folder, and runs the Go application.
- `make clean`: Cleans up the build artifacts, including the `target` directory for Rust and the `librpc.so/rpc.dll` in the Go `libs` folder.
- `make run_rust`: Runs only the Rust code (`cargo run`).
- `make run_go`: Runs only the Go application (`go run main.go`).
- `make test_rust`: Runs unit tests for the Rust part of the program..

## Functionalities

1. **Go Program**:
   - **Fetches the latest block number** from the blockchain using the provided RPC URL via a standard RPC connection. It sends an RPC request to the Ethereum node and retrieves the latest block's details, including transactions.
   - **Sets a fake balance** for a recipient address using the Tenderly-specific `tenderly_setBalance` method via RPC to simulate a blockchain environment and test transaction functionalities.
   - **Sends a transaction** to the recipient address with a specified amount of Ether, utilizing the go-ethereum library (`ethclient`) to connect to an RPC endpoint, create, sign, and broadcast the transaction. This interacts with the blockchain via RPC.
   - **Uses the FFI mechanism** to call the Rust program and fetch transaction data from the latest block, enabling **cross-language communication** for blockchain data fetching and analysis.
   - **Fetches maximum gas transactions** from the last 5 blocks by calling Rust via **FFI**, which retrieves and analyzes transaction receipts to identify the **highest gas-used** transactions per block, including their **percentage** of the block's total gas.

2. **Rust Program**:
   - The Rust program can independently **fetch the latest block number** and details directly via RPC, similar to the Go program.
   - The Rust library (`rpc.dll` or `librpc.so`) exposes functions like `fetch_transactions`, which retrieves transaction data from the latest block on the blockchain. This function is called from the Go program through FFI.
   - Additionally, it exposes `fetch_max_tx_per_last_5_blocks`, which fetches the last 5 blocks, retrieves transaction receipts, and identifies the transaction with the maximum gas usage in each block, calculating its percentage of the block's gas limit. This data is returned as JSON for processing in Go.
   - The Rust program can run standalone to fetch and analyze blocks from both testnet and mainnet, printing block info, transaction details, and summaries of max gas transactions.

In summary:
- The **Go program** handles blockchain interactions like setting fake balances (via Tenderly RPC), sending transactions (via go-ethereum RPC client), and fetching block data, all through external RPC endpoints without requiring a local Geth client.
- The **Rust library** is used for fetching and analyzing blockchain data via RPC calls, including transaction details and gas usage summaries from multiple blocks. It communicates with Go through FFI for integrated functionality.


### FFI Usage:
The project uses **FFI (Foreign Function Interface)** to enable communication between Go and Rust. Go interacts with the Ethereum blockchain using RPC endpoints (via the go-ethereum library) for transaction creation and fake balance setting. Rust, on the other hand, fetches transaction data from the latest block through the rpc.dll library. Go calls Rust functions via FFI to retrieve blockchain data.

---

### Notes:

* **.env File**: Ensure the `.env` file is properly configured with the RPC URL, private key, and recipient address for both Go and Rust parts.

* **Tenderly Virtual Network**: The project runs on the **Virtual TestNets**, simulating Ethereum blockchain interactions for testing and safe experimentation.

* **Communication**: 
   - **Go** handles blockchain interaction (sending transactions, setting balances) using RPC endpoints via the go-ethereum library.
   - **Rust** fetches transaction details from the latest blocks via **RPC** and communicates with Go through **FFI**.

* **Simplify**: In this project, for the sake of simplicity and testing, the same address `RECIPIENT_ADDRESS` was used for both the sender and the recipient. This is done to simulate the transaction process without involving multiple real addresses. In a real-world scenario, the sender and recipient would have distinct addresses.



