# NTRUst

A wasm-compatible pure-rust implementation of the NTRU lattice-based key encapsulation mechanism, which falls into the category of post-quantum cryptographical schemes.

* This implementation is based on the NTRU reference implementation of NIST round 3
* It depends on `tiny-keccak` as SHA-3 implementation and `aes` as AES block cipher (used as RNG) implementation
* It passes the 100 testcases of the C reference implementation
* The implementation takes between 50 milliseconds (`ntruhps2048509`) and 100 milliseconds (`ntruhps4096821`) to run on a modern computer
* The implementation is constant-time on software instruction level
* The random number generator is based on AES256 in counter mode

## Including it in your project
Add this to your `Cargo.toml`:
```toml
[dependencies]
ntrust = { version = "0.7", features = ["<feature>"] }
```
where `<feature>` can be any of "ntruhps2048509", "ntruhps2048677", "ntruhps4096821" and "ntruhrss701".

In your `package.json`, add the following dependency:
```JSON
{
  "dependencies": {
    "ntrust": "file:<path-to-crate>"
  }
}
```
Don't forget to run `npm install` to install the new dependency.

## Usage
First, import the package under an alias (wasm in this example):
```JavaScript
import * as wasm from "ntrust";
```
Now you can use the exposed functions, like so:
```JavaScript
wasm.get_public_key_length();
```
Generating keys:
```JavaScript
let keys = wasm.ntrust_keypair();
let public_key_length = wasm.get_public_key_length();
let pk = keys.slice(0, public_key_length);
let sk = keys.slice(public_key_length);
```

## API
All functionality is provided by the following three functions:
```rust
pub fn ntrust_keypair() -> Vec<u8> { ... }
pub fn ntrust_enc(k: Vec<u8>, pk: Vec<u8>) -> Vec<u8> { ... }
pub fn ntrust_dec(k: Vec<u8>, c: Vec<u8>, sk: Vec<u8>) -> Vec<u8> { ... }
```
`ntrust_keypair()` returns a vector containing both the public and the secret key.  
The first `CRYPTO_PUBLICKEYBYTES` elements are the public key, while the last `CRYPTO_SECRETKEYBYTES` correspond to the
secret key.

Also provided by the module are the following helper functions which provide the sizes corresponding to variables:
```rust
pub fn get_public_key_length() -> usize { ... }
pub fn get_secret_key_length() -> usize { ... }
pub fn get_ciphertext_length() -> usize { ... }
pub fn get_bytes_length() -> usize { ... }
```
