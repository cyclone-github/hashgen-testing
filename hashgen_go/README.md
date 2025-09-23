### hashgen (go)
```
$ hashgen -m md5 -w rockyou.txt -b
2025/08/23 19:18:27 Starting...
2025/08/23 19:18:27 Processing file: rockyou.txt
2025/08/23 19:18:27 Hash function: md5
2025/08/23 19:18:27 CPU Threads: 16
2025/08/23 19:18:28 Finished processing 14344391 lines in 0.465 sec (30.839 M lines/sec)
```
This repository is for hashgen-testing. The latest release version of hashgen can be found here:
- https://github.com/cyclone-github/hashgen

Hashgen is a CLI hash generator written in Go and can be cross compiled for Linux, Raspberry Pi, Windows & Mac, although testing and compiling is mainly done on debian 12 linux.

To use hashgen, type your mode, wordlist input & hash output files with a simple command line.

### usage example:
- ./hashgen.bin -m md5 -w wordlist.txt -o output.txt

### compile from source:
- This assumes you have Go and Git installed
  - `git clone https://github.com/cyclone-github/hashgen.git`  # clone repo
  - `cd hashgen`                                               # enter project directory
  - `go mod init hashgen`                                      # initialize Go module (skips if go.mod exists)
  - `go mod tidy`                                              # download dependencies
  - `go build -ldflags="-s -w" .`                              # compile binary in current directory
  - `go install -ldflags="-s -w" .`                            # compile binary and install to $GOPATH
- Compile from source code how-to:
  - https://github.com/cyclone-github/scripts/blob/main/intro_to_go.txt

### changelog:
-  https://github.com/cyclone-github/hashgen/blob/main/CHANGELOG.md