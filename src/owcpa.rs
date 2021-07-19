use crate::api::{CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};
use crate::pack3::poly_s3_tobytes;
use crate::packq::{poly_rq_sum_zero_frombytes, poly_rq_sum_zero_tobytes, poly_sq_tobytes};
use crate::params::{NTRU_N, NTRU_OWCPA_MSGBYTES, NTRU_PACK_TRINARY_BYTES, NTRU_SAMPLE_FG_BYTES};
use crate::poly::{poly_rq_inv, poly_sq_mul, poly_z3_to_zq};
use crate::poly::Poly;
use crate::poly_lift::poly_lift;
use crate::poly_rq_mul::poly_rq_mul;
use crate::poly_s3_inv::poly_s3_inv;
use crate::sample::sample_fg;

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

    let mut sk_bytes: [u8; NTRU_OWCPA_MSGBYTES] = [0u8; NTRU_OWCPA_MSGBYTES];
    sk_bytes.copy_from_slice(&sk[..NTRU_OWCPA_MSGBYTES]);
    poly_s3_tobytes(&mut sk_bytes, f);

    sk[..NTRU_OWCPA_MSGBYTES].copy_from_slice(&sk_bytes);

    let mut sk_msgbytes: [u8; NTRU_OWCPA_MSGBYTES] = [0u8; NTRU_OWCPA_MSGBYTES];
    sk_msgbytes.copy_from_slice(&sk[NTRU_PACK_TRINARY_BYTES..NTRU_OWCPA_MSGBYTES + NTRU_PACK_TRINARY_BYTES]);
    poly_s3_tobytes(&mut sk_msgbytes, invf_mod3);
    sk[NTRU_PACK_TRINARY_BYTES..NTRU_OWCPA_MSGBYTES + NTRU_PACK_TRINARY_BYTES].copy_from_slice(&sk_msgbytes);

    /* Lift coeffs of f and g from Z_p to Z_q */
    poly_z3_to_zq(f);
    poly_z3_to_zq(g);

    if cfg!(feature="ntruhrss701") {
        /* g = 3*(x-1)*g */
        // C implementation loops from [NTRU_N - 1;0)
        // .rev() reverses the iterator AFTER the range has been evaluated
        for i in (1..NTRU_N).rev() {
            g.coeffs[i] = 3 * (g.coeffs[i - 1] - g.coeffs[i]);
        }
        g.coeffs[0] = 0 - (3 * g.coeffs[0]);
    }

    if cfg!(any(feature = "ntruhps2048509",
                feature = "ntruhps2048677",
                feature="ntruhps4096821")
        ) {
        /* g = 3*g */
        for i in 0..NTRU_N {
            g.coeffs[i] = 3 * g.coeffs[i];
        }
    }

    poly_rq_mul(&mut x3, g, f);
    poly_rq_inv(invgf, &mut x3);
    poly_rq_mul(tmp, invgf, f);
    poly_sq_mul(&mut x3, tmp, f);

    const SK_PACK_TRINARY_BYTE_SIZE: usize = CRYPTO_SECRETKEYBYTES - 2 * NTRU_PACK_TRINARY_BYTES;
    let mut sk_pack_trinary_bytes: [u8; SK_PACK_TRINARY_BYTE_SIZE] = [0u8; SK_PACK_TRINARY_BYTE_SIZE];
    sk_pack_trinary_bytes.copy_from_slice(&sk[2 * NTRU_PACK_TRINARY_BYTES..]);
    poly_sq_tobytes(&mut sk_pack_trinary_bytes, &mut x3);
    sk[2 * NTRU_PACK_TRINARY_BYTES..].copy_from_slice(&sk_pack_trinary_bytes);

    poly_rq_mul(tmp, invgf, g);
    poly_rq_mul(&mut x3, tmp, g);
    poly_rq_sum_zero_tobytes(pk, &mut x3);
}

pub fn owcpa_enc(c: &mut [u8; CRYPTO_CIPHERTEXTBYTES],
                 r: &mut Poly,
                 m: &mut Poly,
                 pk: &mut [u8; CRYPTO_PUBLICKEYBYTES]) {
    let x1: &mut Poly = &mut Poly::new();
    let x2: &mut Poly = &mut Poly::new();

    // poly *h = &x1, *liftm = &x1;
    // poly *ct = &x2;

    poly_rq_sum_zero_frombytes(x1, pk);

    poly_rq_mul(x2, r, x1);

    poly_lift(x1, m);
    for i in 0..NTRU_N {
        x2.coeffs[i] = x2.coeffs[i] + x1.coeffs[i];
    }
    poly_rq_sum_zero_tobytes(c, x2);
}