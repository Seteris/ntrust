use wasm_bindgen::prelude::*;

use crate::api::{
    CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES,
};
use crate::kem::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use crate::owcpa::owcpa_keypair;
use crate::params::NTRU_SAMPLE_FG_BYTES;
use crate::rng::Aes256CtrDrbgStruct;

pub mod api;
mod cmov;
mod crypto_sort_int32;
pub mod kem;
pub mod owcpa;
mod pack3;
mod packq;
pub mod params;
mod poly;
mod poly_lift;
mod poly_mod;
mod poly_r2_inv;
mod poly_rq_mul;
mod poly_s3_inv;
pub mod rng;
mod sample;
mod sample_iid;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ntrust_keypair() -> Vec<u8> {
    let pk = &mut [0u8; CRYPTO_PUBLICKEYBYTES];
    let sk = &mut [0u8; CRYPTO_SECRETKEYBYTES];
    pk.copy_from_slice(&pk_vec[..CRYPTO_PUBLICKEYBYTES]);
    sk.copy_from_slice(&sk_vec[..CRYPTO_SECRETKEYBYTES]);
    let aes256ctrdrbg = &mut Aes256CtrDrbgStruct::new();
    crypto_kem_keypair(pk, sk, aes256ctrdrbg);
    let mut result = pk.to_vec();
    result.append(&mut sk.to_vec());
    result
}

#[wasm_bindgen]
pub fn ntrust_enc(k_vec: Vec<u8>, pk_vec: Vec<u8>) -> Vec<u8> {
    assert_eq!(k_vec.len(), CRYPTO_BYTES);
    assert_eq!(pk_vec.len(), CRYPTO_PUBLICKEYBYTES);
    let c = &mut [0u8; CRYPTO_CIPHERTEXTBYTES];
    let k = &mut [0u8; CRYPTO_BYTES];
    let pk = &mut [0u8; CRYPTO_PUBLICKEYBYTES];
    k.copy_from_slice(&k_vec[..CRYPTO_BYTES]);
    pk.copy_from_slice(&pk_vec[..CRYPTO_PUBLICKEYBYTES]);
    let aes256ctrdrbg = &mut Aes256CtrDrbgStruct::new();
    crypto_kem_enc(c, k, pk, aes256ctrdrbg);
    c.to_vec()
}


#[wasm_bindgen]
pub fn ntrust_dec(k_vec: Vec<u8>, c_vec: Vec<u8>, sk_vec: Vec<u8>) -> Vec<u8> {
    assert_eq!(k_vec.len(), CRYPTO_BYTES);
    assert_eq!(c_vec.len(), CRYPTO_CIPHERTEXTBYTES);
    assert_eq!(sk_vec.len(), CRYPTO_SECRETKEYBYTES);
    let k = &mut [0u8; CRYPTO_BYTES];
    let c = &mut [0u8; CRYPTO_CIPHERTEXTBYTES];
    let sk = &mut [0u8; CRYPTO_SECRETKEYBYTES];
    k.copy_from_slice(&k_vec[..CRYPTO_BYTES]);
    c.copy_from_slice(&k_vec[..CRYPTO_CIPHERTEXTBYTES]);
    sk.copy_from_slice(&k_vec[..CRYPTO_SECRETKEYBYTES]);
    crypto_kem_dec(k, c, sk);
    k.to_vec()
}

#[wasm_bindgen]
pub fn get_public_key_length() -> usize {
    CRYPTO_PUBLICKEYBYTES
}

#[wasm_bindgen]
pub fn get_secret_key_length() -> usize {
    CRYPTO_PUBLICKEYBYTES
}

#[wasm_bindgen]
pub fn get_ciphertext_length() -> usize {
    CRYPTO_CIPHERTEXTBYTES
}

#[wasm_bindgen]
pub fn get_bytes_length() -> usize {
    CRYPTO_BYTES
}