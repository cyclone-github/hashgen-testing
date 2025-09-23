// src/algo/hash_mode.rs

use anyhow::{anyhow, Result};
use base64::engine::general_purpose::STANDARD as B64_STD;
use base64::Engine as _;
use crc32fast::Hasher as Crc32;
use crc64fast::Digest as Crc64;
use data_encoding::BASE32;
//use digest::Digest;
//use md4::Digest as Md4Digest;
use md4::Md4;
//use md5::Digest as Md5Digest;
use md5::Md5;
use rand::Rng;
//use ripemd::Digest as RipemdDigest;
use ripemd::Ripemd160;
use sha1::Digest as Sha1Digest;
use sha1::Sha1;
//use sha2::Digest as Sha2Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
//use sha3::Digest as Sha3Digest;
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
//use std::io::Write;

//pub use crate::algo::hex::check_for_hex;
use crate::consts::help_text;

// bring in other algo helpers from algo/*
use crate::algo::{
    argon::argon2id_tag,
    blake2::{blake2b_var_hex, blake2s256_hex},
    keccak::{keccak_224_hex, keccak_256_hex, keccak_384_hex, keccak_512_hex},
    md5crypt::md5crypt,
    morse::encode_morse,
    ntlm::ntlm_hex,
    phpass::phpass_md5,
    sha256crypt::sha256crypt,
    sha512crypt::sha512crypt,
    wpbcrypt::wpbcrypt,
    yescrypt::yescrypt_mcf,
};

// hash mode

