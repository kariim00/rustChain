use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex, // Error indicating that the block's index does not match the expected value
    InvalidHash, // Error indicating that the block's hash does not meet the required difficulty
    AchronologicalTimestamp, // Error indicating that the block's timestamp is not chronologically after the previous block
    MismatchedPreviousHash, // Error indicating that the block's previous block hash does not match the expected value
    InvalidGenesisBlockFormat, // Error indicating that the format of the genesis block is invalid
    InvalidInput, // Error indicating that one or more of the block's transactions have an invalid input
    InsufficientInputValue, // Error indicating that one or more of the block's transactions have insufficient input value to cover the output value
    InvalidCoinbaseTransaction, // Error indicating that the block's coinbase transaction is invalid
}

pub struct Blockchain {
    pub blocks: Vec<Block>,// Vector of blocks in the blockchain
    unspent_outputs: HashSet<Hash>, // Set of unspent outputs in the blockchain
}

impl Blockchain {
    // Creates a new instance of the Blockchain struct with an empty vector of blocks and an empty set of unspent outputs
    pub fn new () -> Self {
        Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
    // Define a `getter` method for the `unspent_outputs` field
    pub fn unspent_outputs(&self) -> &HashSet<Hash> {
        &self.unspent_outputs
    }

    // Updates the blockchain instance with a new block
    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        // Check the block's index and hash
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 { // If the block is not the genesis block, check its timestamp and previous block hash
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
             // If the block is the genesis block, check its previous block hash
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        // Split the block's transactions into the coinbase transaction and the other transactions

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
             // Check that the coinbase transaction is valid
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if
                    !(&input_hashes - &self.unspent_outputs).is_empty() ||
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}