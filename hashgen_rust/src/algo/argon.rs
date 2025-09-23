// src/algo/argon.rs

use argon2::{Algorithm, Argon2, Params, Version};
use base64::engine::general_purpose::STANDARD_NO_PAD as B64_NOPAD;
use base64::Engine as _;
use rand::Rng;

/// argon2id hash (Hashcat mode 3400)
pub fn argon2id_tag(pass: &[u8]) -> String {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);

    // m = 64 MiB, t = 4 iterations, p = 1 thread, output length = 16
    let params = Params::new(65536, 4, 1, Some(16)).unwrap();
    let a2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 16];
    a2.hash_password_into(pass, &salt, &mut key).unwrap();

    let sb = B64_NOPAD.encode(&salt);
    let kb = B64_NOPAD.encode(&key);

    format!("$argon2id$v=19$m=65536,t=4,p=1${}${}", sb, kb)
}
