// src/algo/sha512crypt.rs

use rand::Rng;
use sha2::{Digest, Sha512};
use std::cmp::min;

use crate::consts::CRYPT64;

/// sha512crypt ($6$), rounds=5000 (Linux shadow)
pub fn sha512crypt(password: &[u8]) -> String {
    const MAGIC: &str = "$6$";
    const ROUNDS: usize = 5000;

    // salt (16 chars, crypt64 alphabet)
    let mut rb = [0u8; 16];
    rand::thread_rng().fill(&mut rb);
    let mut salt = [0u8; 16];
    for i in 0..16 {
        salt[i] = CRYPT64[(rb[i] & 0x3f) as usize];
    }

    // initial digest
    let mut a = Sha512::new();
    a.update(password);
    a.update(&salt);

    // alternate sum
    let mut alt = Sha512::new();
    alt.update(password);
    alt.update(&salt);
    alt.update(password);
    let alt_sum = alt.finalize().to_vec();

    let key_len = password.len();
    if key_len > 0 {
        let n = key_len / 64;
        for _ in 0..n {
            a.update(&alt_sum);
        }
        a.update(&alt_sum[..(key_len % 64)]);
    }

    // mix length-based bits
    let mut cnt = key_len;
    while cnt > 0 {
        if (cnt & 1) != 0 {
            a.update(&alt_sum);
        } else {
            a.update(password);
        }
        cnt >>= 1;
    }
    let mut finalv = a.finalize().to_vec();

    // DP (repeat password)
    let mut dp = Sha512::new();
    for _ in 0..key_len {
        dp.update(password);
    }
    let dp_sum = dp.finalize().to_vec();
    let mut p = vec![0u8; key_len];
    for i in (0..key_len).step_by(64) {
        let end = min(i + 64, key_len);
        p[i..end].copy_from_slice(&dp_sum[..end - i]);
    }

    // DS (repeat salt)
    let mut ds = Sha512::new();
    for _ in 0..(16 + finalv[0] as usize) {
        ds.update(&salt);
    }
    let ds_sum = ds.finalize().to_vec();
    let mut s = vec![0u8; salt.len()];
    for i in (0..salt.len()).step_by(64) {
        let end = min(i + 64, salt.len());
        s[i..end].copy_from_slice(&ds_sum[..end - i]);
    }

    // rounds
    for i in 0..ROUNDS {
        let mut c = Sha512::new();
        if (i & 1) != 0 {
            c.update(&p);
        } else {
            c.update(&finalv);
        }
        if i % 3 != 0 {
            c.update(&s);
        }
        if i % 7 != 0 {
            c.update(&p);
        }
        if (i & 1) != 0 {
            c.update(&finalv);
        } else {
            c.update(&p);
        }
        finalv = c.finalize().to_vec();
    }

    // output encoding
    let mut out = Vec::with_capacity(MAGIC.len() + salt.len() + 1 + 86);
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
    let f = &finalv;
    enc(f[0], f[21], f[42], 4);
    enc(f[22], f[43], f[1], 4);
    enc(f[44], f[2], f[23], 4);
    enc(f[3], f[24], f[45], 4);
    enc(f[25], f[46], f[4], 4);
    enc(f[47], f[5], f[26], 4);
    enc(f[6], f[27], f[48], 4);
    enc(f[28], f[49], f[7], 4);
    enc(f[50], f[8], f[29], 4);
    enc(f[9], f[30], f[51], 4);
    enc(f[31], f[52], f[10], 4);
    enc(f[53], f[11], f[32], 4);
    enc(f[12], f[33], f[54], 4);
    enc(f[34], f[55], f[13], 4);
    enc(f[56], f[14], f[35], 4);
    enc(f[15], f[36], f[57], 4);
    enc(f[37], f[58], f[16], 4);
    enc(f[59], f[17], f[38], 4);
    enc(f[18], f[39], f[60], 4);
    enc(f[40], f[61], f[19], 4);
    enc(f[62], f[20], f[41], 4);

    let mut v = f[63] as u32;
    for _ in 0..2 {
        out.push(CRYPT64[(v & 0x3f) as usize]);
        v >>= 6;
    }

    String::from_utf8(out).unwrap()
}
