# Simple Blockchain Implementation in Rust

This is a simple implementation of a blockchain in Rust. It provides a basic framework for creating and verifying blocks and transactions in a distributed ledger.

## Installation

To use this implementation, you will need to have Rust installed on your system. You can follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install Rust.

Once Rust is installed, you can clone this repository and build the project using the following commands:
```sh
cargo build --release
cargo run --bin rust_chain
```

## source code

### block.rs
A Block struct that represents a block in a blockchain. The Block struct has fields for the block's index, timestamp, hash, previous block hash, nonce, list of transactions, and difficulty level.

also a constructor method for creating new instances of Block, as well as a mine method that finds a valid nonce for the block by trying different nonce values until a valid one is found.

Additionally, the code implements the Debug trait for the Block struct, which allows us to print a human-readable representation of a Block using the {:?} formatter. The code also implements the Hashable trait for the Block struct, which allows us to calculate the hash of a Block. Finally, the code defines a check_difficulty function that checks if a given hash is valid given the block's difficulty.

### hashable.rs
a Hashable trait that provides methods for calculating the hash of an object that implements the trait. The bytes method returns the object's data as a vector of bytes, and the hash method calculates the SHA256 hash of the object's data using the crypto_hash crate. The Hashable trait can be implemented for any struct that needs to have its hash calculated.


### blockchain.rs
A Blockchain struct that represents a blockchain and its associated methods. It also defines an BlockValidationErr enum that represents the different types of errors that can occur when validating a block.

In the Blockchain struct, the blocks field is a vector of Blocks that represents the chain of blocks in the blockchain. The unspent_outputs field is a HashSet of Hashes that represents the set of unspent outputs in the blockchain.

The new() method creates a new Blockchain instance with an empty vector of blocks and an empty set of unspent outputs.

The update_with_block() method takes a Block as an argument and tries to append it to the blocks vector in the Blockchain instance. It first checks the block's index and hash to see if they match the expected values. If the block is not the genesis block, it also checks the block's timestamp and previous block hash to ensure they are valid.

Next, the method splits the block's transactions into the coinbase transaction and the other transactions. It then checks that the coinbase transaction is valid, and that the inputs and outputs of the other transactions are valid. If any of these checks fail, the method returns an error.

Finally, if all the checks pass, the method updates the unspent_outputs set with the spent and created outputs from the transactions in the block, and then adds the block to the blocks vector. If successful, the method returns Ok(()).

### transaction.rs
 a Transaction struct that represents a transaction in the blockchain. A Transaction has a vector of inputs and a vector of outputs.

The code also defines an Output struct, which represents an output in a transaction. An Output has an address to which it is being sent, and a value.

The code implements the Hashable trait for both the Transaction and Output structs, which allows us to calculate the hash of a Transaction or Output object.

The code also provides several methods for the Transaction struct, including methods for computing the sum of the values of the inputs or outputs, methods for computing the set of hashes of the inputs or outputs, and a method for checking if the transaction is a coinbase transaction (i.e. has no inputs).

### lib.rs
This code defines several types and functions that are used in the blockchain implementation.

## Tests
You can run the tests with 
```sh
cargo test
```