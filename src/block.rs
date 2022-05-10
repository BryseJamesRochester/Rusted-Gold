use std::collections::{BTreeMap, HashMap};
use std::ptr::null;
use super::*;
use serde::*;
use serde::Serializer;
use serde_json::*;

#[derive(Serialize)]
pub struct Block {
    pub reward_addr:Address,
    pub prev_block_hash:Hash,
    pub pow_target:Hash,
    pub proof:u128,
    pub coinbase_reward:u16,
    pub transactions:BTreeMap<Hash, Transaction>,
    pub chain_length:u32,
    pub timestamp:u128,
    pub balances:BTreeMap<Address, u128>,
    pub next_nonce:BTreeMap<Address, u128>
}
impl Default for Block {
    fn default() -> Block {
        Block {
            reward_addr:"0".to_string(),
            prev_block_hash:Hash(vec![0;32]),
            pow_target:calc_pow_target(),
            proof:0,
            coinbase_reward:0,
            transactions:BTreeMap::new(),
            chain_length:0,
            timestamp:0,
            balances:BTreeMap::new(),
            next_nonce:BTreeMap::new()
        }
    }
}

fn reward_coinbase(prev_block:&Block) -> BTreeMap<Address, u128> {
    let winner_balance = prev_block.balance_of(&prev_block.reward_addr);
    let mut balances = prev_block.balances.clone();
    let new_winner_balance = match winner_balance.checked_add(prev_block.coinbase_reward as u128) {
        Some(bal) => bal,
        None => u128::MAX
    };
    balances.insert(prev_block.reward_addr.clone(), new_winner_balance);
    balances
}

impl Block {
    pub fn new (reward_addr:Address, coinbase_reward:u16, transactions:BTreeMap<Hash, Transaction>, prev_block:&Block) -> Self {
        Block {
            reward_addr,
            prev_block_hash: prev_block.hash_val(),
            coinbase_reward,
            transactions,
            chain_length: prev_block.chain_length+1,
            balances: reward_coinbase(prev_block),
            next_nonce: prev_block.next_nonce.clone(),
            ..Default::default()
        }
    }

    pub fn add_transaction(&mut self, mut tx:Transaction) -> bool {
        if self.transactions.contains_key::<Hash>(&tx.id()) {println!("duplicate tx"); false;}
        else if tx.sig.is_none() {println!("No sig"); false;}
        else if !tx.valid_signature() {println!("Invalid Sig"); false;}
        else if !tx.sufficient_funds(self) {println!("Insufficient Funds"); false;}

        let nonce = match self.next_nonce.get_mut(&tx.from){
            Some(nonce) => nonce,
            None => {
                self.next_nonce.insert(tx.from.clone(), 0);
                self.next_nonce.get_mut(&tx.from).unwrap()
            },
        };
        //replayed transaction
        if tx.nonce < *nonce {println!("replayed tx"); false;}
        //out of order tx
        else if tx.nonce > *nonce {println!("out of order tx");return false;}
        else {*nonce += 1}

        let mut balance = match self.balances.get_mut(&tx.from){
            Some(balance) => balance,
            None => {
                self.balances.insert(tx.from.clone(), 0);
                self.balances.get_mut(&tx.from).unwrap()
            },
        };
        *balance -= tx.total_output();

        for i in 0..tx.outputs.len() {
            let mut oldBalance = match self.balances.get_mut(&tx.outputs[i].0){
                Some(balance) => balance,
                None => {
                    self.balances.insert(tx.outputs[i].0.clone(), 0);
                    self.balances.get_mut(&tx.outputs[i].0).unwrap()
                },
            };
            *oldBalance += tx.outputs[i].1;
        }

        self.transactions.insert(tx.id(), tx);

        return true;
    }

    pub fn rerun(&mut self, prev_block:&Block) -> bool {
        self.balances = reward_coinbase(prev_block);
        self.next_nonce = prev_block.next_nonce.clone();
        let txs = self.transactions.clone();
        self.transactions = BTreeMap::new();

        //in the case there are multiple txs from same address, this will ensure their txs arent out of order
        let mut txs_sorted_by_nonce:Vec<Transaction> = txs.into_values().collect();
        txs_sorted_by_nonce.sort_by_key(|tx| tx.nonce);
        for tx in txs_sorted_by_nonce {
            let success = self.add_transaction(tx);
            if !success {return false}
        }
        true
    }

    pub fn total_rewards(&self) {
        let mut total = 0;
        for (_, tx) in &self.transactions {
            total+=tx.fee
        }
    }

    pub fn contains(&self, tx:&Transaction) -> bool {
        self.transactions.contains_key::<Hash>(&tx.id())
    }

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

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn hash_val(&self) -> Hash {
        Hash(hash(self.serialize().as_bytes()).to_vec())
    }
}