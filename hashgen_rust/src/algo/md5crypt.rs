// src/algo/md5crypt.rs

use md5::{Digest, Md5};
use rand::Rng;
use std::cmp::min;

use crate::consts::CRYPT64;

/// MD5 crypt ($1$) (Linux shadow)
pub fn md5crypt(password: &[u8]) -> String {
    const MAGIC: &str = "$1$";

    // generate 8-char salt from crypt64 set
    let mut rb = [0u8; 8];
    rand::thread_rng().fill(&mut rb);
    let mut salt = [0u8; 8];
    for i in 0..8 {
        salt[i] = CRYPT64[(rb[i] & 0x3f) as usize];
    }

    // initial digest
    let mut a = Md5::new();
    a.update(password);
    a.update(MAGIC.as_bytes());
    a.update(&salt);

    let mut alt = Md5::new();
    alt.update(password);
    alt.update(&salt);
    alt.update(password);
    let alt_sum = alt.finalize();

    // mix in
    let pw_len = password.len();
    let mut n = pw_len;
    while n > 0 {
        a.update(&alt_sum[..min(16, n)]);
        if n > 16 {
            n -= 16;
        } else {
            break;
        }
    }

    let mut n2 = pw_len;
    while n2 > 0 {
        if (n2 & 1) == 1 {
            a.update(&[0u8]);
        } else if pw_len > 0 {
            a.update(&password[..1]);
        } else {
            a.update(&[0u8]);
        }
        n2 >>= 1;
    }

    let mut finalv = a.finalize().to_vec();

    // 1000 rounds of mixing
    for i in 0..1000 {
        let mut r = Md5::new();
        if (i & 1) == 1 {
            r.update(password);
        } else {
            r.update(&finalv);
        }
        if i % 3 != 0 {
            r.update(&salt);
        }
        if i % 7 != 0 {
            r.update(password);
        }
        if (i & 1) == 1 {
            r.update(&finalv);
        } else {
            r.update(password);
        }
        finalv = r.finalize().to_vec();
    }

    // output encoding
    let mut out = Vec::with_capacity(MAGIC.len() + salt.len() + 1 + 22);
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
    enc(finalv[0], finalv[6], finalv[12], 4);
    enc(finalv[1], finalv[7], finalv[13], 4);
    enc(finalv[2], finalv[8], finalv[14], 4);
    enc(finalv[3], finalv[9], finalv[15], 4);
    enc(finalv[4], finalv[10], finalv[5], 4);
    let mut v = (finalv[11] as u32) & 0xff;
    for _ in 0..2 {
        out.push(CRYPT64[(v & 0x3f) as usize]);
        v >>= 6;
    }

    String::from_utf8(out).unwrap()
}
