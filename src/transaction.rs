use std::fmt::{Debug, Formatter, write};
use hex::encode;
use ring::agreement::PublicKey;
use ring::signature::{Signature, UnparsedPublicKey};
use super::*;
use serde::*;
use serde::ser::Serializer;
use serde_json::*;


#[derive(Clone, Serialize)]
//#[derive(Clone)]
pub struct Transaction {
    pub from:Address,
    pub nonce:u128,
    pub pubkey_bytes:Vec<u8>,
    pub sig:Option<SigWrapper>,
    pub outputs:Vec<(Address, u128)>,
    pub fee: u32,
    pub data: String
}

impl Debug for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}||{}||{}||{:?}||{}||{}",
            &self.from,
            &self.nonce,
            encode(&self.pubkey_bytes),
            &self.outputs,
            &self.fee,
            &self.data
        )
    }
}



impl Transaction {
    pub fn new(from:Address, nonce:u128, pubkey_bytes:Vec<u8>, outputs:Vec<(Address, u128)>, fee: u32, data: String) -> Self {
        Transaction{
            from,
            nonce,
            pubkey_bytes,
            sig:None,
            outputs,
            fee,
            data
        }
    }

    pub fn serialize(&self) -> String {
        let serialized_from = serde_json::to_string(&self.from).unwrap();
        let serialized_nonce = serde_json::to_string(&self.nonce).unwrap();
        let serialized_pubkey_bytes = serde_json::to_string(&self.pubkey_bytes).unwrap();
        let serialized_outputs = serde_json::to_string(&self.outputs).unwrap();
        let serialized_fee = serde_json::to_string(&self.fee).unwrap();
        let serialized_data = serde_json::to_string(&self.data).unwrap();
        format!("{}||{}||{}||{}||{}||{}", serialized_from, serialized_nonce, serialized_pubkey_bytes, serialized_outputs, serialized_fee, serialized_data)
    }

    pub fn id(&self) -> Hash {
        Hash(hash(self.serialize().as_bytes()))
    }

    pub fn sign(&mut self, keypair:&Ed25519KeyPair) -> () {
        self.sig = Some(SigWrapper(keypair.sign(self.id().as_ref())));
    }

    pub fn valid_signature(&self) -> bool {
        return match &self.sig {
            Some(sig) => {
                let pubkey = UnparsedPublicKey::new(&signature::ED25519, &self.pubkey_bytes);
                if pubkey.verify(self.id().as_ref(), sig.as_ref()).is_ok() { true } else { false }
            }
            None => { false }
        }
    }

    pub fn sufficient_funds(&self, block:&Block) -> bool {
        self.total_output() <= block.balance_of(&self.from)
    }

    pub fn total_output(&self) -> u128 {
        let mut sum:u128 = 0;
        for i in 0..self.outputs.len() {
            sum+=self.outputs[i].1
        }
        sum + u128::from(self.fee)
    }
}