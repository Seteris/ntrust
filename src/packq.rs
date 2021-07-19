use crate::params::{NTRU_N, NTRU_PACK_DEG};
use crate::poly::MODQ;
use crate::poly::Poly;

#[cfg(any(feature = "ntruhps2048509", feature = "ntruhps2048677", feature = "ntruhps4096821"))]
pub fn poly_sq_tobytes(r: &mut [u8],
                       a: &mut Poly) {
    let mut t: [u16; 8] = [0; 8];

    let i: i16;

    for i in 0..NTRU_PACK_DEG / 8 {
        for j in 0..8 {
            t[j] = MODQ(a.coeffs[8 * i + j]);
        }
        r[11 * i + 0] = (t[0] & 0xff) as u8;
        r[11 * i + 1] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3)) as u8;
        r[11 * i + 2] = ((t[1] >> 5) | ((t[2] & 0x03) << 6)) as u8;
        r[11 * i + 3] = ((t[2] >> 2) & 0xff) as u8;
        r[11 * i + 4] = ((t[2] >> 10) | ((t[3] & 0x7f) << 1)) as u8;
        r[11 * i + 5] = ((t[3] >> 7) | ((t[4] & 0x0f) << 4)) as u8;
        r[11 * i + 6] = ((t[4] >> 4) | ((t[5] & 0x01) << 7)) as u8;
        r[11 * i + 7] = ((t[5] >> 1) & 0xff) as u8;
        r[11 * i + 8] = ((t[5] >> 9) | ((t[6] & 0x3f) << 2)) as u8;
        r[11 * i + 9] = ((t[6] >> 6) | ((t[7] & 0x07) << 5)) as u8;
        r[11 * i + 10] = (t[7] >> 3) as u8;
    }

    i = (NTRU_PACK_DEG / 8) as i16;
    for j in 0..(NTRU_PACK_DEG as i16 - 8 * i) {
        t[j as usize] = MODQ(a.coeffs[(8 * i + j) as usize]);
    }
    let j = (NTRU_PACK_DEG as i16 - 8 * i) - 1;
    for x in j + 1..8 {
        t[x as usize] = 0;
    }

    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r[(11 * i + 0) as usize] = (t[0] & 0xff) as u8;
            r[(11 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3)) as u8;
            r[(11 * i + 2) as usize] = ((t[1] >> 5) | ((t[2] & 0x03) << 6)) as u8;
            r[(11 * i + 3) as usize] = ((t[2] >> 2) & 0xff) as u8;
            r[(11 * i + 4) as usize] = ((t[2] >> 10) | ((t[3] & 0x7f) << 1)) as u8;
            r[(11 * i + 5) as usize] = ((t[3] >> 7) | ((t[4] & 0x0f) << 4)) as u8;
        }
        2 => {
            r[(11 * i + 0) as usize] = (t[0] & 0xff) as u8;
            r[(11 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3)) as u8;
            r[(11 * i + 2) as usize] = ((t[1] >> 5) | ((t[2] & 0x03) << 6)) as u8;
        }
        _ => {}
    }
}

