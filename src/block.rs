use super::{
    difficulty_bytes_as_u128, u128_bytes, u32_bytes, u64_bytes, Hash, Hashable, Transaction,
};
use std::fmt::{self, Debug, Formatter};

// Define a new struct, `Block`, that represents a block in the blockchain.
// Each block has an index, a timestamp, a hash, the hash of the previous block in the chain,
// a nonce, a list of transactions, and a difficulty level.
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block {
            index: self.index,
            timestamp: self.timestamp,
            hash: self.hash.clone(),
            prev_block_hash: self.prev_block_hash.clone(),
            nonce: self.nonce,
            transactions: self.transactions.clone(),
            difficulty: self.difficulty,
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index &&
        self.timestamp == other.timestamp &&
        self.hash == other.hash &&
        self.prev_block_hash == other.prev_block_hash &&
        self.nonce == other.nonce &&
        self.transactions == other.transactions &&
        self.difficulty == other.difficulty
    }
}

// Implement the `Debug` trait for `Block`, which allows us to print a human-readable representation
// of the `Block` struct using the `{:?}` formatter.
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}

// Implement some methods for the `Block` struct.
impl Block {
    // Define a constructor method for `Block` that allows us to create a new instance
    // of `Block` with the given parameters.
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    // Mine the block by finding a valid nonce
    pub fn mine(&mut self) {
        // Try different nonce values until a valid one is found
        for nonce_attempt in 0..(u64::max_value()) {
            // Set the block's nonce to the current attempt
            self.nonce = nonce_attempt;
            // Calculate the hash of the block
            let hash = self.hash();
            // Check if the hash is valid given the block's difficulty
            if check_difficulty(&hash, self.difficulty) {
                // If the hash is valid, set the block's hash and return
                self.hash = hash;
                return;
            }
        }
    }
}

// Implement the Hashable trait for the Block struct
// This allows us to calculate the hash of a block
impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        // Convert the block's data to bytes and append to the vector
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
