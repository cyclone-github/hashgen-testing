// src/algo/keccak.rs

use tiny_keccak::{Keccak, Hasher};

// keccak helpers
pub fn keccak_224_hex(data: &[u8]) -> String {
    let mut k = Keccak::v224();
    k.update(data);
    let mut out = [0u8; 28];
    k.finalize(&mut out);
    hex::encode(out)
}
pub fn keccak_256_hex(data: &[u8]) -> String {
    let mut k = Keccak::v256();
    k.update(data);
    let mut out = [0u8; 32];
    k.finalize(&mut out);
    hex::encode(out)
}
pub fn keccak_384_hex(data: &[u8]) -> String {
    let mut k = Keccak::v384();
    k.update(data);
    let mut out = [0u8; 48];
    k.finalize(&mut out);
    hex::encode(out)
}
pub fn keccak_512_hex(data: &[u8]) -> String {
    let mut k = Keccak::v512();
    k.update(data);
    let mut out = [0u8; 64];
    k.finalize(&mut out);
    hex::encode(out)
}
