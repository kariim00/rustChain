use rust_chain::{Block, Blockchain, Transaction, transaction, now};

fn main () {
    // Difficulty is the number of leading zeros that the hash of a block must have
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    // Create the first block in the blockchain (called the "genesis block")
    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 70,
                },
            ],
        },
    ], difficulty);

    // "Mine" the block by finding a nonce that results in a hash with the required number of leading zeros
    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);
    
    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();
    
    // Add the genesis block to the blockchain
    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 1,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 2,
                },
            ],
        },
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    blockchain.update_with_block(block).expect("Failed to add block");
}