// src/algo/wpbcrypt.rs

use anyhow::anyhow;
use base64::engine::general_purpose::STANDARD as B64_STD;
use base64::Engine as _;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha384;

/// WordPress bcrypt(HMAC-SHA384) -> $wp$2y$...
pub fn wpbcrypt(pass: &[u8], cost: u32) -> anyhow::Result<String> {
    type HmacSha384 = Hmac<Sha384>;

    // HMAC-SHA384 with "wp-sha384" as key
    let mut mac = HmacSha384::new_from_slice(b"wp-sha384").unwrap();
    mac.update(pass);
    let tag = mac.finalize().into_bytes(); // 48 bytes

    // base64 encode tag
    let b64 = B64_STD.encode(&tag);

    // generate random salt
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);

    // bcrypt using base64 HMAC
    let hp = bcrypt::hash_with_salt(&b64, cost, salt).map_err(|e| anyhow!("{e:?}"))?;
    let s = hp.to_string(); // "$2b$..$.."

    Ok(format!("$wp$2y${}", &s[4..]))
}