pub fn hash_line(mode: &str, data: &[u8], cost: u32) -> Result<Option<Vec<u8>>> {
    let out = match mode {
        "plaintext" | "dehex" | "99999" => Some(data.to_vec()),

        "hex" => {
            let mut out = Vec::with_capacity(5 + data.len() * 2 + 1);
            out.extend_from_slice(b"$HEX[");
            out.extend_from_slice(hex::encode(data).as_bytes());
            out.push(b']');
            Some(out)
        }

        "base64encode" | "base64e" => Some(B64_STD.encode(data).into_bytes()),
        "base64decode" | "base64d" => {
            let t = trim_ws(data);
            match B64_STD.decode(t) {
                Ok(v) => Some(v),
                Err(_) => {
                    eprintln!("Invalid Base64 string");
                    None
                }
            }
        }

        "base58encode" | "base58e" => Some(bs58::encode(data).into_string().into_bytes()),
        "base58decode" | "base58d" => match bs58::decode(trim_ws(data)).into_vec() {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("Invalid Base58 string: {e}");
                None
            }
        },

        "base32encode" | "base32e" => Some(BASE32.encode(data).into_bytes()),
        "base32decode" | "base32d" => match BASE32.decode(trim_ws(data)) {
            Ok(v) => Some(v),
            Err(_) => {
                eprintln!("Invalid Base32 string");
                None
            }
        },

        "morsecode" | "morse" => Some(encode_morse(data)),

        "crc32" => {
            let mut h = Crc32::new();
            h.update(data);
            Some(hex::encode(h.finalize().to_be_bytes()).into_bytes())
        }
        "11500" => {
            let mut h = Crc32::new();
            h.update(data);
            let mut s = hex::encode(h.finalize().to_be_bytes());
            s.push_str(":00000000");
            Some(s.into_bytes())
        }

        "crc64" => {
            let mut d = Crc64::new();
            let _ = d.write(data);
            let v = d.sum64();
            Some(hex::encode(v.to_be_bytes()).into_bytes())
        }

        "md4" | "900" => {
            let mut m = Md4::new();
            m.update(data);
            Some(hex::encode(m.finalize()).into_bytes())
        }

        "md5" | "0" => {
            let mut m = Md5::new();
            m.update(data);
            Some(hex::encode(m.finalize()).into_bytes())
        }

        "10" | "md5passsalt" | "20" | "md5saltpass" => {
            let salt = make_salt_hex();
            let mut m = Md5::new();
            if mode == "20" || mode == "md5saltpass" {
                m.update(&salt);
                m.update(data);
            } else {
                m.update(data);
                m.update(&salt);
            }
            let mut out = hex::encode(m.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "md5md5" | "2600" => {
            let mut inner = Md5::new();
            inner.update(data);
            let inner = hex::encode(inner.finalize());
            let mut outer = Md5::new();
            outer.update(inner.as_bytes());
            Some(hex::encode(outer.finalize()).into_bytes())
        }

        "sha1" | "100" => {
            let v = Sha1::digest(data);
            Some(hex::encode(v).into_bytes())
        }

        "110" | "sha1passsalt" | "120" | "sha1saltpass" => {
            let salt = make_salt_hex();
            let mut h = Sha1::new();
            if mode == "120" || mode == "sha1saltpass" {
                h.update(&salt);
                h.update(data);
            } else {
                h.update(data);
                h.update(&salt);
            }
            let mut out = hex::encode(h.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "sha1sha1" | "4500" => {
            let inner = Sha1::digest(data);
            let outer = Sha1::digest(&inner);
            Some(hex::encode(outer).into_bytes())
        }

        "sha224" | "sha2-224" | "1300" => {
            let mut h = Sha224::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "1310" | "sha224passsalt" | "1320" | "sha224saltpass" => {
            let salt = make_salt_hex();
            let mut h = Sha224::new();
            if mode == "1320" || mode == "sha224saltpass" {
                h.update(&salt);
                h.update(data);
            } else {
                h.update(data);
                h.update(&salt);
            }
            let mut out = hex::encode(h.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "sha256" | "sha2-256" | "1400" => {
            let mut h = Sha256::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "1410" | "sha256passsalt" | "1420" | "sha256saltpass" => {
            let salt = make_salt_hex();
            let mut h = Sha256::new();
            if mode == "1420" || mode == "sha256saltpass" {
                h.update(&salt);
                h.update(data);
            } else {
                h.update(data);
                h.update(&salt);
            }
            let mut out = hex::encode(h.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "sha384" | "sha2-384" | "10800" => {
            let mut h = Sha384::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "10810" | "sha384passsalt" | "10820" | "sha384saltpass" => {
            let salt = make_salt_hex();
            let mut h = Sha384::new();
            if mode == "10820" || mode == "sha384saltpass" {
                h.update(&salt);
                h.update(data);
            } else {
                h.update(data);
                h.update(&salt);
            }
            let mut out = hex::encode(h.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "sha512" | "sha2-512" | "1700" => {
            let mut h = Sha512::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "1710" | "sha512passsalt" | "1720" | "sha512saltpass" => {
            let salt = make_salt_hex();
            let mut h = Sha512::new();
            if mode == "1720" || mode == "sha512saltpass" {
                h.update(&salt);
                h.update(data);
            } else {
                h.update(data);
                h.update(&salt);
            }
            let mut out = hex::encode(h.finalize()).into_bytes();
            out.push(b':');
            out.extend_from_slice(&salt);
            Some(out)
        }

        "sha512-224" | "sha2-512-224" | "sha512224" => {
            let mut h = Sha512_224::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "sha512-256" | "sha2-512-256" | "sha512256" => {
            let mut h = Sha512_256::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }

        "sha3-224" | "sha3224" | "17300" => {
            let mut h = Sha3_224::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "sha3-256" | "sha3256" | "17400" => {
            let mut h = Sha3_256::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "sha3-384" | "sha3384" | "17500" => {
            let mut h = Sha3_384::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }
        "sha3-512" | "sha3512" | "17600" => {
            let mut h = Sha3_512::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }

        "keccak-224" | "keccak224" | "17700" => Some(keccak_224_hex(data).into_bytes()),
        "keccak-256" | "keccak256" | "17800" => Some(keccak_256_hex(data).into_bytes()),
        "keccak-384" | "keccak384" | "17900" => Some(keccak_384_hex(data).into_bytes()),
        "keccak-512" | "keccak512" | "18000" => Some(keccak_512_hex(data).into_bytes()),

        // BLAKE2b via blake2b_simd
        "blake2b-256" | "blake2b256" => Some(blake2b_var_hex(data, 32).into_bytes()),
        "blake2b-384" | "blake2b384" => Some(blake2b_var_hex(data, 48).into_bytes()),
        "blake2b-512" | "blake2b512" => Some(blake2b_var_hex(data, 64).into_bytes()),
        "600" => {
            let hexv = blake2b_var_hex(data, 64);
            let mut out = b"$BLAKE2$".to_vec();
            out.extend_from_slice(hexv.as_bytes());
            Some(out)
        }

        // BLAKE2s
        "blake2s-256" | "blake2s256" => Some(blake2s256_hex(data).into_bytes()),
        "31000" => {
            let hexv = blake2s256_hex(data);
            let mut out = b"$BLAKE2$".to_vec();
            out.extend_from_slice(hexv.as_bytes());
            Some(out)
        }

        "ripemd-160" | "ripemd160" | "6000" => {
            let mut h = Ripemd160::new();
            h.update(data);
            Some(hex::encode(h.finalize()).into_bytes())
        }

        "mysql4" | "mysql5" | "300" => Some(mysql_gen(data).into_bytes()),

        "ntlm" | "1000" => match ntlm_hex(data) {
            Some(s) => Some(s.into_bytes()),
            None => None,
        },

        "argon2id" | "34000" => Some(argon2id_tag(data).into_bytes()),

        "bcrypt" | "3200" => {
            if cost < 4 || cost > 31 {
                return Err(anyhow!("Invalid bcrypt cost: 4..31"));
            }
            let mut salt = [0u8; 16];
            rand::thread_rng().fill(&mut salt);
            let s = match std::str::from_utf8(data) {
                Ok(text) => bcrypt::hash_with_salt(text, cost, salt)
                    .map_err(|e| anyhow!("{e:?}"))?
                    .to_string(),
                Err(_) => {
                    let b64 = base64::engine::general_purpose::STANDARD.encode(data);
                    bcrypt::hash_with_salt(&b64, cost, salt)
                        .map_err(|e| anyhow!("{e:?}"))?
                        .to_string()
                }
            };
            Some(s.into_bytes())
        }

        "wpbcrypt" => Some(wpbcrypt(data, cost)?.into_bytes()),

        "md5crypt" | "500" => Some(md5crypt(data).into_bytes()),
        "sha256crypt" | "7400" => Some(sha256crypt(data).into_bytes()),
        "sha512crypt" | "1800" => Some(sha512crypt(data).into_bytes()),

        "phpass" | "phpbb3" | "400" => Some(phpass_md5(data, mode, 11, None).into_bytes()),

        "yescrypt" => Some(yescrypt_mcf(data)?.into_bytes()),

        _ => {
            eprintln!("--> Invalid hash function: {mode} <--");
            help_text();
            std::process::exit(1);
        }
    };
    Ok(out)
}

// helpers

fn trim_ws(b: &[u8]) -> &[u8] {
    let mut s = 0usize;
    let mut e = b.len();
    while s < e && b[s].is_ascii_whitespace() {
        s += 1;
    }
    while e > s && b[e - 1].is_ascii_whitespace() {
        e -= 1;
    }
    &b[s..e]
}

fn mysql_gen(data: &[u8]) -> String {
    let first = Sha1::digest(data);
    let second = Sha1::digest(&first);
    let mut s = String::with_capacity(1 + second.len() * 2);
    s.push('*');
    for &b in second.as_slice() {
        s.push_str(&hex::encode_upper([b]));
    }
    s
}

fn make_salt_hex() -> Vec<u8> {
    let mut raw = [0u8; 8];
    rand::thread_rng().fill(&mut raw);
    hex::encode(raw).into_bytes()
}
