// src/algo/blake2.rs

use blake2b_simd::Params as Blake2bParams;
use blake2s_simd::Params as Blake2sParams;
use hex;

/// BLAKE2s-256 fast
pub fn blake2s256_hex(data: &[u8]) -> String {
    let p = Blake2sParams::new();
    let h = p.hash(data);
    hex::encode(h.as_bytes())
}

/// BLAKE2b variable output via blake2b_simd
pub fn blake2b_var_hex(data: &[u8], out_len: usize) -> String {
    let mut p = Blake2bParams::new();
    p.hash_length(out_len);
    let h = p.hash(data);
    hex::encode(h.as_bytes())
}
