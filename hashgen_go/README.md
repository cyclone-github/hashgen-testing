### hashgen (go)
```
$ ./hashgen_amd64.bin -m 0 -w rockyou.txt -o /dev/null
2024/12/10 19:07:31 Starting...
2024/12/10 19:07:31 Processing file: rockyou.txt
2024/12/10 19:07:31 Hash function: 0
2024/12/10 19:07:31 CPU Threads: 16
2024/12/10 19:07:31 Finished processing 14344391 lines in 0.475 sec (30.228 M lines/sec)
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
-  https://github.com/cyclone-github/hashgen-testing/blob/main/hashgen_go/CHANGELOG.md