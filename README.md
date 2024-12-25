> [!WARNING]
> This branch is deprecated. Use the [`main` branch](https://github.com/ibnaleem/checksum) (rewritten in Rust) for the latest release, which offers better performance, especially with large files like `.iso`. Issues will be prioritised on the [`main` branch](https://github.com/ibnaleem/checksum).
# checksum
File checksum verification tool written in Go.

## Installation
```
$ git clone https://github.com/ibnaleem/checksum.git
```
```
$ go build
```
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
