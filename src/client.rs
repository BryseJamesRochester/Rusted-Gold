use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::{default, result};
use std::error::Error;
use std::fmt::format;
use std::ops::Add;
use std::ptr::null;
use base64::encode as base64;
use hex::{encode, decode};
use ring::signature::{Ed25519KeyPair, KeyPair};
use crate::{Address, Block, calc_address, CONFIRMED_DEPTH, DEFAULT_FEE, generate_keypair, Hash, Transaction};

pub struct Client {
    pub keypair:Ed25519KeyPair,
    pub name: String,
    nonce: u128,
    pending_outgoing_transactions: BTreeMap<Hash, Transaction>,
    pending_received_transactions: BTreeMap<Hash, Transaction>,
    pub blocks:BTreeMap<Hash, Block>,
    last_confirmed_block_id: Option<Hash>,
    last_block_id: Option<Hash>,
    pending_blocks: BTreeMap<Hash, Vec<Block>>

}

impl Default for Client {
    fn default() -> Self {
        Client {
            keypair:generate_keypair(),
            name:String::from(""),
            nonce: 0,
            pending_outgoing_transactions: BTreeMap::new(),
            pending_received_transactions: BTreeMap::new(),
            blocks: BTreeMap::new(),
            last_confirmed_block_id: None,
            last_block_id: None,
            pending_blocks:BTreeMap::new()
        }
    }
}

impl Client {
    pub fn new(name:String, starting_block:Option<Block>, keypair:Option<Ed25519KeyPair>) -> Self {
        let mut client = Client{
            keypair: {
                if keypair.is_some() {keypair.unwrap()}
                else {generate_keypair()}
            },
            name,
            ..Default::default()
        };
        if starting_block.is_some() {
            client.set_genesis(starting_block.unwrap());
        }
        client
    }

    pub fn set_genesis(&mut self,starting_block: Block) {
        if !self.last_block_id.is_none() {
            panic!("Trying to set_genesis on existing blockchain")
        }
        if !self.blocks.is_empty() {
            panic!("Trying to set_genesis on existing blockchain")
        }
        self.blocks.insert(starting_block.id(), starting_block.clone());
        self.last_block_id = Some(starting_block.id());
        self.last_confirmed_block_id = Some(starting_block.id());
    }

    pub fn last_block(&self) -> Option<Block> {
        if self.last_block_id.is_some() {
            Some(self.blocks.get(&self.last_block_id.as_ref().unwrap()).unwrap().clone())
        }
        else { None }

    }

    pub fn last_confirmed_block(&self) -> Option<Block> {
        if self.last_confirmed_block_id.is_some() {
            Some(self.blocks.get(&self.last_confirmed_block_id.as_ref().unwrap()).unwrap().clone())
        }
        else { None }
    }

    pub fn confirmed_balance(&self) -> u128 {
        let last_confirmed_block = self.last_confirmed_block();
        if last_confirmed_block.is_some() {
            last_confirmed_block.unwrap().balance_of(&self.address())
        }
        else { 0 }
    }

    pub fn available_gold(&self) -> u128 {
        let mut pending_spent = 0;
        for (_, tx) in &self.pending_outgoing_transactions{
            pending_spent += tx.total_output();
        }
        self.confirmed_balance() - pending_spent
    }

    fn pub_key_bytes(&self) -> Vec<u8> {
        self.keypair.public_key().as_ref().to_vec()
    }

    pub fn address(&self) -> Address {
        calc_address(self.keypair.public_key().as_ref())
    }

