use crate::params::NTRU_PACK_DEG;
use crate::poly::MODQ;
use crate::poly::Poly;

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

    i = ((NTRU_PACK_DEG / 8) - 1) as i16;

    for j in 0..NTRU_PACK_DEG as i16 - 8 * i {
        t[j as usize] = MODQ(a.coeffs[(8 * i + j) as usize]);
    }
    let j = (NTRU_PACK_DEG as i16 - 8 * i) - 1;
    for _ in j..8 {
        t[j as usize] = 0;
    }

    match NTRU_PACK_DEG {
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

pub fn poly_rq_sum_zero_tobytes(r: &mut [u8], a: &mut Poly) {
    poly_sq_tobytes(r, a);
}