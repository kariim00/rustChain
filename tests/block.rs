use rust_chain::{Block, check_difficulty};

#[test]
fn test_block() {
    // Create a new block with a given index, timestamp, previous block hash, and list of transactions
    let mut block = Block::new(0, 12345, vec![0; 32], vec![], 0x000fffffffffffffffffffffffffffff);

    // Verify that the block's properties are initialized correctly
    assert_eq!(block.index, 0);
    assert_eq!(block.timestamp, 12345);
    assert_eq!(block.prev_block_hash, vec![0; 32]);
    assert_eq!(block.nonce, 0);
    assert_eq!(block.transactions.len(), 0);
    assert_eq!(block.difficulty, 0x000fffffffffffffffffffffffffffff);

    // Mine the block
    block.mine();

    // Verify that the block's nonce and hash are updated correctly
    assert_ne!(block.nonce, 0);
    assert_ne!(block.hash, vec![0; 32]);

    // Verify that the block's hash satisfies the difficulty level
    assert!(check_difficulty(&block.hash, block.difficulty));
}