    /** Currently only creates and returns a tx if client has enough gold.
           Will eventually broadcast this transaction to the network
     */
    pub fn post_transaction(&mut self, outputs:Vec<(Address, u128)>, custom_fee:Option<u32>) -> Option<Transaction> {
        let mut fee = custom_fee.unwrap_or(DEFAULT_FEE);
        if fee < DEFAULT_FEE {
            fee = DEFAULT_FEE;
        }
        let mut tx = Transaction::new(
            self.address(),
            self.nonce,
            self.pub_key_bytes(),
            outputs,
            fee,
            "".to_string()
        );
        if self.available_gold() > tx.total_output(){
            tx.sign(&self.keypair);
            self.pending_outgoing_transactions.insert(tx.id(), tx.clone());
            self.nonce+=1;
            Some(tx)
        }
        else {
            self.log(&format!("Insufficient funds. {} gold available, tx total output: {}", self.available_gold(), tx.total_output()));
            None
        }
    }

    pub fn receive_block(&mut self, mut block:Block) -> Option<Block>{
        //will need to deserialize when blocks are communicated through network
        if self.blocks.contains_key(&block.id()) { return None }
        if !block.has_valid_proof() && !block.is_genesis() {
            self.log(&format!("Block {} does not have a valid proof.", encode(&*block.id())));
            return None;
        }
        let prev_block:Option<&Block> = self.blocks.get(&block.prev_block_hash);

        if prev_block.is_none() && !block.is_genesis() {
            let mut stuck_blocks: Option<Vec<Block>> = self.pending_blocks.remove(&block.prev_block_hash); //get(&block.prev_block_hash).as_mut();

            if stuck_blocks.is_none() {
                //request missing block
                stuck_blocks = Some(vec![]);
            }
            let mut new_stuck_blocks = stuck_blocks.unwrap();
            new_stuck_blocks.push(block.clone());
            self.pending_blocks.insert(block.prev_block_hash, new_stuck_blocks.clone());
            return None;
        }

        if !block.is_genesis() {
            let success = block.rerun(prev_block.unwrap());
            if !success { return None}
        }

        //block is good
        self.blocks.insert(block.id(), block.clone());

        let current_length = match self.last_block() {
            Some(last_block) => last_block.chain_length,
            None => 0
        };
        if current_length < block.chain_length {
            self.last_block_id = Some(block.id());
            self.set_last_confirmed();
        }
        let unstuck_blocks:Vec<Block> = self.pending_blocks.remove(&block.id()).unwrap_or(vec![]);
        for unstuck_block in unstuck_blocks {
            self.log(&format!("Processing unstuck block {}", encode(&*block.id())));
            self.receive_block(unstuck_block);
        }
        return Some(block);

    }

    //request missing block

    //resend pending transactions

    //provide missing block

    fn set_last_confirmed(&mut self) {
        if self.blocks.is_empty() {
            panic!("Trying to set last confirmed on empty blockchain");
        }
        let mut block = self.last_block().unwrap();
        let confirmed_block_height = block.chain_length.checked_sub(CONFIRMED_DEPTH as u32).unwrap_or(0);
        while block.chain_length > confirmed_block_height {
            block = self.blocks.get(&block.prev_block_hash).unwrap().clone();
        }

        self.pending_outgoing_transactions.retain( |id,_| !block.contains(id));

        self.pending_received_transactions.retain( |id,_| !block.contains(id));

        self.last_confirmed_block_id = Some(block.id());
    }

    pub fn show_all_balances(&self) {
        self.log("Showing Balances:");
        let last_confirmed_block = self.last_confirmed_block();
        if last_confirmed_block.is_some(){
            for (addr, balance) in &last_confirmed_block.unwrap().balances {
                println!("{}: {}", addr, balance);
            }
        }

    }

    pub fn log(&self, msg: &str) {
        println!("{}: {}", self.name, msg);
    }

    pub fn show_blockchain(&self) {
        match self.last_block() {
            Some(block) => {
                let mut block = Some(&block);
                self.log("BLOCKCHAIN:");
                while block.is_some() {
                    self.log(&format!("{:?}", encode(&*block.unwrap().id())));
                    block = self.blocks.get(&block.unwrap().prev_block_hash);
                }
            },
            None => { self.log("Empty Blockchain") }
        }

    }

}