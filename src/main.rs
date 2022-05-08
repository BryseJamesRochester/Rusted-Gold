use std::collections::HashMap;
use ring::signature;
use ring::signature::{Ed25519KeyPair, KeyPair};
use project_rusted_gold::*;
use hex::{encode, decode};




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
    println!("balance of 'address':{}", gen_block.balance_of(&"address".to_string()));
    println!("balance of 'none':{}", gen_block.balance_of(&"none".to_string()));



}
