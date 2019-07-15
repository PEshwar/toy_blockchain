use serde::{Deserialize, Serialize};
extern crate crypto_hash;
extern crate serde_json;
use crypto_hash::{hex_digest, Algorithm};

fn main() {
    println!("Jaya Guru Datta. Attempting to build blockchain in rust!");
}
// Define data structure for transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    id: String,
    timestamp: u64,
    // sender: String,
    // receiver: String,
    // amount: u32,
    payload: String,
}

// Define data structure for block

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    previous_block_hash: String,
    timestamp: u64,
    transaction_list: Vec<Transaction>,
    block_number: u64,
    pub nonce: u64,
}

// Define Block implementation

impl Block {

    pub fn new(timestamp: u64, transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            block_number: previous_block.block_number + 1,
            timestamp: timestamp,
            transaction_list: transactions,
            previous_block_hash: Self::hash(previous_block),
            nonce: 0,
            
        }
    }

    pub fn genesis() -> Block {
        let transaction = Transaction {
            id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
            timestamp: 0,
            payload: String::from("Prabhu's first blockchain"),
        };
        Block {
            block_number: 1,
            timestamp: 0,
            transaction_list: vec![transaction],
            previous_block_hash: String::from("0"),
            nonce: 0,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.to_json().as_bytes())
    }

    pub fn is_valid(hash: &str, prefix: &str) -> bool {
        hash.starts_with(prefix)
    }
    fn mine_next_block(candidate_block: &mut Block, prefix: &str)  {
    while !Self::is_valid(&Self::hash(candidate_block),prefix) {
        candidate_block.nonce += 1
    }
}
}

// Define data structure for chain of blocks

struct Blockchain {
    blocks: Vec<Block>,
}

// Define function for mining

// How does consensus algorithm fit in here? How to receive blocks from other // peers in P2P network?
