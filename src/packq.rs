use crate::params::{NTRU_PACK_DEG, NTRU_Q};
use crate::poly::MODQ;

pub fn poly_sq_tobytes(mut pk: &[u8],
                       mut sk: &[u8]) {
    let mut t: [u16; 8] = [0; 8];

    let i: i16;

    for i in 0..NTRU_PACK_DEG / 8 {
        for j in 0..8 {
            t[j] = MODQ(a.coeffs[8 * i + j]);
        }
        r[11 * i + 0] = (t[0] & 0xff);
        r[11 * i + 1] = ((t[0] >> 8) | ((t[1] & 0x1f) << 3));
        r[11 * i + 2] = ((t[1] >> 5) | ((t[2] & 0x03) << 6));
        r[11 * i + 3] = ((t[2] >> 2) & 0xff);
        r[11 * i + 4] = ((t[2] >> 10) | ((t[3] & 0x7f) << 1));
        r[11 * i + 5] = ((t[3] >> 7) | ((t[4] & 0x0f) << 4));
        r[11 * i + 6] = ((t[4] >> 4) | ((t[5] & 0x01) << 7));
        r[11 * i + 7] = ((t[5] >> 1) & 0xff);
        r[11 * i + 8] = ((t[5] >> 9) | ((t[6] & 0x3f) << 2));
        r[11 * i + 9] = ((t[6] >> 6) | ((t[7] & 0x07) << 5));
        r[11 * i + 10] = (t[7] >> 3);
    }

    i = ((NTRU_PACK_DEG / 8) - 1) as i16;

    for j in 0..NTRU_PACK_DEG - 8 * i {
        t[j] = MODQ(a.coeffs[8 * i + j]);
    }
    let j = (NTRU_PACK_DEG - 8 * i) - 1;
    for x in j..8 {
        t[j] = 0;
    }

    match NTRU_PACK_DEG {
        // cases 0 and 6 are impossible since 2 generates (Z/n)* and
        // p mod 8 in {1, 7} implies that 2 is a quadratic residue.
        4 => {
            r[11 * i + 0] = t[0] & 0xff;
            r[11 * i + 1] = (t[0] >> 8) | ((t[1] & 0x1f) << 3);
            r[11 * i + 2] = (t[1] >> 5) | ((t[2] & 0x03) << 6);
            r[11 * i + 3] = (t[2] >> 2) & 0xff;
            r[11 * i + 4] = (t[2] >> 10) | ((t[3] & 0x7f) << 1);
            r[11 * i + 5] = (t[3] >> 7) | ((t[4] & 0x0f) << 4);
        }
        2 => {
            r[11 * i + 0] = t[0] & 0xff;
            r[11 * i + 1] = (t[0] >> 8) | ((t[1] & 0x1f) << 3);
            r[11 * i + 2] = (t[1] >> 5) | ((t[2] & 0x03) << 6);
        }
        _ => {}
    }
}