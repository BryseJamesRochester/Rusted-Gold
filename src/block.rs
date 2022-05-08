use std::collections::HashMap;
use std::ptr::null;
use super::*;

pub struct Block {
    pub reward_addr:Address,
    pub prev_block_hash:Hash,
    pub pow_target:Hash,
    pub proof:u128,
    pub coinbase_reward:u16,
    //transactions:Vec<Transaction>,
    pub chain_length:u32,
    pub timestamp:u128,
    pub balances:HashMap<Address, u128>,
    pub next_nonce:HashMap<Address, u128>
}
impl Default for Block {
    fn default() -> Block {
        Block {
            reward_addr:"0".to_string(),
            prev_block_hash:vec![0;32],
            pow_target:calc_pow_target(),
            proof:0,
            coinbase_reward:0,
            //transactions:Vec<Transaction>,
            chain_length:0,
            timestamp:now(),
            balances:HashMap::new(),
            next_nonce:HashMap::new()
        }
    }
}

impl Block {
    pub fn new (reward_addr:Address, coinbase_reward:u16, prev_block:&Block) -> Self {
        Block {
            reward_addr,
            prev_block_hash: prev_block.hash_val(),
            coinbase_reward,
            //transactions,
            chain_length: prev_block.chain_length+1,
            balances: prev_block.balances.clone(),
            next_nonce: prev_block.next_nonce.clone(),
            ..Default::default()
        }
    }

    //add_transaction(tx)

    //rerun(prev_block)

    //total_rewards()

    //contains(tx)

    pub fn balance_of(&self, address:&Address) -> u128 {
        match self.balances.get(address){
            Some(bal) => bal.clone(),
            None => 0
        }
    }

    pub fn is_genesis(&self) -> bool {
        self.chain_length == 0
    }

    pub fn has_valid_proof(&self) -> bool {
        //self.hash_val() < self.pow_target
        let hash = self.hash_val();
        for i in 0..hash.len() {
            if self.pow_target[i] > 0x0f {return true;}
            if hash[i] > self.pow_target[i] {return false;}
        }
        true
    }

    pub fn mine(&mut self) -> u128 {
        while !self.has_valid_proof() {
            self.proof += 1;
        }
        self.proof
    }

    fn serialize(&self) -> String {
        format!("{}||{:?}||{:?}||{}||{}||{}||{}",
                self.reward_addr,
                self.prev_block_hash,
                self.pow_target,
                self.proof,
                self.coinbase_reward,
                //self.transactions,
                self.chain_length,
                self.timestamp,
        )
    }

    pub fn hash_val(&self) -> Hash {
        hash(self.serialize().as_bytes()).to_vec()
    }
}