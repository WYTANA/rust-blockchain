extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Serialize, Clone, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    previous_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

// #[derive(Serialize, Debug)]
pub struct Chain {
    chain: Vec<Block>,
    curr_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_transaction.push(Transaction {
            sender,
            receiver,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            previous_hash: self.last_hash(),
            // merkle: String::from_utf8(vec![48; 64]).unwrap(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };
    }
}
