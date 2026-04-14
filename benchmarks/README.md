# Benchmarks
 
### Latest Version: 
- hashgen v1.3.1
- Test rig specs:
  - OS: Linux pve 6.17.13-2-pve (Debian 13.4)
  - CPU: AMD Ryzen 7 3700X 8-Core (16 Thread) Processor @ 3600MHz
  - RAM: 64gb DDR4
- Benchmarks for all 100 supported modes:

| Mode  | h/s |
| ------------- | ------------- |
| plaintext | 46094000 |
| hex | 36980000 |
| dehex | 46658000 |
| base64encode | 38424000 |
| base64decode | 38269000 |
| base58encode | 16783000 |
| base58decode | 19581000 |
| base32encode | 34042000 |
| base32decode | 34626000 |
| morsecode | 20604000 |
| morsedecode | 16170000 |
| crc32 | 37231000 |
| crc64 | 37844000 |
| md4 | 22222000 |
| md5 | 32708000 |
| halfmd5 | 33953000 |
| ntlm | 14213000 |
| md6-128 | 3797000 |
| md6-224 | 2854000 |
| md6-256 | 2690000 |
| md6-384 | 2064000 |
| md6-512 | 1722000 |
| md5passsalt | 18543000 |
| md5saltpass | 18624000 |
| md5md5 | 26757000 |
| md5utf16passsalt | 14585000 |
| md5utf16saltpass | 14191000 |
| md5utf16le | 21705000 |
| sha1 | 25693000 |
| sha1passsalt | 18011000 |
| sha1saltpass | 19005000 |
| sha1utf16passsalt | 13269000 |
| sha1utf16saltpass | 13876000 |
| sha1utf16le | 18211000 |
| sha1sha1 | 23158000 |
| sha2-224 | 20642000 |
| sha224passsalt | 14975000 |
| sha224saltpass | 15062000 |
| sha2-256 | 21742000 |
| sha256passsalt | 12507000 |
| sha256saltpass | 13006000 |
| sha256utf16passsalt | 10639000 |
| sha256utf16saltpass | 10884000 |
| sha256utf16le | 16314000 |
| sha2-384 | 11755000 |
| sha384passsalt | 9510000 |
| sha384saltpass | 9563000 |
| sha2-512 | 10868000 |
| sha512passsalt | 8019000 |
| sha512saltpass | 8101000 |
| sha512utf16passsalt | 7175000 |
| sha512utf16saltpass | 7106000 |
| sha512utf16le | 9153000 |
| sha2-512-224 | 16749000 |
| sha2-512-256 | 16404000 |
| sha3-224 | 13217000 |
| sha3-256 | 12912000 |
| sha3-384 | 9889000 |
| sha3-512 | 9031000 |
| keccak-224 | 8277000 |
| keccak-256 | 13037000 |
| keccak-384 | 7548000 |
| keccak-512 | 9348000 |
| blake2s-256 | 21427000 |
| blake2b-256 | 20787000 |
| blake2b-384 | 13097000 |
| blake2b-512 | 11121000 |
| blake2b512passsalt | 7952000 |
| blake2b512saltpass | 7959000 |
| blake2b256passsalt | 11199000 |
| blake2b256saltpass | 11243000 |
| hmac-blake2s-pass | 6075000 |
| ripemd-160 | 14885000 |
| hmac-ripemd160-pass | 4251000 |
| hmac-ripemd160-salt | 4182000 |
| hmac-md5-pass | 7735000 |
| hmac-md5-salt | 7702000 |
| hmac-sha1-pass | 7983000 |
| hmac-sha1-salt | 7865000 |
| hmac-sha256-pass | 6530000 |
| hmac-sha256-salt | 6457000 |
| hmac-sha512-pass | 3728000 |
| hmac-sha512-salt | 3718000 |
| mysql5 | 20576000 |
| pbkdf2-sha256 | 61538 |
| pbkdf2-md5 | 26667 |
| pbkdf2-sha1 | 62841 |
| pbkdf2-sha512 | 16878 |
| md5crypt | 57263 |
| sha256crypt | 7386 |
| sha512crypt | 4036 |
| phpass | 34662 |
| phpbb3 | 29498 |
| bcrypt | 242 |
| wpbcrypt | 246 |
| yescrypt | 204 |
| scrypt | 203 |
| argon2id | 22 |

### Hash generator benchmarks
 - Head to head comparison of different hash generators
 - All testing was performed hashing rockyou.txt (14,344,391 lines) to md5
 - Hashing was written to /dev/null where applicable, or to an ssd zpool (this was to keep write speed from being a bottleneck)

### Results:
| Program  | Time/s | h/s |
| ------------- | ------------- | ------------- |
| hashgen v1.3.1 (go)  | 0.446s | 32,162,312 |
| hashgen v1.2.0 (rust)    | 0.452s | 31,735,378 |
| mdxfind v1.302	| 3.032s | 4,781,463 |
| hashgen v2023-10-30.1615 (php) | 3.877s | 3,876,788 |
| hashgen v2023-06-05 (c)   | 4.120s | 3,652,047 |
| hashgen v2023-10-30 (python)  | 8.611s | 1,748,178 |
| hashcat test.pl	| 23.086s | 653,840 |
| ULM | 129s | 116,694 |
| bash | 2h+ | N/A |

### Links:
- hashgen (go) https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_go
- hashgen (php) https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_php
- hashgen (rust) https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_rust
- hashgen (c) https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_c
- hashgen (python) https://github.com/cyclone-github/hashgen-testing/tree/main/hashgen_python
- mdxfind https://github.com/Cynosureprime/mdxfind
- hashcat test.pl https://github.com/hashcat/hashcat/blob/master/tools/test.pl
- ULM v1E139 https://github.com/cyclone-github/hashgen-testing/blob/main/benchmarks/ulm_results.txt