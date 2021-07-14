use tiny_keccak::Hasher;
use tiny_keccak::Sha3;
use tiny_keccak::Shake;
use wasm_bindgen::prelude::*;
use web_sys;

use crate::api::{CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};
use crate::owcpa::owcpa_keypair;
use crate::params::{NTRU_OWCPA_SECRETKEYBYTES, NTRU_PRFKEYBYTES, NTRU_SAMPLE_FG_BYTES};

mod utils;
mod sample;
mod params;
mod crypto_sort_int32;
mod poly_s3_inv;
mod pack3;
mod poly_mod;
mod poly;
mod poly_rq_mul;
mod poly_r2_inv;
mod packq;
mod sample_iid;
mod api;
mod owcpa;
mod poly_lift;

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

#[wasm_bindgen]
pub fn crypto_kem_keypair() {
    let mut pk: [u8; CRYPTO_PUBLICKEYBYTES] = [0; CRYPTO_PUBLICKEYBYTES];
    let mut sk: [u8; CRYPTO_SECRETKEYBYTES] = [0; CRYPTO_SECRETKEYBYTES];
    let mut seed: [u8; NTRU_SAMPLE_FG_BYTES] = [0; NTRU_SAMPLE_FG_BYTES];
    randombytes(&mut seed, NTRU_SAMPLE_FG_BYTES as u64);

    owcpa_keypair(&mut pk, &mut sk, seed);

    let mut sk_copy: [u8; NTRU_PRFKEYBYTES] = [0; NTRU_PRFKEYBYTES];
    sk_copy.copy_from_slice(&sk[NTRU_OWCPA_SECRETKEYBYTES..]);
    randombytes(&mut sk_copy, NTRU_PRFKEYBYTES as u64);
    sk[NTRU_OWCPA_SECRETKEYBYTES..].copy_from_slice(&sk_copy);

    log!("----PK----");
    log!("{:x?}", pk);
    log!("----SK----");
    log!("{:x?}", sk);
    log!("----Seed----");
    log!("{:x?}", seed);
}

pub fn randombytes(x: &mut [u8], xlen: u64) -> i32 {
    for i in 0..xlen {
        x[i as usize] = i as u8;
    }
    0
}