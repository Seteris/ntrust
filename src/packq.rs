use crate::params::{NTRU_N, NTRU_PACK_DEG};
use crate::poly::Poly;
use crate::poly::MODQ;

#[cfg(any(feature = "ntruhps2048509", feature = "ntruhps2048677"))]
#[allow(arithmetic_overflow)]
pub fn poly_sq_tobytes(r: &mut [u8], a: &mut Poly) {
    let mut t = [0u16; 8];
    for i in 0..NTRU_PACK_DEG / 8 {
        for (j, val) in t.iter_mut().enumerate() {
            *val = MODQ(a.coeffs[8 * i + j]);
        }
        r[11 * i] = t[0] as u8;
        r[11 * i + 2] = (((t[1]).wrapping_shr(5)) | ((t[2] & 0x03).wrapping_shl(6))) as u8;
        r[11 * i + 1] = (((t[0]).wrapping_shr(8)) | ((t[1] & 0x1f).wrapping_shl(3))) as u8;
        r[11 * i + 3] = (((t[2]).wrapping_shr(2)) & 0xff) as u8;
        r[11 * i + 4] = (((t[2]).wrapping_shr(10)) | ((t[3] & 0x7f).wrapping_shl(1))) as u8;
        r[11 * i + 5] = (((t[3]).wrapping_shr(7)) | ((t[4] & 0x0f).wrapping_shl(4))) as u8;
        r[11 * i + 6] = (((t[4]).wrapping_shr(4)) | ((t[5] & 0x01).wrapping_shl(7))) as u8;
        r[11 * i + 7] = (((t[5]).wrapping_shr(1)) & 0xff) as u8;
        r[11 * i + 8] = (((t[5]).wrapping_shr(9)) | ((t[6] & 0x3f).wrapping_shl(2))) as u8;
        r[11 * i + 9] = (((t[6]).wrapping_shr(6)) | ((t[7] & 0x07).wrapping_shl(5))) as u8;
        r[11 * i + 10] = (t[7]).wrapping_shr(3) as u8;
    }

    let i = NTRU_PACK_DEG as i16 / 8;
    for j in 0..NTRU_PACK_DEG as i16 - 8 * i {
        t[j as usize] = MODQ(a.coeffs[(8 * i + j) as usize]);
    }
    let j = NTRU_PACK_DEG as i16 - 8 * i;
    for x in j..8 {
        t[x as usize] = 0;
    }

    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r[(11 * i) as usize] = (t[0] & 0xff) as u8;
            r[(11 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3)) as u8;
            r[(11 * i + 2) as usize] = ((t[1] >> 5) | ((t[2] & 0x03) << 6)) as u8;
            r[(11 * i + 3) as usize] = ((t[2] >> 2) & 0xff) as u8;
            r[(11 * i + 4) as usize] = ((t[2] >> 10) | ((t[3] & 0x7f) << 1)) as u8;
            r[(11 * i + 5) as usize] = ((t[3] >> 7) | ((t[4] & 0x0f) << 4)) as u8;
        }
        2 => {
            r[(11 * i) as usize] = (t[0] & 0xff) as u8;
            r[(11 * i + 1) as usize] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3)) as u8;
            r[(11 * i + 2) as usize] = ((t[1] >> 5) | ((t[2] & 0x03) << 6)) as u8;
        }
        _ => {}
    }
}

#[cfg(feature = "ntruhps4096821")]
pub fn poly_sq_tobytes(r: &mut [u8], a: &mut Poly) {
    for i in 0..NTRU_PACK_DEG / 2 {
        r[3 * i + 0] = (MODQ(a.coeffs[2 * i + 0] as u16) & 0xff) as u8;
        r[3 * i + 1] =
            ((MODQ(a.coeffs[2 * i + 0]) >> 8) | ((MODQ(a.coeffs[2 * i + 1]) & 0x0f) << 4)) as u8;
        r[3 * i + 2] = (MODQ(a.coeffs[2 * i + 1]) >> 4) as u8;
    }
}

