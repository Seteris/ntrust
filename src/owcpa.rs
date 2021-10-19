use crate::api::{CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES};
use crate::pack3::{poly_s3_frombytes, poly_s3_tobytes};
use crate::packq::{poly_rq_sum_zero_frombytes, poly_rq_sum_zero_tobytes, poly_sq_frombytes, poly_sq_tobytes};
use crate::params::{NTRU_CIPHERTEXTBYTES, NTRU_LOGQ, NTRU_N, NTRU_OWCPA_MSGBYTES, NTRU_PACK_DEG, NTRU_PACK_TRINARY_BYTES, NTRU_Q, NTRU_SAMPLE_FG_BYTES, NTRU_WEIGHT};
use crate::poly::{poly_rq_inv, poly_s3_mul, poly_sq_mul, poly_trinary_zq_to_z3, poly_z3_to_zq};
use crate::poly::Poly;
use crate::poly_lift::poly_lift;
use crate::poly_mod::poly_rq_to_s3;
use crate::poly_rq_mul::poly_rq_mul;
use crate::poly_s3_inv::poly_s3_inv;
use crate::sample::sample_fg;

pub fn owcpa_check_ciphertext(ciphertext: &[u8]) -> u16 {
    /* A ciphertext is log2(q)*(n-1) bits packed into bytes.  */
    /* Check that any unused bits of the final byte are zero. */

    let mut t: u16 = ciphertext[NTRU_CIPHERTEXTBYTES - 1] as u16;
    t &= 0xff << (8 - (7 & (NTRU_LOGQ * NTRU_PACK_DEG)));

    /* We have 0 <= t < 256 */
    /* Return 0 on success (t=0), 1 on failure */
    1 & ((!t + 1) >> 15)
}

pub fn owcpa_check_r(r: &Poly) -> u32 {
    /* A valid r has coefficients in {0,1,q-1} and has r[N-1] = 0 */
    /* Note: We may assume that 0 <= r[i] <= q-1 for all i        */
    let mut t: u32 = 0;
    for i in 0..NTRU_N - 1 {
        let c = r.coeffs[i];
        t |= (c as u32 + 1) & (NTRU_Q as u32 - 4); /* 0 iff c is in {-1,0,1,2} */
        t |= (c as u32 + 2) & 4; /* 1 if c = 2, 0 if c is in {-1,0,1} */
    }
    t |= r.coeffs[NTRU_N - 1] as u32; /* Coefficient n-1 must be zero */

    /* We have 0 <= t < 2^16. */
    /* Return 0 on success (t=0), 1 on failure */
    1 & ((!t + 1) >> 31)
}