#[cfg(feature = "ntruhrss701")]
pub fn poly_sq_tobytes(r: &mut [u8],
                       a: &mut Poly) {
    let mut t: [u16; 8] = [0; 8];

    let i: i16;

    for i in 0..NTRU_PACK_DEG / 8 {
        for j in 0..8 {
            t[j] = MODQ(a.coeffs[8 * i + j]);
        }
        r[13 * i + 0] = (t[0] & 0xff) as u8;
        r[13 * i + 1] = ((t[0] >> 8) | ((t[1] & 0x07) << 5)) as u8;
        r[13 * i + 2] = ((t[1] >> 3) & 0xff) as u8;
        r[13 * i + 3] = ((t[1] >> 11) | ((t[2] & 0x3f) << 2)) as u8;
        r[13 * i + 4] = ((t[2] >> 6) | ((t[3] & 0x01) << 7)) as u8;
        r[13 * i + 5] = ((t[3] >> 1) & 0xff) as u8;
        r[13 * i + 6] = ((t[3] >> 9) | ((t[4] & 0x0f) << 4)) as u8;
        r[13 * i + 7] = ((t[4] >> 4) & 0xff) as u8;
        r[13 * i + 8] = ((t[4] >> 12) | ((t[5] & 0x7f) << 1)) as u8;
        r[13 * i + 9] = ((t[5] >> 7) | ((t[6] & 0x03) << 6)) as u8;
        r[13 * i + 10] = ((t[6] >> 2) & 0xff) as u8;
        r[13 * i + 11] = ((t[6] >> 10) | ((t[7] & 0x1f) << 3)) as u8;
        r[13 * i + 12] = (t[7] >> 5) as u8;
    }

    i = (NTRU_PACK_DEG / 8) as i16;
    for j in 0..(NTRU_PACK_DEG as i16 - 8 * i) {
        t[j as usize] = MODQ(a.coeffs[(8 * i + j) as usize]);
    }
    let j = (NTRU_PACK_DEG as i16 - 8 * i) - 1;
    for x in j + 1..8 {
        t[x as usize] = 0;
    }

    match NTRU_PACK_DEG - 8 * (NTRU_PACK_DEG / 8) {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r[(13 * i + 0) as usize] = (t[0] & 0xff) as u8;
            r[(13 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x07) << 5)) as u8;
            r[(13 * i + 2) as usize] = ((t[1] >> 3) & 0xff) as u8;
            r[(13 * i + 3) as usize] = ((t[1] >> 11) | ((t[2] & 0x3f) << 2)) as u8;
            r[(13 * i + 4) as usize] = ((t[2] >> 6) | ((t[3] & 0x01) << 7)) as u8;
            r[(13 * i + 5) as usize] = ((t[3] >> 1) & 0xff) as u8;
            r[(13 * i + 6) as usize] = ((t[3] >> 9) | ((t[4] & 0x0f) << 4)) as u8;
        }
        2 => {
            r[(13 * i + 0) as usize] = (t[0] & 0xff) as u8;
            r[(13 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x07) << 5)) as u8;
            r[(13 * i + 2) as usize] = ((t[1] >> 3) & 0xff) as u8;
            r[(13 * i + 3) as usize] = ((t[1] >> 11) | ((t[2] & 0x3f) << 2)) as u8;
        }
        _ => {}
    }
}

