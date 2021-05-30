use std::convert::TryInto;

use crate::crypto_sort_int32;
use crate::params::{NTRU_HPS, NTRU_HRSS, NTRU_N, NTRU_SAMPLE_FG_BYTES, NTRU_SAMPLE_FT_BYTES, NTRU_SAMPLE_IID_BYTES, NTRU_WEIGHT};
use crate::poly::Poly;
use crate::sample_iid::sample_iid;

pub fn sample_fg(f: &mut Poly, g: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_FG_BYTES]) {
    if NTRU_HRSS {
        let mut bytes: [u8; NTRU_SAMPLE_IID_BYTES] = uniformbytes[..NTRU_N]
            .try_into()
            .expect("Slice has incorrect length.");
        sample_iid_plus(f, bytes);
        bytes = uniformbytes[NTRU_SAMPLE_IID_BYTES..]
            .try_into()
            .expect("Slice has incorrect length.");
        sample_iid_plus(f, bytes);
    }

    if NTRU_HPS {
        let bytes: [u8; NTRU_SAMPLE_IID_BYTES] = uniformbytes[..NTRU_N]
            .try_into()
            .expect("Slice has incorrect length.");
        sample_iid(f, bytes);
        let fixed_type_bytes = uniformbytes[NTRU_SAMPLE_IID_BYTES..]
            .try_into()
            .expect("Slice has incorrect length.");
        sample_fixed_type(g, fixed_type_bytes);
    }
}

pub fn sample_iid_plus(r: &mut Poly, uniformbytes: [u8; NTRU_SAMPLE_IID_BYTES]) {
    /* Sample r using sample then conditionally flip    */
    /* signs of even index coefficients so that <x*r, r> >= 0.      */

    let mut s: u16 = 0;
    sample_iid(r, uniformbytes);

    /* Map {0,1,2} -> {0, 1, 2^16 - 1} */
    for i in 0..(NTRU_N - 1) {
        r.coeffs[i] = r.coeffs[i] | (0 - (r.coeffs[i] >> 1));
    }

    /* s = <x*r, r>.  (r[n-1] = 0) */
    for i in 0..(NTRU_N - 1) {
        s += ((r.coeffs[i + 1] as u32) * (r.coeffs[i] as u32)) as u16;
    }

    /* Extract sign of s (sign(0) = 1) */
    s = 1 | (0 - (s >> 15));


    for i in (0..NTRU_N).step_by(2) {
        r.coeffs[i] = ((s as u32) * (r.coeffs[i] as u32)) as u16;
    }

    /* Map {0,1,2^16-1} -> {0, 1, 2} */
    for i in 0..NTRU_N {
        r.coeffs[i] = 3 & (r.coeffs[i] ^ r.coeffs[i] >> 15)
    }
}

#[allow(arithmetic_overflow)]
fn sample_fixed_type(r: &mut Poly, u: [u8; NTRU_SAMPLE_FT_BYTES]) {
    // Assumes NTRU_SAMPLE_FT_BYTES = ceil(30*(n-1)/8)

    let mut s: [i32; NTRU_N - 1] = [0; NTRU_N - 1];
    let i;

    for i in 0..((NTRU_N - 1) / 4) {
        s[4 * i + 0] = ((u[15 * i + 0] << 2) + (u[15 * i + 1] << 10) + (u[15 * i + 2] << 18) + ((u[15 * i + 3]) << 26)) as i32;
        s[4 * i + 1] = (((u[15 * i + 3] & 0xc0) >> 4) + (u[15 * i + 4] << 4) + (u[15 * i + 5] << 12) + (u[15 * i + 6] << 20) + ((u[15 * i + 7]) << 28)) as i32;
        s[4 * i + 2] = (((u[15 * i + 7] & 0xf0) >> 2) + (u[15 * i + 8] << 6) + (u[15 * i + 9] << 14) + (u[15 * i + 10] << 22) + ((u[15 * i + 11]) << 30)) as i32;
        s[4 * i + 3] = ((u[15 * i + 11] & 0xfc) + (u[15 * i + 12] << 8) + (u[15 * i + 13] << 16) + ((u[15 * i + 14]) << 24)) as i32;
    }
    if (NTRU_N - 1) > ((NTRU_N - 1) / 4) * 4 {
        i = (NTRU_N - 1) / 4;
        s[4 * i + 0] = ((u[15 * i + 0] << 2) + (u[15 * i + 1] << 10) + (u[15 * i + 2] << 18) + ((u[15 * i + 3]) << 26)) as i32;
        s[4 * i + 1] = (((u[15 * i + 3] & 0xc0) >> 4) + (u[15 * i + 4] << 4) + (u[15 * i + 5] << 12) + (u[15 * i + 6] << 20) + ((u[15 * i + 7]) << 28)) as i32;
    }

    for i in 0..(NTRU_WEIGHT / 2) {
        s[i] |= 1;
    }

    for i in (NTRU_WEIGHT / 2)..NTRU_WEIGHT {
        s[i] |= 2;
    }
    crypto_sort_int32::crypto_sort_int32(&mut s, NTRU_N - 1);

    // for(i=0; i<NTRU_N-1; i++)
    // r->coeffs[i] = ((uint16_t) (s[i] & 3));
    for i in 0..(NTRU_N - 1) {
        r.coeffs[i] = (s[i] & 3) as u16;
    }
    r.coeffs[NTRU_N - 1] = 0;
}