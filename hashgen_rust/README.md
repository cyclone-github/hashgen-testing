### hashgen (Rust)
- this is an experimental POC rewrite of hashgen (Go) v1.2.0-dev in Rust -- not recommended for production
- hashgen (Go) will remain my primary implementation of hashgen and I do not expect hashgen (Rust) to be maintained

### usage example:
- ./hashgen -m md5 -w wordlist.txt -o output.txt

### version history
- v2023-10-30.1615
  - initial github release
- v2024-05-01.1100
  - add support for non-UTF8 char, read input as bytes, add support for hashcat modes (-m)
- v1.2.0-rust; 2025-09-23
  - complete rewrite of hashgen (Go) v1.2.0-dev in Rust

### compile from source:
- cargo build --release