use base64::{encode, encode_config};
use ring::{digest, rand, signature::{self, KeyPair, Ed25519KeyPair}};
use crate::Address;

extern crate base64;

pub fn generate_keypair() -> Ed25519KeyPair {
    // Generate a key pair in PKCS#8 (v2) format.
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap()
}

pub fn hash(bytes: &[u8]) -> Vec<u8> {
    digest::digest(&digest::SHA256,bytes).as_ref().to_vec()
}

pub fn calc_address(pub_key: &[u8]) -> Address {
    encode_config(pub_key,base64::STANDARD)
}

fn test() {
    let keypair:Ed25519KeyPair = generate_keypair();

    const MESSAGE: &[u8] = b"hello, world";

    let sig = keypair.sign(MESSAGE);

    let pub_key_bytes = keypair.public_key().as_ref();

    let pub_key = signature::UnparsedPublicKey::new(&signature::ED25519, pub_key_bytes);

    if pub_key.verify(b"hello world", sig.as_ref()).is_ok() {
        println!("Passed!");
    }
    else {
        println!("failed");
    }
}





// Normally the application would store the PKCS#8 file persistently. Later
// it would read the PKCS#8 file from persistent storage to use it.


/*
// Sign the message "hello, world".
const MESSAGE: &[u8] = b"hello, world";
let sig = key_pair.sign(MESSAGE);

// Normally an application would extract the bytes of the signature and
// send them in a protocol message to the peer(s). Here we just get the
// public key key directly from the key pair.
let peer_public_key_bytes = key_pair.public_key().as_ref();

// Verify the signature of the message using the public key. Normally the
// verifier of the message would parse the inputs to this code out of the
// protocol message(s) sent by the signer.
let peer_public_key =
signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
peer_public_key.verify(MESSAGE, sig.as_ref())?;

*/