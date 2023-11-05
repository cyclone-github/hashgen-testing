### hashgen (go)
```
$ ./hashgen_amd64.bin -m 0 -w rockyou.txt -o /dev/null
2023/11/02 19:10:51 Starting...
2023/11/02 19:10:51 Processing file: rockyou.txt
2023/11/02 19:10:51 Hash function: 0
2023/11/02 19:10:51 CPU Threads: 16
2023/11/02 19:10:52 Finished hashing 15053568 lines in 0.500 sec (30.123 M lines/sec)
```
This repository is for hashgen-testing. The latest release version of hashgen can be found here:
- https://github.com/cyclone-github/hashgen

Hashgen is a CLI hash generator written in Go and can be cross compiled for Linux, Raspberry Pi, Windows & Mac, although testing and compiling is mainly done on debian 12 linux.

To use hashgen, type your mode, wordlist input & hash output files with a simple command line.

### usage example:
- ./hashgen.bin -m md5 -w wordlist.txt -o output.txt

### compile from source:
- If you want the latest hashgen features, compiling from source is the best option since the release version may run several revisions behind the source code.
- Compile from source code info:
- https://github.com/cyclone-github/scripts/blob/main/intro_to_go.txt

### version history
-  https://github.com/cyclone-github/hashgen-testing/hashgen_go/CHANGELOG.md