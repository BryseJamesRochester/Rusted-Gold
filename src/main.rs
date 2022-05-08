use std::collections::HashMap;
use ring::signature;
use ring::signature::{Ed25519KeyPair, KeyPair};
use project_rusted_gold::*;
use hex::{encode, decode};
extern crate project_rusted_gold;




fn main() {
    let target = calc_pow_target();
    println!("target:{:?}", target);
    let gen_block = Block{balances:HashMap::from([("address".to_owned(), 20)]), ..Default::default()};
    println!("gen block hash:{:?}", encode(gen_block.hash_val()));
    println!("balance of 'address':{}", gen_block.balance_of(&"address".to_string()));
    println!("balance of 'none':{}", gen_block.balance_of(&"none".to_string()));
    let mut block1 = Block::new("0".to_owned(), 0, &gen_block);
    println!("block1 hash: {:?}", encode(block1.hash_val()));
    println!("valid proof:{:?}", block1.has_valid_proof());
    let nonce = block1.mine();
    println!("nonce: {}", nonce);
    println!("block1 post-mine hash: {:?}", encode(block1.hash_val()));
    println!("valid proof post-mine:{:?}", block1.has_valid_proof());
    println!("block1 balance of 'address':{}", block1.balance_of(&"address".to_string()));
    println!("block1 balance of 'none':{}", block1.balance_of(&"none".to_string()));
    let keypair = generate_keypair();
    let mut tx = Transaction::new("address".to_string(), 0,keypair.public_key().as_ref().to_vec(), vec![("0".to_string(), 15)], 5, "".to_string());
    println!("tx id: {:?}", tx.id());
    tx.sign(keypair);
    println!("valid sig:{}", tx.valid_signature());
    println!("tx total output:{}", tx.total_output());
    println!("tx sufficient funds:{}", tx.sufficient_funds(&block1));


}