#[cfg(feature = "ntruhps")]
pub fn owcpa_check_m(m: &Poly) -> u32 {
    /* Check that m is in message space, i.e.                  */
    /*  (1)  |{i : m[i] = 1}| = |{i : m[i] = 2}|, and          */
    /*  (2)  |{i : m[i] != 0}| = NTRU_WEIGHT.                  */
    /* Note: We may assume that m has coefficients in {0,1,2}. */

    let mut t: u32 = 0;
    let mut ps: u16 = 0;
    let mut ms: u16 = 0;

    for i in 0..NTRU_N {
        ps += m.coeffs[i] & 1;
        ms += m.coeffs[i] & 2;
    }
    t |= (ps ^ (ms >> 1)) as u32; /* 0 if (1) holds */
    t |= ms as u32 ^ NTRU_WEIGHT as u32; /* 0 if (1) and (2) hold */

    /* We have 0 <= t < 2^16. */
    /* Return 0 on success (t=0), 1 on failure */
    1 & ((!t + 1) >> 31)
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn owcpa_keypair(pk: &mut [u8; CRYPTO_PUBLICKEYBYTES],
                     sk: &mut [u8; CRYPTO_SECRETKEYBYTES],
                     seed: [u8; NTRU_SAMPLE_FG_BYTES]) {
    log!("starting owcpa_keypair");
    let mut x3: Poly = Poly::new();

    log!("pk = {:?}", pk);
    log!("sk = {:?}", sk);
    log!("seed = {:?}", seed);

    let f: &mut Poly = &mut Poly::new();
    let g: &mut Poly = &mut Poly::new();

    let invgf: &mut Poly = &mut Poly::new();
    let tmp: &mut Poly = &mut Poly::new();
    // let invf_mod3: &mut Poly = &mut x3;
    // let gf: &mut Poly = &mut x3;
    // let invh: &mut Poly = &mut x3;
    // let h: &mut Poly = &mut x3;

    sample_fg(f, g, seed);
    poly_s3_inv(&mut x3, f);

    let mut sk_bytes: [u8; NTRU_OWCPA_MSGBYTES] = [0u8; NTRU_OWCPA_MSGBYTES];
    sk_bytes.copy_from_slice(&sk[..NTRU_OWCPA_MSGBYTES]);
    poly_s3_tobytes(&mut sk_bytes, f);

    sk[..NTRU_OWCPA_MSGBYTES].copy_from_slice(&sk_bytes);

    let mut sk_msgbytes: [u8; NTRU_OWCPA_MSGBYTES] = [0u8; NTRU_OWCPA_MSGBYTES];
    sk_msgbytes.copy_from_slice(&sk[NTRU_PACK_TRINARY_BYTES..NTRU_OWCPA_MSGBYTES + NTRU_PACK_TRINARY_BYTES]);
    poly_s3_tobytes(&mut sk_msgbytes, &mut x3);
    sk[NTRU_PACK_TRINARY_BYTES..NTRU_OWCPA_MSGBYTES + NTRU_PACK_TRINARY_BYTES].copy_from_slice(&sk_msgbytes);

    #[cfg(feature = "ntruhrss701")] {
        /* g = 3*(x-1)*g */
        // C implementation loops from [NTRU_N - 1;0)
        // .rev() reverses the iterator AFTER the range has been evaluated
        for i in (1..NTRU_N).rev() {
            g.coeffs[i] = 3 * (g.coeffs[i - 1] - g.coeffs[i]);
        }
        g.coeffs[0] = 0 - (3 * g.coeffs[0]);
    }

    #[cfg(feature = "ntruhps")] {
        /* g = 3*g */
        for i in 0..NTRU_N {
            g.coeffs[i] = 3 * g.coeffs[i];
        }
    }
    log!("x3 = {:?}", x3.coeffs);
    // FIXME: check x3 correctness in poly_rq_mul
    // FIXME: g.coeffs values after g = 3 * g
    poly_rq_mul(&mut x3, g, f);
    log!("x3 = {:?}", x3.coeffs);
    log!("g = {:?}", g.coeffs);
    log!("f = {:?}", f.coeffs);
    /* Lift coeffs of f and g from Z_p to Z_q */
    poly_z3_to_zq(f);
    poly_z3_to_zq(g);
    poly_rq_inv(invgf, &x3);
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
    log!("finished owcpa_keypair");
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

