use std::collections::btree_map::BTreeMap;
use crate::{Address, Block, Transaction};

pub const DEFAULT_MINING_ROUNDS:usize = 3000;
pub const DEFAULT_FEE:u32 = 1;
pub const COINBASE_REWARD:u16 = 25;
pub const CONFIRMED_DEPTH:u8 = 2;
pub const POW_LEADING_ZEROS:usize = 3;



pub struct Blockchain {
    pow_leading_zeros:usize,
    coinbase_reward:u16,
    default_tx_fee:u32,
    confirmed_depth:u8
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain {
            pow_leading_zeros: POW_LEADING_ZEROS,
            coinbase_reward: COINBASE_REWARD,
            default_tx_fee: DEFAULT_FEE,
            confirmed_depth: CONFIRMED_DEPTH
        }
    }
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain{
            ..Default::default()
        }
    }

    pub fn make_genesis(starting_balances:BTreeMap<Address, u128>) -> Block {
        Block {
            balances: starting_balances,
            ..Default::default()
        }
    }

    //fn deserialize_block

    pub fn make_block(reward_addr:Address, prev_block:&Block) -> Block {
        Block::new(reward_addr, prev_block)
    }

    pub fn make_transaction(from:Address, nonce:u128, pubkey_bytes:Vec<u8>, outputs:Vec<(Address, u128)>, fee: u32, data: String) -> Transaction {
        Transaction::new(from, nonce, pubkey_bytes, outputs, fee, data)
    }
}