#[cfg(any(feature = "ntruhps2048509", feature = "ntruhps2048677", feature = "ntruhps4096821"))]
pub fn poly_sq_frombytes(r: &mut Poly, a: &mut [u8]) {
    let i = (NTRU_PACK_DEG / 8) - 1;
    for i in 0..(NTRU_PACK_DEG / 8) {
        r.coeffs[8 * i + 0] = ((a[11 * i + 0] >> 0) | ((a[11 * i + 1] & 0x07) << 8)) as u16;
        r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
        r.coeffs[8 * i + 2] = ((a[11 * i + 2] >> 6) | ((a[11 * i + 3] & 0xff) << 2) | ((a[11 * i + 4] & 0x01) << 10)) as u16;
        r.coeffs[8 * i + 3] = ((a[11 * i + 4] >> 1) | ((a[11 * i + 5] & 0x0f) << 7)) as u16;
        r.coeffs[8 * i + 4] = ((a[11 * i + 5] >> 4) | ((a[11 * i + 6] & 0x7f) << 4)) as u16;
        r.coeffs[8 * i + 5] = ((a[11 * i + 6] >> 7) | ((a[11 * i + 7] & 0xff) << 1) | ((a[11 * i + 8] & 0x03) << 9)) as u16;
        r.coeffs[8 * i + 6] = ((a[11 * i + 8] >> 2) | ((a[11 * i + 9] & 0x1f) << 6)) as u16;
        r.coeffs[8 * i + 7] = ((a[11 * i + 9] >> 5) | ((a[11 * i + 10] & 0xff) << 3)) as u16;
    }
    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r.coeffs[8 * i + 0] = ((a[11 * i + 0] >> 0) | ((a[11 * i + 1] & 0x07) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
            r.coeffs[8 * i + 2] = ((a[11 * i + 2] >> 6) | ((a[11 * i + 3] & 0xff) << 2) | ((a[11 * i + 4] & 0x01) << 10)) as u16;
            r.coeffs[8 * i + 3] = ((a[11 * i + 4] >> 1) | ((a[11 * i + 5] & 0x0f) << 7)) as u16;
        }
        2 => {
            r.coeffs[8 * i + 0] = ((a[11 * i + 0] >> 0) | ((a[11 * i + 1] & 0x07) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
        }
        _ => {}
    }
    r.coeffs[NTRU_N - 1] = 0;
}

#[cfg(feature = "ntruhrss701")]
pub fn poly_sq_frombytes(r: &mut Poly, a: &mut [u8]) {
    let i = (NTRU_PACK_DEG / 8) - 1;
    for i in 0..(NTRU_PACK_DEG / 8) {
        r.coeffs[8 * i + 0] = (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
        r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5) | ((a[13 * i + 2] as u16) << 3) | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
        r.coeffs[8 * i + 2] = ((a[13 * i + 3] as u16 >> 2) | ((a[13 * i + 4] as u16 & 0x7f) << 6)) as u16;
        r.coeffs[8 * i + 3] = ((a[13 * i + 4] as u16 >> 7) | ((a[13 * i + 5] as u16) << 1) | ((a[13 * i + 6] as u16 & 0x0f) << 9)) as u16;
        r.coeffs[8 * i + 4] = ((a[13 * i + 6] as u16 >> 4) | ((a[13 * i + 7] as u16) << 4) | ((a[13 * i + 8] as u16 & 0x01) << 12)) as u16;
        r.coeffs[8 * i + 5] = ((a[13 * i + 8] as u16 >> 1) | ((a[13 * i + 9] as u16 & 0x3f) << 7)) as u16;
        r.coeffs[8 * i + 6] = ((a[13 * i + 9] as u16 >> 6) | ((a[13 * i + 10] as u16) << 2) | ((a[13 * i + 11] as u16 & 0x07) << 10)) as u16;
        r.coeffs[8 * i + 7] = ((a[13 * i + 11] as u16 >> 3) | ((a[13 * i + 12] as u16) << 5)) as u16;
    }
    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r.coeffs[8 * i + 0] = (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5) | ((a[13 * i + 2] as u16) << 3) | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
            r.coeffs[8 * i + 2] = ((a[13 * i + 3] as u16 >> 2) | ((a[13 * i + 4] as u16 & 0x7f) << 6)) as u16;
            r.coeffs[8 * i + 3] = ((a[13 * i + 4] as u16 >> 7) | ((a[13 * i + 5] as u16) << 1) | ((a[13 * i + 6] as u16 & 0x0f) << 9)) as u16;
        }
        2 => {
            r.coeffs[8 * i + 0] = (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5) | ((a[13 * i + 2] as u16) << 3) | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
        }
        _ => {}
    }
    r.coeffs[NTRU_N - 1] = 0;
}

pub fn poly_rq_sum_zero_tobytes(r: &mut [u8], a: &mut Poly) {
    poly_sq_tobytes(r, a);
}

pub fn poly_rq_sum_zero_frombytes(r: &mut Poly, a: &mut [u8]) {
    poly_sq_frombytes(r, a);

    /* Set r[n-1] so that the sum of coefficients is zero mod q */
    r.coeffs[NTRU_N - 1] = 0;
    for i in 0..NTRU_PACK_DEG {
        r.coeffs[NTRU_N - 1] -= r.coeffs[i];
    }
}