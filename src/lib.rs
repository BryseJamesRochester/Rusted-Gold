use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH };

type Hash = Vec<u8>;
type Address = String;





const POW_LEADING_ZEROS:usize = 2;

pub fn calc_pow_target () -> Hash {
    let mut pow_target:Hash = vec![0xff;32];
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