use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::{default, result};
use std::error::Error;
use std::ops::Add;
use std::ptr::null;
use base64::encode;
use ring::signature::{Ed25519KeyPair, KeyPair};
use crate::{Address, Block, calc_address, CONFIRMED_DEPTH, DEFAULT_FEE, generate_keypair, Hash, Transaction};

pub struct Client {
    pub keypair:Ed25519KeyPair,
    pub name: String,
    nonce: u128,
    pending_outgoing_transactions: BTreeMap<Hash, Transaction>,
    pending_received_transactions: BTreeMap<Hash, Transaction>,
    pub blocks:BTreeMap<Hash, Block>,
    pendingBlocks: BTreeMap<Hash, Block>,
    last_confirmed_block_id: Option<Hash>,
    last_block_id: Option<Hash>

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
            pendingBlocks: BTreeMap::new(),
            last_confirmed_block_id: None,
            last_block_id: None
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
    pub fn post_transaction(&mut self, outputs:Vec<(Address, u128)>, fee:Option<u32>) -> Option<Transaction> {
        let mut tx = Transaction::new(
            self.address(),
            self.nonce,
            self.pub_key_bytes(),
            outputs,
            fee.unwrap_or(DEFAULT_FEE),
            "".to_string()
        );
        //println!("{}'s available gold: {}. tx total output: {}", self.name, self.available_gold(), tx.total_output());
        if self.available_gold() > tx.total_output(){
            tx.sign(&self.keypair);
            self.pending_outgoing_transactions.insert(tx.id(), tx.clone());
            self.nonce+=1;
            Some(tx)
        }
        else {
            None
        }
    }

    //recieve block

    //request missing block

    //resend pending transactions

    //provide missing block

    fn set_last_confirmed(&mut self) {
        if self.blocks.is_empty() {
            panic!("Trying to set last confirmed on empty blockchain");
        }
        let mut block = self.last_block().unwrap();
        let mut confirmed_block_height = block.chain_length - CONFIRMED_DEPTH as u32;
        if confirmed_block_height < 0 {
            confirmed_block_height = 0;
        }
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
                    self.log(&format!("{:?}", block.unwrap().id()));
                    block = self.blocks.get(&block.unwrap().prev_block_hash);
                }
            },
            None => { self.log("Empty Blockchain") }
        }

    }

}