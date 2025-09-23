// src/algo/sha256crypt.rs

use rand::Rng;
use sha2::{Digest, Sha256};
use std::cmp::min;

use crate::consts::CRYPT64;

/// sha256crypt ($5$) implementation, with 5000 rounds (Linux shadow)
pub fn sha256crypt(password: &[u8]) -> String {
    const MAGIC: &str = "$5$";
    let rounds = 5000usize;

    // salt (16 chars, crypt64 alphabet)
    let mut rb = [0u8; 16];
    rand::thread_rng().fill(&mut rb);
    let mut salt = [0u8; 16];
    for i in 0..16 {
        salt[i] = CRYPT64[(rb[i] & 0x3f) as usize];
    }

    // initial digest
    let mut a = Sha256::new();
    a.update(password);
    a.update(&salt);

    // alternate sum
    let mut alt = Sha256::new();
    alt.update(password);
    alt.update(&salt);
    alt.update(password);
    let alt_sum = alt.finalize().to_vec();

    // mix password length into a
    let pw_len = password.len();
    let mut n = pw_len;
    while n > 0 {
        a.update(&alt_sum[..min(32, n)]);
        n = n.saturating_sub(32);
    }

    // add length-based mix
    let mut n2 = pw_len;
    while n2 > 0 {
        if (n2 & 1) == 1 {
            a.update(&alt_sum);
        } else {
            a.update(password);
        }
        n2 >>= 1;
    }
    let adigest = a.finalize().to_vec();

    // DP mix
    let mut dp = Sha256::new();
    for _ in 0..pw_len {
        dp.update(password);
    }
    let dp_sum = dp.finalize().to_vec();
    let mut p = vec![0u8; pw_len];
    for i in (0..pw_len).step_by(32) {
        let end = min(i + 32, pw_len);
        p[i..end].copy_from_slice(&dp_sum[..end - i]);
    }

    // DS mix
    let mut ds = Sha256::new();
    for _ in 0..(16 + adigest[0] as usize) {
        ds.update(&salt);
    }
    let ds_sum = ds.finalize().to_vec();
    let mut s = vec![0u8; salt.len()];
    for i in (0..salt.len()).step_by(32) {
        let end = min(i + 32, salt.len());
        s[i..end].copy_from_slice(&ds_sum[..end - i]);
    }

    // rounds
    let mut digest = adigest;
    for i in 0..rounds {
        let mut c = Sha256::new();
        if (i & 1) == 1 {
            c.update(&p);
        } else {
            c.update(&digest);
        }
        if i % 3 != 0 {
            c.update(&s);
        }
        if i % 7 != 0 {
            c.update(&p);
        }
        if (i & 1) == 1 {
            c.update(&digest);
        } else {
            c.update(&p);
        }
        digest = c.finalize().to_vec();
    }

    // output encoding
    let mut out = Vec::with_capacity(MAGIC.len() + salt.len() + 1 + 43);
    out.extend_from_slice(MAGIC.as_bytes());
    out.extend_from_slice(&salt);
    out.push(b'$');

    let mut enc = |b2: u8, b1: u8, b0: u8, nrep: usize| {
        let mut v = (b2 as u32) << 16 | (b1 as u32) << 8 | (b0 as u32);
        for _ in 0..nrep {
            out.push(CRYPT64[(v & 0x3f) as usize]);
            v >>= 6;
        }
    };
    enc(digest[0], digest[10], digest[20], 4);
    enc(digest[21], digest[1], digest[11], 4);
    enc(digest[12], digest[22], digest[2], 4);
    enc(digest[3], digest[13], digest[23], 4);
    enc(digest[24], digest[4], digest[14], 4);
    enc(digest[15], digest[25], digest[5], 4);
    enc(digest[6], digest[16], digest[26], 4);
    enc(digest[27], digest[7], digest[17], 4);
    enc(digest[18], digest[28], digest[8], 4);
    enc(digest[9], digest[19], digest[29], 4);

    let mut v = ((digest[31] as u32) << 8) | (digest[30] as u32);
    for _ in 0..3 {
        out.push(CRYPT64[(v & 0x3f) as usize]);
        v >>= 6;
    }

    String::from_utf8(out).unwrap()
}
