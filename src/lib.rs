use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
use tiny_keccak::Shake;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::api::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};
use crate::kem::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use crate::owcpa::owcpa_keypair;
use crate::params::{NTRU_OWCPA_SECRETKEYBYTES, NTRU_PRFKEYBYTES, NTRU_SAMPLE_FG_BYTES};
use crate::rng::{Aes256CtrDrbgStruct, randombytes};

mod utils;
mod sample;
pub mod params;
mod crypto_sort_int32;
mod poly_s3_inv;
mod pack3;
mod poly_mod;
mod poly;
mod poly_rq_mul;
mod poly_r2_inv;
mod packq;
mod sample_iid;
pub mod api;
pub mod owcpa;
mod poly_lift;
mod rng;
mod kem;
mod cmov;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

const TARGET_SHA3_256: i8 = 0;
const TARGET_SHA3_512: i8 = 1;
const TARGET_SHAKE_128: i8 = 2;
const TARGET_SHAKE_256: i8 = 3;

const DEFAULT_TARGET: i8 = TARGET_SHA3_256;

#[wasm_bindgen]
pub fn get_random_bytes(num_bytes: i64) -> Vec<u8> {
    let mut random: Vec<u8> = vec![0; num_bytes as usize];
    getrandom::getrandom(&mut random).unwrap();
    random
}

#[wasm_bindgen]
pub fn tiny_keccak(input: String, target: i8) -> Vec<u8> {
    let output: Vec<u8> = match target {
        TARGET_SHA3_256 | TARGET_SHA3_512 => {
            sha3_wrapper(input, target)
        }
        TARGET_SHAKE_128 | TARGET_SHAKE_256 => {
            shake_wrapper(input, target)
        }
        _ => {
            // Default:
            sha3_wrapper(input, DEFAULT_TARGET)
        }
    };
    output
}

pub fn sha3_wrapper(input: String, target: i8) -> Vec<u8> {
    log!("[RS][SHA3] Hashing \"{}\"", input);
    let result = match target {
        TARGET_SHA3_512 => {
            log!("[RS][SHA3] SHA3_512");
            let mut sha3 = Sha3::v512();
            let mut sha_3_512_out = [0u8; 64];
            sha3.update(input.as_bytes().as_ref());
            sha3.finalize(&mut sha_3_512_out);
            sha_3_512_out.to_vec()
        }
        TARGET_SHA3_256 | _ => {
            log!("[RS][SHA3] SHA3_256");
            let mut sha3 = Sha3::v256();
            let mut sha_3_256_out = [0u8; 32];
            sha3.update(input.as_bytes().as_ref());
            sha3.finalize(&mut sha_3_256_out);
            sha_3_256_out.to_vec()
        }
    };
    result
}

pub fn shake_wrapper(input: String, target: i8) -> Vec<u8> {
    log!("[RS][SHAKE] Hashing \"{}\"", input);
    let result = match target {
        TARGET_SHAKE_256 => {
            let mut shake = Shake::v256();
            log!("[RS][SHAKE] SHAKE_256");
            let mut shake_256_out = [0u8; 64];
            shake.update(input.as_ref());
            shake.finalize(&mut shake_256_out);
            shake_256_out.to_vec()
        }
        TARGET_SHAKE_128 | _ => {
            let mut shake = Shake::v128();
            log!("[RS][SHAKE] SHAKE_128");
            let mut shake_128_out = [0u8; 32];
            shake.update(input.as_ref());
            shake.finalize(&mut shake_128_out);
            shake_128_out.to_vec()
        }
    };
    result
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn ntru_bench() {
    log!("Starting Bench");
    let pk: &mut [u8; CRYPTO_PUBLICKEYBYTES] = &mut [0; CRYPTO_PUBLICKEYBYTES];
    let sk: &mut [u8; CRYPTO_SECRETKEYBYTES] = &mut [0; CRYPTO_SECRETKEYBYTES];
    let c: &mut [u8; CRYPTO_CIPHERTEXTBYTES] = &mut [0; CRYPTO_CIPHERTEXTBYTES];
    let k: &mut [u8; CRYPTO_BYTES] = &mut [0; CRYPTO_BYTES];
    let aes256ctrdrbg: &mut Aes256CtrDrbgStruct = &mut Aes256CtrDrbgStruct::new();
    log!("Running Keypair");
    crypto_kem_keypair(pk, sk, aes256ctrdrbg);
    log!("Running Enc");
    crypto_kem_enc(c, k, pk, aes256ctrdrbg);
    log!("Running Dec");
    crypto_kem_dec(k, c, sk);
    log!("DONE");
}

#[wasm_bindgen]
pub fn ntru_encrypt() -> Vec<u8> {
    let pk: &mut [u8; CRYPTO_PUBLICKEYBYTES] = &mut [0; CRYPTO_PUBLICKEYBYTES];
    let sk: &mut [u8; CRYPTO_SECRETKEYBYTES] = &mut [0; CRYPTO_SECRETKEYBYTES];
    let c: &mut [u8; CRYPTO_CIPHERTEXTBYTES] = &mut [0; CRYPTO_CIPHERTEXTBYTES];
    let k: &mut [u8; CRYPTO_BYTES] = &mut [0; CRYPTO_BYTES];
    let aes256ctrdrbg: &mut Aes256CtrDrbgStruct = &mut Aes256CtrDrbgStruct::new();
    crypto_kem_keypair(pk, sk, aes256ctrdrbg);
    crypto_kem_enc(c, k, pk, aes256ctrdrbg);
    sk.to_vec()
}

const TEST_PASS: i32 = 0;
const PARAMETER_SIZE_INVALID: i32 = 1;

#[wasm_bindgen]
pub fn crypto_kem_keypair_test(
    mut pk_vec: Vec<u8>,
    mut sk_vec: Vec<u8>,
    mut seed_vec: Vec<u8>,
    comparison_pk_vec: Vec<u8>,
    comparison_sk_vec: Vec<u8>,
    comparison_seed_vec: Vec<u8>,
) -> i32 {
    if pk_vec.len() != CRYPTO_PUBLICKEYBYTES ||
        sk_vec.len() != CRYPTO_SECRETKEYBYTES ||
        seed_vec.len() != NTRU_SAMPLE_FG_BYTES ||
        comparison_pk_vec.len() != CRYPTO_PUBLICKEYBYTES ||
        comparison_sk_vec.len() != CRYPTO_SECRETKEYBYTES ||
        comparison_seed_vec.len() != NTRU_SAMPLE_FG_BYTES {
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