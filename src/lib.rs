extern crate core;

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::time::{SystemTime, UNIX_EPOCH };
use hex::{encode, ToHex};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct Hash(Vec<u8>);
impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&*encode(self.as_hex()))
    }
}
impl Deref for Hash {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Hash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Hash {
    pub fn as_hex(&self) -> String{
        encode(<Vec<u8> as AsRef<[u8]>>::as_ref(self))
    }
}
type Address = String;

#[derive(Clone, Copy)]
pub struct SigWrapper(Signature);

impl Serialize for SigWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&*encode(self.as_ref()))
    }
}

impl Deref for SigWrapper {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SigWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub const DEFAULT_MINING_ROUNDS:usize = 3000;
pub const DEFAULT_FEE:u32 = 1;
pub const COINBASE_REWARD:u16 = 25;
pub const CONFIRMED_DEPTH:u8 = 2;
const POW_LEADING_ZEROS:usize = 3;

pub fn calc_pow_target () -> Hash {
    let mut pow_target:Hash = Hash(vec![0xff;32]);
    for i in 0..POW_LEADING_ZEROS/2 {
        pow_target[i] = 0x00;
    }
    if POW_LEADING_ZEROS % 2 != 0 {pow_target[POW_LEADING_ZEROS/2] = 0x0f};
    pow_target
}

pub fn now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u16_to_bytes (u: &u16) -> [u8; 2] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
    ]
}

pub fn u32_to_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_to_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8
    ]
}

pub fn u128_to_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8
    ]
}

mod block;
pub use crate::block::Block;
mod utils;
pub use crate::utils::*;
mod transaction;
mod client;
pub use crate::client::Client;
mod miner;
pub use crate::miner::Miner;
pub use crate::transaction::Transaction;
pub use ring::{digest, rand, signature::{self, Signature, KeyPair, Ed25519KeyPair}};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
