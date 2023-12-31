### hashgen (go)
- v2022-12-15.2030; initial github release
- v2022-12-16.1800; fixed ntlm hash function, tweaked -w flag to be less restrictive, clean up code
- v2022-12-17.2100; fixed typo in wordlist tag, added '-m plaintext' output mode (prints -w wordlist file to stdout)
- v2022-12-20.1200; cleaned up bcrypt code
- v2022-12-20.1430-goroutine; complete rewrite using goroutines & read/write buffers
- v2022-12-21.1400-goroutine; added multiple new algo's including hashcat mode equivalents
- v2022-12-24.1800-optimize; optimized all hashing functions, tweaked buffer size
- v2023-03-15.0900-optimize; added "stdout", edited "lines/sec" to show "M lines/sec", tweaked output buffer for stdout, tweaked sha2xxx flags
- v2023-03-28.1155-optimize; added "stdin"
- v2023-05-13.0000-optimize; optimized code all hashing functions for better performance (version not released on github)
- v2023-08-15.1900-hashplain; added: -hashplain flag for hash:plain output, support for $HEX[] wordlist, -cost flag for bcrypt, tweaked: write buffers & custom buffers for argon & bcrypt, tweaked logging outputs
- v2023-08-16.1200-hashplain; added error correction to 'fix' improperly formatted $HEX[] lines
- v2023-10-30.1600-threaded; rewrote code base for multi-threading support, some algos have not been implemented from previous version
- v2023-11-04.0945-threaded; added hashcat 11500 (CRC32 w/padding), re-added CRC32 / CRC64, fix stdin