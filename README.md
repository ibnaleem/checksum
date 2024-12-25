![BUILD](https://github.com/ibnaleem/checksum/actions/workflows/rust.yml/badge.svg?event=push)
# checksum
File checksum verification tool written in Rust.

## Installation
```
$ git clone https://github.com/ibnaleem/checksum.git
```
```
$ cargo build
```
This will generate a binary under `/target/debug/`. You can also download the compiled binary [here.](https://github.com/ibnaleem/checksum/releases)
## Usage
```
$ checksum <file_path> <hash>
```

## Hashing Algorithms
This script supports the following hashing algorithms:
- MD5
- SHA1
- SHA224
- SHA256
- SHA384
- SHA512
- BLAKE2b
- Tiger192
- RIPEMD320
