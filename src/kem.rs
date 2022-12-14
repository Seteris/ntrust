use tiny_keccak::{Hasher, Sha3};

use crate::api::{
    CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES,
};
use crate::cmov::cmov;
use crate::owcpa::{owcpa_dec, owcpa_enc, owcpa_keypair};
use crate::pack3::poly_s3_tobytes;
use crate::params::{
    NTRU_CIPHERTEXTBYTES, NTRU_OWCPA_MSGBYTES, NTRU_OWCPA_SECRETKEYBYTES, NTRU_PACK_TRINARY_BYTES,
    NTRU_PRFKEYBYTES, NTRU_SAMPLE_FG_BYTES, NTRU_SAMPLE_RM_BYTES, NTRU_SHAREDKEYBYTES,
};
use crate::poly::{poly_z3_to_zq, Poly};
use crate::rng::{randombytes, Aes256CtrDrbgStruct};
use crate::sample::sample_rm;

pub fn crypto_kem_keypair(
    pk: &mut [u8; CRYPTO_PUBLICKEYBYTES],
    sk: &mut [u8; CRYPTO_SECRETKEYBYTES],
    aes256ctrdrbg: &mut Aes256CtrDrbgStruct,
) {
    let mut seed = [0u8; NTRU_SAMPLE_FG_BYTES];
    randombytes(&mut seed, aes256ctrdrbg);
    owcpa_keypair(pk, sk, seed);

    let mut sk_copy = [0u8; NTRU_PRFKEYBYTES];
    sk_copy.copy_from_slice(&sk[NTRU_OWCPA_SECRETKEYBYTES..]);
    randombytes(&mut sk_copy, aes256ctrdrbg);
    sk[NTRU_OWCPA_SECRETKEYBYTES..].copy_from_slice(&sk_copy);
}

pub fn crypto_kem_enc(
    c: &mut [u8; CRYPTO_CIPHERTEXTBYTES],
    k: &mut [u8; CRYPTO_BYTES],
    pk: &[u8; CRYPTO_PUBLICKEYBYTES],
    aes256ctrdrbg: &mut Aes256CtrDrbgStruct,
) {
    let r = &mut Poly::new();
    let m = &mut Poly::new();
    let rm = &mut [0u8; NTRU_OWCPA_MSGBYTES];
    let rm_seed = &mut [0u8; NTRU_SAMPLE_RM_BYTES];

    randombytes(rm_seed, aes256ctrdrbg);

    sample_rm(r, m, *rm_seed);

    poly_s3_tobytes(rm, r);
    let trinary_bytes = &mut [0u8; NTRU_OWCPA_MSGBYTES];
    trinary_bytes[..NTRU_OWCPA_MSGBYTES - NTRU_PACK_TRINARY_BYTES]
        .copy_from_slice(&rm[NTRU_PACK_TRINARY_BYTES..]);
    poly_s3_tobytes(trinary_bytes, m);
    sha3_256(k, rm);

    poly_z3_to_zq(r);
    owcpa_enc(c, r, m, pk);
}

pub fn sha3_256(output: &mut [u8; 32], input: &[u8]) {
    let mut sha3 = Sha3::v256();
    sha3.update(input);
    sha3.finalize(output);
}

pub fn crypto_kem_dec(
    k: &mut [u8; CRYPTO_BYTES],
    c: &[u8; CRYPTO_CIPHERTEXTBYTES],
    sk: &[u8; CRYPTO_SECRETKEYBYTES],
) -> i32 {
    let rm = &mut [0u8; NTRU_OWCPA_MSGBYTES];
    let mut buf = [0u8; NTRU_PRFKEYBYTES + NTRU_CIPHERTEXTBYTES];
    let fail = owcpa_dec(rm, c, sk);
    /* If fail = 0 then c = Enc(h, rm). There is no need to re-encapsulate. */
    /* See comment in owcpa_dec for details.                                */

    sha3_256(k, rm);

    /* shake(secret PRF key || input ciphertext) */
    buf[..NTRU_PRFKEYBYTES].clone_from_slice(
        &sk[NTRU_OWCPA_SECRETKEYBYTES..(NTRU_PRFKEYBYTES + NTRU_OWCPA_SECRETKEYBYTES)],
    );
    buf[NTRU_PRFKEYBYTES..(NTRU_CIPHERTEXTBYTES + NTRU_PRFKEYBYTES)]
        .clone_from_slice(&c[..NTRU_CIPHERTEXTBYTES]);
    let mut rm_bytes = [0u8; 32];
    rm_bytes.copy_from_slice(&rm[0..32]);

    sha3_256(&mut rm_bytes, &buf);

    cmov(k, rm, NTRU_SHAREDKEYBYTES as isize, fail as u8);

    0
}
