# BlockchainSimulations
This repository is dedicated to experimenting with blockchain technologies.

## Project Structure
```
./
├── README.md*
├── code/
│   ├── go.mod*
│   ├── go.sum*
│   ├── goApp/
│   │   ├── librpc.a*
│   │   ├── main.go*
│   │   └── rpc.dll*
│   └── internal/
│       ├── config/
│       │   └── config.go*
│       ├── rpc/
│       │   ├── rpc.go*
│       │   └── types.go*
│       ├── transaction/
│       │   ├── balance.go*
│       │   └── transaction.go*
│       └── utilis/
│           └── utilis.go*
└── rustCode/
    ├── Cargo.lock*
    ├── Cargo.toml*
    └── src/
        ├── config/
        │   └── mod.rs*
        ├── lib.rs*
        ├── main.rs*
        ├── models/
        │   └── mod.rs*
        └── rpc/
            ├── fetch.rs*
            └── mod.rs*
```
## Setup Instructions

### Prerequisites

Before you can run the program, ensure you have the following installed:

1. **Go**: Version 1.18 or higher. You can download it from the official [Go website](https://golang.org/dl/).
   
2. **Rust**: Ensure you have the Rust toolchain installed, which includes `cargo` and `rustc`. You can install it from the official [Rust website](https://www.rust-lang.org/learn/get-started).
   
3. **Geth**: The **Go Ethereum** client is required for the Go program to interact with the Ethereum blockchain. You can download it from the official [Geth website](https://geth.ethereum.org/downloads/).

4. **.env file**: Ensure the `.env` file is properly configured in the `code` folder with the following variables:

```env
PRIVATE_KEY=your_private_key
RECIPIENT_ADDRESS=recipient_address
RPC_URL=rpc_url_of_your_choice
```

5. **Tenderly Virtual Network** (Optional but Recommended)

    - The project runs on the **Tenderly Virtual Network**, which simulates Ethereum blockchain interactions for testing and safe experimentation. Tenderly provides a controlled environment where you can simulate and test Ethereum transactions, making it ideal for development and experimentation without risking real assets.

    - You can connect to the Tenderly Virtual Network by configuring the appropriate RPC URL in the `.env` file.



### Building and Running the Program on Windows
**Rust Part (FFI Library)**
  - Navigate to the `rustCode` folder.
  - Run the following command to build the Rust FFI library:
```
cargo build --release
```
  - this will generate **rpc.dll** and **rpc.dll.lib** in `rustCode/target/release` folder
  - Then copy them to `code/goApp` folder as
      - rpc.dll -> **rpc.dll**
      - rpc.dll.lib -> **librpc.a** *(renamed)*
    
**Go Part**
  - Navigate to the `code/goApp` folder.
  - Then run 
```go run main.go```

You can allso run just rust part of rust code trough command `cargo run` in `rustCode` folder



## Functionalities

1. **Go Program**:
   - **Fetches the latest block number** from the blockchain using the provided RPC URL via a standard RPC connection. It sends an RPC request to the Ethereum node and retrieves the latest block's details, including transactions.
   - **Sets a fake balance** for a recipient address using the **Geth client** in the Go program, by calling the `tenderly_setBalance` method to simulate a blockchain environment and test transaction functionalities.
   - **Sends a transaction** to the recipient address with a specified amount of Ether, utilizing the **Geth client** to interact directly with the Ethereum blockchain. This includes creating and signing the transaction using a private key and then broadcasting it to the network.
   - **Uses the FFI mechanism** to call the Rust program and fetch transaction data from the latest block, enabling cross-language communication for blockchain data fetching and analysis.
  
2. **Rust Program**:
   - The Rust Program can also **fetch the latest block number** directly as Go Program
   - The Rust library (`rpc.dll`) exposes the function `fetch_transactions`, which retrieves **transaction data** from the latest block on the blockchain. This function is called from the Go program through FFI.

In summary:
- The **Go program** uses **Geth client** for sending transactions and setting fake balances, directly interacting with the Ethereum blockchain.
- The **Rust library** is used for fetching blockchain data via RPC calls for getting transaction details from the latest block.



### FFI Usage:
The project uses **FFI (Foreign Function Interface)** to enable communication between Go and Rust. Go interacts with the Ethereum blockchain using the **Geth client** for transaction creation and fake balance setting. Rust, on the other hand, fetches transaction data from the latest block through the `rpc.dll` library. Go calls Rust functions via FFI to retrieve blockchain data.

---

### Notes:

* **.env File**: Ensure the `.env` file is properly configured with the RPC URL, private key, and recipient address for both Go and Rust parts.

* **Tenderly Virtual Network**: The project runs on the **Virtual TestNets**, simulating Ethereum blockchain interactions for testing and safe experimentation.

* **Communication**: 
   - **Go** handles blockchain interaction (sending transactions, setting balances) using the **Geth client**.
   - **Rust** fetches transaction details from the latest block via **RPC** and communicates with Go through **FFI**.

* **Simplify**: In this project, for the sake of simplicity and testing, the same address `RECIPIENT_ADDRESS` was used for both the sender and the recipient. This is done to simulate the transaction process without involving multiple real addresses. In a real-world scenario, the sender and recipient would have distinct addresses.

