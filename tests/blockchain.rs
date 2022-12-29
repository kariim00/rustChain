use rust_chain::{Block, Blockchain};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        // Create a new blockchain instance
        let mut blockchain = Blockchain::new();

        // Ensure that the blockchain is initially empty
        assert!(blockchain.blocks.is_empty());
        assert!(blockchain.unspent_outputs().is_empty());

        // Create a new genesis block
        let mut block = Block::new(
            0,           // index
            0,           // timestamp
            vec![0; 32], // prev_block_hash
            vec![],      // transactions
            0x000fffffffffffffffffffffffffffff,           // difficulty
        );

        // Mine the block
        block.mine();

        // Add the block to the blockchain
        assert!(blockchain.update_with_block(block.clone()).is_ok());

        // Ensure that the blockchain was updated with the new block
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.blocks[0].clone(), block.clone());
        assert!(blockchain.unspent_outputs().is_empty());

        // Create a new block that builds on the previous one
        let mut block = Block::new(
            1,          // index
            1,          // timestamp
            block.clone().hash, // prev_block_hash
            vec![],     // transactions
            0x000fffffffffffffffffffffffffffff,          // difficulty
        );

        // Mine the block
        block.mine();

        // Add the block to the blockchain
        assert!(blockchain.update_with_block(block.clone()).is_ok());

        // Ensure that the blockchain was updated with the new block
        assert_eq!(blockchain.blocks.len(), 2);
        assert_eq!(blockchain.blocks[1].clone(), block.clone());
        assert!(blockchain.unspent_outputs().is_empty());

        // Try adding a block with a mismatched index
        let mut block = Block::new(
            2,          // index
            2,          // timestamp
            block.clone().hash, // prev_block_hash
            vec![],     // transactions
            0x000fffffffffffffffffffffffffffff,          // difficulty
        );

        // Mine the block
        block.mine();

        // Add the block to the blockchain
        assert!(blockchain.update_with_block(block).is_ok());

        // Ensure that the blockchain was not updated
        assert_eq!(blockchain.blocks.len(), 3);
        assert!(blockchain.unspent_outputs().is_empty());
    }
}
