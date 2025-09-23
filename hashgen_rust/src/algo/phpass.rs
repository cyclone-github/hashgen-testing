// src/algo/phpass.rs

use md5::{Digest, Md5};
use rand::Rng;
use std::cmp::min;

use crate::consts::CRYPT64;

/// phpass / phpBB3
pub fn phpass_md5(pass: &[u8], mode: &str, count_log2: i32, salt_raw_opt: Option<&[u8]>) -> String {
    let prefix = if mode == "phpbb3" { b'H' } else { b'P' };
    let cl2 = if count_log2 <= 0 { 11 } else { count_log2 };

    // raw salt (6 bytes)
    let mut salt_raw = [0u8; 6];
    if let Some(s) = salt_raw_opt {
        let take = min(6, s.len());
        salt_raw[..take].copy_from_slice(&s[..take]);
    } else {
        rand::thread_rng().fill(&mut salt_raw);
    }

    // crypt-style base64 encoding
    let encode64 = |src: &[u8], out_len: usize| -> Vec<u8> {
        let mut dst = Vec::with_capacity(out_len);
        let mut i = 0usize;
        while i < src.len() {
            let mut v = src[i] as u32;
            i += 1;
            dst.push(CRYPT64[(v & 0x3f) as usize]);
            if i < src.len() {
                v |= (src[i] as u32) << 8;
            }
            dst.push(CRYPT64[((v >> 6) & 0x3f) as usize]);
            if i >= src.len() {
                break;
            }
            i += 1;
            if i < src.len() {
                v |= (src[i] as u32) << 16;
            }
            dst.push(CRYPT64[((v >> 12) & 0x3f) as usize]);
            if i >= src.len() {
                break;
            }
            i += 1;
            dst.push(CRYPT64[((v >> 18) & 0x3f) as usize]);
            if dst.len() >= out_len {
                break;
            }
        }
        if dst.len() > out_len {
            dst.truncate(out_len);
        }
        dst
    };

    let salt_enc = encode64(&salt_raw, 8);

    // initial hash
    let mut h = Md5::new();
    h.update(&salt_enc);
    h.update(pass);
    let mut sum = h.finalize().to_vec();

    // repeated rounds
    let rounds = 1 << cl2;
    for _ in 0..rounds {
        let mut r = Md5::new();
        r.update(&sum);
        r.update(pass);
        sum = r.finalize().to_vec();
    }

    let digest_enc = encode64(&sum, 22);

    // final string
    let mut out = Vec::with_capacity(3 + 1 + 8 + 22);
    out.push(b'$');
    out.push(prefix);
    out.push(b'$');
    out.push(CRYPT64[(cl2 & 0x3f) as usize]);
    out.extend_from_slice(&salt_enc);
    out.extend_from_slice(&digest_enc);

    String::from_utf8(out).unwrap()
}
