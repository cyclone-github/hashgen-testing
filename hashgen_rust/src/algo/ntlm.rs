// src/algo/ntlm.rs

use hex;
use md4::{Md4, Digest};

/// NTLM = MD4 hash of UTF-16LE encoding input string
/// returns `None` if the input is not valid UTF-8 (sanity check)
pub fn ntlm_hex(line: &[u8]) -> Option<String> {
    // interpret as UTF-8
    let s = std::str::from_utf8(line).ok()?;
    // convert to UTF-16LE
    let utf16: Vec<u16> = s.encode_utf16().collect();

    let mut md4 = Md4::new();
    for w in utf16 {
        md4.update(&w.to_le_bytes());
    }

    Some(hex::encode(md4.finalize()))
}
