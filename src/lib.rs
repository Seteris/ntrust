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

use wasm_bindgen::prelude::*;
use tiny_keccak::Sha3;
use tiny_keccak::Shake;
use tiny_keccak::Hasher;
use web_sys;

use crate::params::{NTRU_N, NTRU_SAMPLE_FG_BYTES, NTRU_PACK_TRINARY_BYTES, NTRU_HRSS, NTRU_HPS, NTRU_OWCPA_MSGBYTES};
use crate::sample::sample_fg;
use crate::pack3::poly_s3_tobytes;
use crate::poly::{poly_z3_to_zq, poly_rq_inv};
use crate::poly_s3_inv::poly_s3_inv;
use crate::poly_rq_mul::poly_rq_mul;
use crate::packq::{poly_sq_tobytes, poly_rq_sum_zero_tobytes};
use std::convert::TryInto;
use crate::api::{CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};

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

pub struct Poly {
    coeffs: [u16; NTRU_N],
}

impl Poly {
    pub fn new() -> Poly {
        Poly {
            coeffs: [0; NTRU_N],
        }
    }
    pub fn build(value: u16) -> Poly {
        Poly {
            coeffs: [value; NTRU_N],
        }
    }
}

pub fn owcpa_keypair(pk: &mut [u8; CRYPTO_PUBLICKEYBYTES],
                     sk: &mut [u8; CRYPTO_SECRETKEYBYTES],
                     seed: [u8; NTRU_SAMPLE_FG_BYTES]) {

    let mut x3: Poly = Poly::new();

    let f: &mut Poly = &mut Poly::new();
    let g: &mut Poly = &mut Poly::new();
    let invf_mod3: &mut Poly = &mut Poly::new();

    let invgf: &mut Poly = &mut Poly::new();
    let tmp: &mut Poly = &mut Poly::new();

    // let gf: &mut Poly = &mut x3;
    // let invh: &mut Poly = &mut x3;
    // let h: &mut Poly = &mut x3;


    sample_fg(f, g, seed);

    poly_s3_inv(invf_mod3, f);
    let mut sk_bytes: [u8; NTRU_OWCPA_MSGBYTES] = sk[..NTRU_OWCPA_MSGBYTES]
        .try_into()
        .expect("Slice has incorrect length.");
    poly_s3_tobytes(&mut sk_bytes, f);
    let mut sk_msgbytes = sk[NTRU_OWCPA_MSGBYTES..NTRU_OWCPA_MSGBYTES * 2]
        .try_into()
        .expect("Slice has incorrect length.");
    poly_s3_tobytes(&mut sk_msgbytes, invf_mod3);

    /* Lift coeffs of f and g from Z_p to Z_q */
    poly_z3_to_zq(f);
    poly_z3_to_zq(g);
    if NTRU_HRSS {
        /* g = 3*(x-1)*g */
        // C implementation loops from [NTRU_N - 1;0)
        // .rev() reverses the iterator AFTER the range has been evaluated
        for i in (1..NTRU_N).rev() {
            g.coeffs[i] = 3 * (g.coeffs[i - 1] - g.coeffs[i]);
        }
        g.coeffs[0] = 0 - (3 * g.coeffs[0]);
    }

    if NTRU_HPS {
        /* g = 3*g */
        for i in 0..NTRU_N {
            g.coeffs[i] = 3 * g.coeffs[i];
        }
    }

    poly_rq_mul(&mut x3, g, f);

    poly_rq_inv(invgf, &mut x3);

    poly_rq_mul(tmp, invgf, f);
    poly_rq_mul(&mut x3, tmp, f);
    const SK_PACK_TRINARY_BYTE_SIZE: usize = CRYPTO_SECRETKEYBYTES - 2 * NTRU_PACK_TRINARY_BYTES;
    let mut sk_pack_trinary_bytes: [u8; SK_PACK_TRINARY_BYTE_SIZE] = sk[2 * NTRU_PACK_TRINARY_BYTES..]
        .try_into()
        .expect("Slice has incorrect length.");
    poly_sq_tobytes(&mut sk_pack_trinary_bytes, &mut x3);

    poly_rq_mul(tmp, invgf, g);
    poly_rq_mul(&mut x3, tmp, g);
    poly_rq_sum_zero_tobytes(pk, &mut x3);
}

