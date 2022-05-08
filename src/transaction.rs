use ring::agreement::PublicKey;
use ring::signature::{Signature, UnparsedPublicKey};
use super::*;

pub struct Transaction {
    from:Address,
    nonce:u128,
    pubkey_bytes:Vec<u8>,
    sig:Option<Signature>,
    outputs:Vec<(Address, u128)>,
    pub fee: u32,
    data: String
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

    fn serialize (&self) -> String {
        format!("{}||{}||{:?}||{:?}||{}||{}",
            self.from,
            self.nonce,
            self.pubkey_bytes,
            self.outputs,
            self.fee,
            self.data
        )
    }

    pub fn id(&self) -> Hash {
        hash(self.serialize().as_bytes())
    }

    pub fn sign(&mut self, keypair:Ed25519KeyPair) -> () {
        self.sig = Some(keypair.sign(self.id().as_ref()))
    }

    pub fn valid_signature(&self) -> bool {
        return match self.sig {
            Some(sig) => {
                let pubkey = signature::UnparsedPublicKey::new(&signature::ED25519, &self.pubkey_bytes);
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