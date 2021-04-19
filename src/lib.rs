mod utils;
pub mod sample;

use wasm_bindgen::prelude::*;
use tiny_keccak::Sha3;
use tiny_keccak::Shake;
use tiny_keccak::Hasher;
use web_sys;

use sample::params::NTRU_N as NTRU_N;
use sample::params::NTRU_SAMPLE_FG_BYTES as NTRU_SAMPLE_FG_BYTES;
use sample::sample_fg as sample_fg;

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
    pub fn new(poly_coeffs: [u16; NTRU_N]) -> Poly {
        Poly {
            coeffs: poly_coeffs,
        }
    }
}

pub fn owcpa_keypair(mut pk: &u8,
                     mut sk: &u8,
                     seed: [u8; NTRU_SAMPLE_FG_BYTES]) {
    let i: isize;
    let mut x1: Poly = Poly::new([0; NTRU_N]);
    let mut x2: Poly = Poly::new([0; NTRU_N]);
    let mut x3: Poly = Poly::new([0; NTRU_N]);
    let mut x4: Poly = Poly::new([0; NTRU_N]);
    let mut x5: Poly = Poly::new([0; NTRU_N]);

    let f: &mut Poly = &mut x1;
    let g: &mut Poly = &mut x2;
    let invf_mod3: &Poly = &x3;
    let gf: &Poly = &x3;
    let invgf: &Poly = &x4;
    let tmp: &Poly = &x5;
    let invh: &Poly = &x3;
    let h: &Poly = &x3;


    sample_fg(f,g,seed);
    // TODO: continue port of C function

}

