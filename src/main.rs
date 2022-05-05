use ring::signature;
use ring::signature::{Ed25519KeyPair, KeyPair};

mod utils;

fn main() {
    let keypair:Ed25519KeyPair = utils::generate_keypair();

    const MESSAGE: &[u8] = b"hello, world";

    let sig = keypair.sign(MESSAGE);

    let pubKeyBytes = keypair.public_key().as_ref();

    let pubKey = signature::UnparsedPublicKey::new(&signature::ED25519, pubKeyBytes);

    if pubKey.verify(b"hello world", sig.as_ref()).is_ok() {
        println!("Passed!");
    }
    else {
        println!("failed");
    }
}
