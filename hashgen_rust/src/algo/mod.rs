// src/algo/mod.rs

pub mod argon;
pub mod blake2;
pub mod hex;
pub mod keccak;
pub mod md5crypt;
pub mod hash_mode;
pub mod morse;
pub mod ntlm;
pub mod phpass;
pub mod sha256crypt;
pub mod sha512crypt;
pub mod wpbcrypt;
pub mod yescrypt;

pub use hex::check_for_hex;
pub use hash_mode::hash_line;