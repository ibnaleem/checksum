> [!WARNING]
> This branch is deprecated. Use the [`main` branch](https://github.com/ibnaleem/checksum) (rewritten in Rust) for the latest release, which offers better performance, especially with large files like `.iso`. Issues will be prioritised on the [`main` branch](https://github.com/ibnaleem/checksum).
# checksum
A Python script for verifying file hashes.

## Installation
```
$ git clone https://github.com/ibnaleem/checksum.git
```
For Unix:
```
$ python3 -m venv myvenv
$ source myvenv/bin/activate
$ (venv) pip install -r requirements.txt
```
For Windows:
```
$ python3 -m venv myvenv
$ myvenv\Scripts\activate
$ (venv) pip install -r requirements.txt
```
## Usage
```
python3 checksum.py /path/to/file provided_hash
```

## Hashing Algorithms
This script supports the following hashing algorithms:
- MD5
- SHA1
- SHA224
- SHA256
- SHA384
- SHA512
