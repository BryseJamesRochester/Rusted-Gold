use std::collections::{BTreeMap, HashMap};
use std::fs;
use ring::signature;
use ring::signature::{Ed25519KeyPair, KeyPair};
use project_rusted_gold::*;
use hex::{encode, decode};
extern crate project_rusted_gold;



fn gen_and_store_key() {
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    fs::write("./key", pkcs8_bytes.as_ref()).expect("Unable to write file");
}

fn get_key() -> Ed25519KeyPair {
    let data = fs::read("./key").expect("Unable to read file");
    Ed25519KeyPair::from_pkcs8(data.as_ref()).unwrap()
}

fn main() {
    //gen_and_store_key();
    let keypair = get_key();
    //let keypair = generate_keypair();
    let target = calc_pow_target();
    println!("target:{}", encode(&*target));
    let gen_block = Block{
        balances:BTreeMap::from([("address".to_owned(), 20)]),
        ..Default::default()};
    println!("gen block hash:{}", encode(&*gen_block.hash_val()));
    // println!("balance of 'address':{}", gen_block.balance_of(&"address".to_string()));
    // println!("balance of 'none':{}", gen_block.balance_of(&"none".to_string()));
    let mut block1 = Block::new(
        "0".to_owned(),
        10,
        BTreeMap::new(),
        &gen_block
    );
    //println!("pubkey: {:?}", keypair.public_key().as_ref().to_owned());
    let mut tx = Transaction::new(
        "address".to_string(),
        0,
        keypair.public_key().as_ref().to_vec(),
        vec![("none".to_string(), 5)],
        0,
        "".to_string()
    );
    //println!("tx id: {}", encode(&*tx.id()));
    tx.sign(&keypair);
    //println!("valid sig:{}", tx.valid_signature());
    //println!("tx total output:{}", tx.total_output());
    //println!("tx sufficient funds:{}", tx.sufficient_funds(&block1));
    let mut tx1 = Transaction::new(
        "address".to_string(),
        1,
        keypair.public_key().as_ref().to_vec(),
        vec![("hello".to_string(), 5)],
        0,
        "".to_string()
    );
    tx1.sign(&keypair);
    let mut tx2 = Transaction::new(
        "address".to_string(),
        2,
        keypair.public_key().as_ref().to_vec(),
        vec![("hello".to_string(), 2)],
        0,
        "".to_string()
    );
    tx2.sign(&keypair);
    let tx_clone = tx.clone();
    let tx1_clone = tx1.clone();
    let tx2_clone = tx2.clone();
    block1.add_transaction(tx_clone);
    block1.add_transaction(tx1_clone);
    block1.add_transaction(tx2_clone);
    println!("block1 contains tx:{:?}, tx1:{:?}, tx2{:?}",
             block1.contains(&tx),
             block1.contains(&tx1),
             block1.contains(&tx2)
    );
    println!("block1 balance of 'address':{}, 'none':{}, 'hello':{}, '0':{}",
             block1.balance_of(&"address".to_string()),
             block1.balance_of(&"none".to_string()),
             block1.balance_of(&"hello".to_string()),
             block1.balance_of(&"0".to_string())
    );
    println!("block1 hash: {}", encode(&*block1.hash_val()));
    println!("valid proof:{}", block1.has_valid_proof());
    let nonce = block1.mine();
    println!("nonce: {}", nonce);
    println!("block1 post-mine hash: {}", encode(&*block1.hash_val()));
    println!("rerun success:{}", block1.rerun(&gen_block));
    println!("block1 balance of 'address':{}, 'none':{}, 'hello':{}, '0':{}",
             block1.balance_of(&"address".to_string()),
             block1.balance_of(&"none".to_string()),
             block1.balance_of(&"hello".to_string()),
             block1.balance_of(&"0".to_string())
    );




}