#[cfg(feature = "ntruhrss701")]
pub fn poly_sq_tobytes(r: &mut [u8], a: &mut Poly) {
    let mut t = [0u16; 8];

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

    let i = (NTRU_PACK_DEG / 8) as i16;
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

#[cfg(any(feature = "ntruhps2048509", feature = "ntruhps2048677"))]
#[allow(arithmetic_overflow)]
pub fn poly_sq_frombytes(r: &mut Poly, a: &[u8]) {
    for i in 0..(NTRU_PACK_DEG / 8) {
        r.coeffs[8 * i] = (a[11 * i] | ((a[11 * i + 1] & 0x07) << 8)) as u16;
        r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
        r.coeffs[8 * i + 2] =
            ((a[11 * i + 2] >> 6) | ((a[11 * i + 3]) << 2) | ((a[11 * i + 4] & 0x01) << 10)) as u16;
        r.coeffs[8 * i + 3] = ((a[11 * i + 4] >> 1) | ((a[11 * i + 5] & 0x0f) << 7)) as u16;
        r.coeffs[8 * i + 4] = ((a[11 * i + 5] >> 4) | ((a[11 * i + 6] & 0x7f) << 4)) as u16;
        r.coeffs[8 * i + 5] =
            ((a[11 * i + 6] >> 7) | ((a[11 * i + 7]) << 1) | ((a[11 * i + 8] & 0x03) << 9)) as u16;
        r.coeffs[8 * i + 6] = ((a[11 * i + 8] >> 2) | ((a[11 * i + 9] & 0x1f) << 6)) as u16;
        r.coeffs[8 * i + 7] = ((a[11 * i + 9] >> 5) | ((a[11 * i + 10]) << 3)) as u16;
    }
    let i = (NTRU_PACK_DEG / 8) - 1;
    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r.coeffs[8 * i] = (a[11 * i] | ((a[11 * i + 1] & 0x07) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
            r.coeffs[8 * i + 2] = ((a[11 * i + 2] >> 6)
                | (a[11 * i + 3] << 2)
                | ((a[11 * i + 4] & 0x01) << 10)) as u16;
            r.coeffs[8 * i + 3] = ((a[11 * i + 4] >> 1) | ((a[11 * i + 5] & 0x0f) << 7)) as u16;
        }
        2 => {
            r.coeffs[8 * i] = (a[11 * i] | ((a[11 * i + 1] & 0x07) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[11 * i + 1] >> 3) | ((a[11 * i + 2] & 0x3f) << 5)) as u16;
        }
        _ => {}
    }
    r.coeffs[NTRU_N - 1] = 0;
}

#[cfg(feature = "ntruhps4096821")]
#[allow(arithmetic_overflow)]
pub fn poly_sq_frombytes(r: &mut Poly, a: &[u8]) {
    for i in 0..NTRU_PACK_DEG / 2 {
        r.coeffs[2 * i] = a[3 * i] as u16 | ((a[3 * i + 1] & 0x0f) << 8) as u16;
        r.coeffs[2 * i + 1] = a[3 * i] as u16 | ((a[3 * i + 1] & 0x0f) << 8) as u16;
    }
    r.coeffs[NTRU_N - 1] = 0;
}

#[cfg(feature = "ntruhrss701")]
#[allow(arithmetic_overflow)]
pub fn poly_sq_frombytes(r: &mut Poly, a: &[u8]) {
    for i in 0..(NTRU_PACK_DEG / 8) {
        r.coeffs[8 * i + 0] = (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
        r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5)
            | ((a[13 * i + 2] as u16) << 3)
            | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
        r.coeffs[8 * i + 2] =
            ((a[13 * i + 3] as u16 >> 2) | ((a[13 * i + 4] as u16 & 0x7f) << 6)) as u16;
        r.coeffs[8 * i + 3] = ((a[13 * i + 4] as u16 >> 7)
            | ((a[13 * i + 5] as u16) << 1)
            | ((a[13 * i + 6] as u16 & 0x0f) << 9)) as u16;
        r.coeffs[8 * i + 4] = ((a[13 * i + 6] as u16 >> 4)
            | ((a[13 * i + 7] as u16) << 4)
            | ((a[13 * i + 8] as u16 & 0x01) << 12)) as u16;
        r.coeffs[8 * i + 5] =
            ((a[13 * i + 8] as u16 >> 1) | ((a[13 * i + 9] as u16 & 0x3f) << 7)) as u16;
        r.coeffs[8 * i + 6] = ((a[13 * i + 9] as u16 >> 6)
            | ((a[13 * i + 10] as u16) << 2)
            | ((a[13 * i + 11] as u16 & 0x07) << 10)) as u16;
        r.coeffs[8 * i + 7] =
            ((a[13 * i + 11] as u16 >> 3) | ((a[13 * i + 12] as u16) << 5)) as u16;
    }
    let i = (NTRU_PACK_DEG / 8) - 1;
    match NTRU_PACK_DEG & 0x07 {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r.coeffs[8 * i + 0] =
                (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5)
                | ((a[13 * i + 2] as u16) << 3)
                | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
            r.coeffs[8 * i + 2] =
                ((a[13 * i + 3] as u16 >> 2) | ((a[13 * i + 4] as u16 & 0x7f) << 6)) as u16;
            r.coeffs[8 * i + 3] = ((a[13 * i + 4] as u16 >> 7)
                | ((a[13 * i + 5] as u16) << 1)
                | ((a[13 * i + 6] as u16 & 0x0f) << 9)) as u16;
        }
        2 => {
            r.coeffs[8 * i + 0] =
                (a[13 * i + 0] as u16 | ((a[13 * i + 1] as u16 & 0x1f) << 8)) as u16;
            r.coeffs[8 * i + 1] = ((a[13 * i + 1] as u16 >> 5)
                | ((a[13 * i + 2] as u16) << 3)
                | ((a[13 * i + 3] as u16 & 0x03) << 11)) as u16;
        }
        _ => {}
    }
    r.coeffs[NTRU_N - 1] = 0;
}

pub fn poly_rq_sum_zero_tobytes(r: &mut [u8], a: &mut Poly) {
    poly_sq_tobytes(r, a);
}

pub fn poly_rq_sum_zero_frombytes(r: &mut Poly, a: &[u8]) {
    poly_sq_frombytes(r, a);

    /* Set r[n-1] so that the sum of coefficients is zero mod q */
    r.coeffs[NTRU_N - 1] = 0;
    for i in 0..NTRU_PACK_DEG {
        r.coeffs[NTRU_N - 1] -= r.coeffs[i];
    }
}
