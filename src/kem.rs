use tiny_keccak::{Hasher, Sha3};
use crate::api::{CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};
use crate::cmov::cmov;
use crate::owcpa::{owcpa_dec, owcpa_enc, owcpa_keypair};
use crate::pack3::poly_s3_tobytes;
use crate::params::{NTRU_CIPHERTEXTBYTES, NTRU_OWCPA_MSGBYTES, NTRU_OWCPA_SECRETKEYBYTES, NTRU_PACK_TRINARY_BYTES, NTRU_PRFKEYBYTES, NTRU_SAMPLE_FG_BYTES, NTRU_SAMPLE_RM_BYTES, NTRU_SHAREDKEYBYTES};
use crate::poly::{Poly, poly_z3_to_zq};
use crate::rng::{Aes256CtrDrbgStruct, randombytes};
use crate::sample::sample_rm;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// TODO: create function calling kem_keypair/_enc/_dec
// TODO: instantiate aes256drbgstruct in aforementioned function

#[wasm_bindgen]
pub fn crypto_kem_keypair() {
    let mut pk: [u8; CRYPTO_PUBLICKEYBYTES] = [0; CRYPTO_PUBLICKEYBYTES];
    let mut sk: [u8; CRYPTO_SECRETKEYBYTES] = [0; CRYPTO_SECRETKEYBYTES];
    let mut seed: [u8; NTRU_SAMPLE_FG_BYTES] = [0; NTRU_SAMPLE_FG_BYTES];

    let aes_ctr_drbg: &mut Aes256CtrDrbgStruct = &mut Aes256CtrDrbgStruct::new();
    randombytes(&mut seed, &mut (NTRU_SAMPLE_FG_BYTES as u64), aes_ctr_drbg);

    owcpa_keypair(&mut pk, &mut sk, seed);

    let mut sk_copy: [u8; NTRU_PRFKEYBYTES] = [0; NTRU_PRFKEYBYTES];
    sk_copy.copy_from_slice(&sk[NTRU_OWCPA_SECRETKEYBYTES..]);
    randombytes(&mut sk_copy, &mut (NTRU_PRFKEYBYTES as u64), aes_ctr_drbg);
    sk[NTRU_OWCPA_SECRETKEYBYTES..].copy_from_slice(&sk_copy);

    log!("----PK----");
    log!("{:x?}", pk);
    log!("----SK----");
    log!("{:x?}", sk);
    log!("----Seed----");
    log!("{:x?}", seed);
}

pub fn crypto_kem_enc(
    c: &mut [u8; CRYPTO_CIPHERTEXTBYTES],
    k: &mut [u8; CRYPTO_BYTES],
    pk: &mut [u8; CRYPTO_PUBLICKEYBYTES],
) {
    let r: &mut Poly = &mut Poly::new();
    let m: &mut Poly = &mut Poly::new();
    let rm: &mut [u8; NTRU_OWCPA_MSGBYTES] = &mut [0; NTRU_OWCPA_MSGBYTES];
    let rm_seed: &mut [u8; NTRU_SAMPLE_RM_BYTES] = &mut [0; NTRU_SAMPLE_RM_BYTES];

    // TODO: 3rd parameter type Aes256CtrDrbgStruct in function signature
    // is global in c implementation
    // FIXME: Add main() routine, initialising struct and passing as parameter
    let aes256ctr = &mut Aes256CtrDrbgStruct::new();
    let xlen: &mut u64 = &mut (NTRU_SAMPLE_RM_BYTES as u64);
    randombytes(rm_seed, xlen, aes256ctr);

    sample_rm(r, m, *rm_seed);

    poly_s3_tobytes(rm, r);
    let trinary_bytes: &mut [u8; NTRU_OWCPA_MSGBYTES] = &mut [0; NTRU_OWCPA_MSGBYTES];
    trinary_bytes.copy_from_slice(&rm[NTRU_PACK_TRINARY_BYTES..]);
    poly_s3_tobytes(trinary_bytes, m);
    sha3_256(k, rm);

    poly_z3_to_zq(r);
    owcpa_enc(c, r, m, pk);
}

pub fn sha3_256(output: &mut [u8; 32], input: &[u8]) -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let mut sha_3_256_out = [0u8; 32];
    sha3.update(input);
    sha3.finalize(output);
    sha_3_256_out
}

pub fn crypto_kem_dec(
    k: &mut [u8; CRYPTO_BYTES],
    c: &mut [u8; CRYPTO_CIPHERTEXTBYTES],
    sk: &mut [u8; CRYPTO_SECRETKEYBYTES],
) -> i32 {
    let rm: &mut [u8; NTRU_OWCPA_MSGBYTES] = &mut [0; NTRU_OWCPA_MSGBYTES];
    let mut buf: [u8; NTRU_PRFKEYBYTES + NTRU_CIPHERTEXTBYTES] = [0; NTRU_PRFKEYBYTES + NTRU_CIPHERTEXTBYTES];
    let fail = owcpa_dec(rm, c, sk);
    /* If fail = 0 then c = Enc(h, rm). There is no need to re-encapsulate. */
    /* See comment in owcpa_dec for details.                                */

    sha3_256(k, rm);

    /* shake(secret PRF key || input ciphertext) */
    for i in 0..NTRU_PRFKEYBYTES {
        buf[i] = sk[i + NTRU_OWCPA_SECRETKEYBYTES];
    }
    for i in 0..NTRU_CIPHERTEXTBYTES {
        buf[NTRU_PRFKEYBYTES + i] = c[i];
    }
    let mut rm_bytes: [u8; 32] = [0; 32];
    rm_bytes.copy_from_slice(&rm[0..32]);

    sha3_256(&mut rm_bytes, &buf);

    cmov(k, rm, NTRU_SHAREDKEYBYTES as isize, fail as u8);

    0
}

