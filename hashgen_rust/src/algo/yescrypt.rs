// src/algo/yescrypt.rs

use crate::consts::CRYPT64;
use anyhow::Result;
use rand::RngCore;

/// generate 16-char salt using crypt64 alphabet
fn rand_crypt64_salt16() -> [u8; 16] {
    let mut r = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut r);
    let mut out = [0u8; 16];
    for (i, b) in r.iter().enumerate() {
        out[i] = CRYPT64[(b & 0x3F) as usize];
    }
    out
}

/// hash 'pass' with Debian default yescrypt parameters
pub fn yescrypt_mcf(pass: &[u8]) -> Result<String> {
    let salt = rand_crypt64_salt16();
    let hash = yescrypt::yescrypt(pass, &salt, &Default::default())
        .map_err(|e| anyhow::anyhow!("yescrypt error: {e:?}"))?;
    Ok(hash)
}