pub fn owcpa_dec(rm: &mut [u8], ciphertext: &[u8], secretkey: &[u8; CRYPTO_SECRETKEYBYTES]) -> u16 {
    let x1: &mut Poly = &mut Poly::new();
    let x2: &mut Poly = &mut Poly::new();
    let x3: &mut Poly = &mut Poly::new();
    let x4: &mut Poly = &mut Poly::new();

    //   poly *c = &x1, *f = &x2, *cf = &x3;
    //   poly *mf = &x2, *finv3 = &x3, *m = &x4;
    //   poly *liftm = &x2, *invh = &x3, *r = &x4;
    //   poly *b = &x1;

    poly_rq_sum_zero_frombytes(x1, ciphertext);
    let mut sk_msgbytes: [u8; NTRU_OWCPA_MSGBYTES] = [0; NTRU_OWCPA_MSGBYTES];
    sk_msgbytes.copy_from_slice(&secretkey[0..NTRU_OWCPA_MSGBYTES]);
    poly_s3_frombytes(x2, sk_msgbytes);
    poly_z3_to_zq(x2);

    poly_rq_mul(x3, x1, x2);
    poly_rq_to_s3(x2, x3);

    let mut sk_trinary_bytes: [u8; NTRU_OWCPA_MSGBYTES] = [0; NTRU_OWCPA_MSGBYTES];
    sk_trinary_bytes.copy_from_slice(&secretkey[NTRU_PACK_TRINARY_BYTES..NTRU_PACK_TRINARY_BYTES + NTRU_OWCPA_MSGBYTES]);
    poly_s3_frombytes(x3, sk_trinary_bytes);
    poly_s3_mul(x4, x2, x3);
    let mut ntru_pack_trinary_bytes: [u8; NTRU_OWCPA_MSGBYTES] = [0; NTRU_OWCPA_MSGBYTES];
    ntru_pack_trinary_bytes.copy_from_slice(&rm[NTRU_PACK_TRINARY_BYTES..NTRU_PACK_TRINARY_BYTES + NTRU_OWCPA_MSGBYTES]);
    poly_s3_tobytes(&mut ntru_pack_trinary_bytes, x4);
    rm[NTRU_PACK_TRINARY_BYTES..NTRU_PACK_TRINARY_BYTES + NTRU_OWCPA_MSGBYTES].copy_from_slice(&ntru_pack_trinary_bytes);

    /* Check that the unused bits of the last byte of the ciphertext are zero */
    let mut fail = 0 | owcpa_check_ciphertext(ciphertext);

    /* For the IND-CCA2 KEM we must ensure that c = Enc(h, (r,m)).             */
    /* We can avoid re-computing r*h + Lift(m) as long as we check that        */
    /* r (defined as b/h mod (q, Phi_n)) and m are in the message space.       */
    /* (m can take any value in S3 in NTRU_HRSS) */
    #[cfg(feature = "ntruhps")] {
        fail |= owcpa_check_m(x4) as u16;
    }

    /* b = c - Lift(m) mod (q, x^n - 1) */
    poly_lift(x2, x4);
    for i in 0..NTRU_N {
        x1.coeffs[i] = x1.coeffs[i] - x2.coeffs[i];
    }

    /* r = b / h mod (q, Phi_n) */
    let mut sq_frombytes: [u8; CRYPTO_SECRETKEYBYTES - 2 * NTRU_PACK_TRINARY_BYTES] = [0; CRYPTO_SECRETKEYBYTES - 2 * NTRU_PACK_TRINARY_BYTES];
    sq_frombytes.copy_from_slice(&secretkey[2 * NTRU_PACK_TRINARY_BYTES..]);
    poly_sq_frombytes(x3, &sq_frombytes);
    poly_sq_mul(x4, x1, x3);

    /* NOTE: Our definition of r as b/h mod (q, Phi_n) follows Figure 4 of     */
    /*   [Sch18] https://eprint.iacr.org/2018/1174/20181203:032458.            */
    /* This differs from Figure 10 of Saito--Xagawa--Yamakawa                  */
    /*   [SXY17] https://eprint.iacr.org/2017/1005/20180516:055500             */
    /* where r gets a final reduction modulo p.                                */
    /* We need this change to use Proposition 1 of [Sch18].                    */

    /* Proposition 1 of [Sch18] shows that re-encryption with (r,m) yields c.  */
    /* if and only if fail==0 after the following call to owcpa_check_r        */
    /* The procedure given in Fig. 8 of [Sch18] can be skipped because we have */
    /* c(1) = 0 due to the use of poly_Rq_sum_zero_{to,from}bytes.             */
    fail |= owcpa_check_r(x4) as u16;

    poly_trinary_zq_to_z3(x4);
    let mut s3_tobytes: [u8; NTRU_OWCPA_MSGBYTES] = [0; NTRU_OWCPA_MSGBYTES];
    s3_tobytes.copy_from_slice(&rm[..NTRU_OWCPA_MSGBYTES]);
    poly_s3_tobytes(&mut s3_tobytes, x4);

    fail
}