use crate::params::{NTRU_N, NTRU_OWCPA_MSGBYTES, NTRU_PACK_DEG};
use crate::poly::Poly;
use crate::poly_mod::poly_mod_3_phi_n;

pub fn poly_s3_tobytes(msg: &mut [u8; NTRU_OWCPA_MSGBYTES], a: &Poly) {
    let mut c: u8;
    for i in 0..NTRU_PACK_DEG / 5 {
        c = (a.coeffs[5 * i + 4] & 255) as u8;
        c = ((3 * c as u16 + a.coeffs[5 * i + 3]) & 255) as u8;
        c = ((3 * c as u16 + a.coeffs[5 * i + 2]) & 255) as u8;
        c = ((3 * c as u16 + a.coeffs[5 * i + 1]) & 255) as u8;
        c = ((3 * c as u16 + a.coeffs[5 * i + 0]) & 255) as u8;
        msg[i] = c;
    }

    if NTRU_PACK_DEG > (NTRU_PACK_DEG / 5) * 5 { // if 5 does not divide NTRU_N - 1
        let i = NTRU_PACK_DEG / 5;
        c = 0;
        let mut j: isize = NTRU_PACK_DEG as isize - (5 * i) as isize - 1;
        while j >= 0 {
            c = ((3 * c as u16 + a.coeffs[5 * i + j as usize]) & 255) as u8;
            j -= 1;
        }
        msg[i] = c;
    }
}

#[allow(arithmetic_overflow)]
pub fn poly_s3_frombytes(mut r: &mut Poly, msg: [u8; NTRU_OWCPA_MSGBYTES]) {
    let mut c: u8;
    for i in 0..NTRU_PACK_DEG / 5 {
        c = msg[i];
        r.coeffs[5 * i + 0] = c as u16;
        r.coeffs[5 * i + 1] = (c * 171 >> 9) as u16;  // this is division by 3
        r.coeffs[5 * i + 2] = (c * 57 >> 9) as u16;  // division by 3^2
        r.coeffs[5 * i + 3] = (c * 19 >> 9) as u16;  // division by 3^3
        r.coeffs[5 * i + 4] = (c * 203 >> 14) as u16;  // etc.
    }
    if NTRU_PACK_DEG > (NTRU_PACK_DEG / 5) * 5 {
        let i = NTRU_PACK_DEG / 5;
        c = msg[i];
        let mut j = 0;
        while 5 * i + j < NTRU_PACK_DEG {
            r.coeffs[5 * i + j] = c as u16;
            c = c * 171 >> 9;
            j += 1;
        }
    }
    r.coeffs[NTRU_N - 1] = 0;
    poly_mod_3_phi_n(r);
}