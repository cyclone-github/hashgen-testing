// src/consts.rs

/// crypt64 alphabet for crypt hashes
pub static CRYPT64: &[u8] = b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// print version info (to stderr)
pub fn version() {
    eprintln!("hashgen v1.2.0-rust\nhttps://github.com/cyclone-github/hashgen");
}

/// print custom help text
pub fn help_text() {
    version();
    let s = r#"
Example Usage:

./hashgen -m md5 -w wordlist.txt -o output.txt
./hashgen -m bcrypt -cost 8 -w wordlist.txt
cat wordlist | ./hashgen -m md5 -hashplain

All Supported Options:
-m {mode}
-w {wordlist input}
-t {cpu threads}
-o {wordlist output}
-b {benchmark mode}
-cost {bcrypt, default=10}
-hashplain {generates hash:plain pairs}

If -w is not specified, defaults to stdin
If -o is not specified, defaults to stdout
If -t is not specified, defaults to max available CPU threads

Modes:            Hashcat Mode (notes)
argon2id          34000
base32decode
base32encode
base58decode
base58encode
base64decode
base64encode
bcrypt            3200
blake2s-256
31000             (hashcat-compatible BLAKE2s-256)
blake2b-256
blake2b-384
blake2b-512
600               (hashcat-compatible BLAKE2b-512)
crc32
11500             (hashcat-compatible CRC32)
crc64
hex               (encode to $HEX[])
dehex/plaintext   99999 (decode $HEX[])
keccak-224        17700
keccak-256        17800
keccak-384        17900
keccak-512        18000
md4               900
md5               0
md5passsalt       10
md5saltpass       20
md5md5            2600
md5crypt          500 (Linux shadow $1$)
mysql4/mysql5     300
morsecode         (ITU-R M.1677-1)
ntlm              1000
phpass            400
ripemd-160        6000
sha1              100
sha1sha1          4500
sha1passsalt      110
sha1saltpass      120
sha224            1300
sha224passsalt    1310
sha224saltpass    1320
sha256            1400
sha256passsalt    1410
sha256saltpass    1420
sha256crypt       7400 (Linux shadow $5$)
sha384            10800
sha384passsalt    10810
sha384saltpass    10820
sha512            1700
sha512passsalt    1710
sha512saltpass    1720
sha512crypt       1800 (Linux shadow $6$)
sha512-224
sha512-256
sha3-224          17300
sha3-256          17400
sha3-384          17500
sha3-512          17600
wpbcrypt          (WordPress bcrypt-HMAC-SHA384)
yescrypt          (Linux shadow $y$)
"#;
    eprintln!("{s}");
}
