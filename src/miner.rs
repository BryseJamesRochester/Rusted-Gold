use std::fmt::format;
use std::ops::Add;
use ring::signature::Ed25519KeyPair;
use crate::{Address, Block, Blockchain, Client, Hash, Transaction};
use crate::blockchain::DEFAULT_MINING_ROUNDS;

pub struct Miner {
    pub client: Client,
    transactions: Vec<Transaction>,
    current_block:Option<Block>,
    mining_rounds:usize
}

impl Miner {
    pub fn new(name: String, starting_block:Option<Block>, keypair:Option<Ed25519KeyPair>, mining_rounds:Option<usize>) -> Self {
        Miner {
            client: Client::new(name, starting_block, keypair),
            transactions: vec![],
            current_block: None,
            mining_rounds: mining_rounds.unwrap_or(DEFAULT_MINING_ROUNDS)
        }
    }

    pub fn initialize (&mut self) {
        self.start_new_search(None);
    }

    //inefficient
    pub fn start_new_search (&mut self, tx_set: Option<Vec<Transaction>>) {
        if self.last_block().is_some() {
            self.current_block = Some(Blockchain::make_block(self.address(), &self.last_block().unwrap()));
            if tx_set.is_some() {
                for tx in tx_set.unwrap().iter() {
                    self.transactions.push(tx.clone());
                }
            }
            for tx in &self.transactions{
                self.current_block.as_mut().unwrap().add_transaction(tx.clone());
            }
            self.transactions.clear();
            self.current_block.as_mut().unwrap().proof = 0;
        }
        else {
            panic!("Trying to mine without a genesis block.")
        }
    }

    pub fn find_proof(&mut self) {
        if self.current_block.is_some() {
            //let pause_point = self.current_block.as_ref().unwrap().proof + self.mining_rounds as u128;
            let pause_point = u128::MAX;
            while self.current_block.as_ref().unwrap().proof < pause_point {
                if self.current_block.as_ref().unwrap().has_valid_proof() {
                    self.log(&format!("found proof for block {}: {}", self.current_block.as_ref().unwrap().chain_length, self.current_block.as_ref().unwrap().proof));
                    //self.announce_proof();
                    self.receive_block(self.current_block.as_ref().unwrap().clone());
                    break;
                }
                self.current_block.as_mut().unwrap().proof += 1;
            }
        }
        else { panic!("trying to find proof before setting current block"); }

    }

    fn announce_proof () {
        todo!()
    }

    fn receive_block (&mut self, incoming_block:Block) -> Option<Block> {

        match self.client.receive_block(incoming_block) {
            None => return None,
            Some(block) => {
                let current_block = self.current_block.as_ref();
                if current_block.is_some() && current_block.unwrap().chain_length <= block.chain_length {
                    self.log("Cutting over to new chain.");
                    let tx_set:Option<Vec<Transaction>>;
                    let unincluded_txs = self.sync_transactions(block.clone());
                    if unincluded_txs.is_empty() {tx_set = None}
                    else {tx_set = Some(unincluded_txs)}
                    self.start_new_search(tx_set);
                }
                Some(block)
            }
        }
    }

    fn sync_transactions(&mut self, block: Block) -> Vec<Transaction> {
        let mut new_block = &block;
        let mut current_block:Option<&Block> = Some(self.current_block.as_ref().unwrap());
        let mut new_block_txs:Vec<&Hash> = vec![];
        let mut cur_block_txs:Vec<Transaction> = vec![];

        while new_block.chain_length > current_block.unwrap().chain_length {
            for (id,_) in &new_block.transactions {
                new_block_txs.push(id);
            }
            new_block = self.client.blocks.get(&new_block.prev_block_hash).as_ref().unwrap();
        }

        while current_block.is_some() && current_block.unwrap().id() != new_block.id() {
            for (_,tx) in &current_block.unwrap().transactions {
                cur_block_txs.push(tx.clone());
            }
            for (id,_) in &new_block.transactions {
                new_block_txs.push(id);
            }
            current_block = self.client.blocks.get(&current_block.unwrap().prev_block_hash);
            new_block = self.client.blocks.get(&new_block.prev_block_hash).unwrap();
        }

        cur_block_txs.retain(|tx| new_block_txs.contains(&&tx.id()));
        cur_block_txs


    }

    pub fn add_transaction(&mut self, tx:Transaction) {
        //will need to deserialize when network implemented
        self.transactions.push(tx);
    }

    pub fn post_transaction(&mut self, outputs:Vec<(Address, u128)>, custom_fee:Option<u32>) {
        let tx = self.client.post_transaction(outputs, custom_fee);
        if tx.is_some() {
            self.add_transaction(tx.unwrap())
        }
    }

    pub fn log(&self, msg:&str) {
        self.client.log(msg);
    }

    pub fn address(&self) -> Address {
        self.client.address()
    }

    fn last_block(&self) -> Option<Block> {
        self.client.last_block()
    }

    fn last_confirmed_block(&self) -> Option<Block> {
        self.client.last_confirmed_block()
    }
}