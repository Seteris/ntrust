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
mod kem;
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
mod rng;
mod sample;
mod sample_iid;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn ntru_encrypt() -> Vec<u8> {
    let pk: &mut [u8; CRYPTO_PUBLICKEYBYTES] = &mut [0; CRYPTO_PUBLICKEYBYTES];
    let sk: &mut [u8; CRYPTO_SECRETKEYBYTES] = &mut [0; CRYPTO_SECRETKEYBYTES];
    let c: &mut [u8; CRYPTO_CIPHERTEXTBYTES] = &mut [0; CRYPTO_CIPHERTEXTBYTES];
    let k: &mut [u8; CRYPTO_BYTES] = &mut [0; CRYPTO_BYTES];
    let aes256ctrdrbg: &mut Aes256CtrDrbgStruct = &mut Aes256CtrDrbgStruct::new();
    crypto_kem_keypair(pk, sk, aes256ctrdrbg);
    crypto_kem_enc(c, k, pk, aes256ctrdrbg);
    c.to_vec()
}

const TEST_PASS: i32 = 0;
const PARAMETER_SIZE_INVALID: i32 = 1;

#[wasm_bindgen]
pub fn crypto_kem_keypair_test(
    pk_vec: Vec<u8>,
    sk_vec: Vec<u8>,
    seed_vec: Vec<u8>,
    comparison_pk_vec: Vec<u8>,
    comparison_sk_vec: Vec<u8>,
    comparison_seed_vec: Vec<u8>,
) -> i32 {
    if pk_vec.len() != CRYPTO_PUBLICKEYBYTES
        || sk_vec.len() != CRYPTO_SECRETKEYBYTES
        || seed_vec.len() != NTRU_SAMPLE_FG_BYTES
        || comparison_pk_vec.len() != CRYPTO_PUBLICKEYBYTES
        || comparison_sk_vec.len() != CRYPTO_SECRETKEYBYTES
        || comparison_seed_vec.len() != NTRU_SAMPLE_FG_BYTES
    {
        return PARAMETER_SIZE_INVALID;
    }
    let pk: &mut [u8; CRYPTO_PUBLICKEYBYTES] = &mut [0; CRYPTO_PUBLICKEYBYTES];
    let sk: &mut [u8; CRYPTO_SECRETKEYBYTES] = &mut [0; CRYPTO_SECRETKEYBYTES];
    let mut seed: [u8; NTRU_SAMPLE_FG_BYTES] = [0; NTRU_SAMPLE_FG_BYTES];
    let comparison_pk: &mut [u8; CRYPTO_PUBLICKEYBYTES] = &mut [0; CRYPTO_PUBLICKEYBYTES];
    let comparison_sk: &mut [u8; CRYPTO_SECRETKEYBYTES] = &mut [0; CRYPTO_SECRETKEYBYTES];
    let mut comparison_seed: [u8; NTRU_SAMPLE_FG_BYTES] = [0; NTRU_SAMPLE_FG_BYTES];
    pk.copy_from_slice(&pk_vec[..CRYPTO_PUBLICKEYBYTES]);
    sk.copy_from_slice(&sk_vec[..CRYPTO_SECRETKEYBYTES]);
    seed.copy_from_slice(&seed_vec[..NTRU_SAMPLE_FG_BYTES]);
    comparison_pk.copy_from_slice(&comparison_pk_vec[..CRYPTO_PUBLICKEYBYTES]);
    comparison_sk.copy_from_slice(&comparison_sk_vec[..CRYPTO_SECRETKEYBYTES]);
    comparison_seed.copy_from_slice(&comparison_seed_vec[..NTRU_SAMPLE_FG_BYTES]);
    owcpa_keypair(pk, sk, seed);
    assert_eq!(pk, comparison_pk);
    assert_eq!(sk, comparison_sk);
    assert_eq!(seed, comparison_seed);
    TEST_PASS
}
