use crate::params;
use crate::Poly;
use crate::poly_mod;

use params::NTRU_OWCPA_MSGBYTES as NTRU_OWCPA_MSGBYTES;
use params::NTRU_PACK_DEG as NTRU_PACK_DEG;
use params::NTRU_N as NTRU_N;
use poly_mod::poly_mod_3_phi_n as poly_mod_3_phi_n;

pub fn poly_s3_tobytes(mut msg: [u8; NTRU_OWCPA_MSGBYTES], mut a: &Poly) {
    let c: u8;
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
        let mut j = NTRU_PACK_DEG - (5 * i) - 1;
        while j >= 0 {
            c = ((3 * c as u16 + a.coeffs[5 * i + j]) & 255) as u8;
            j -= 1;
        }
        msg[i] = c;
    }
}

pub fn poly_s3_frombytes(mut r: &mut Poly, msg: [u8; NTRU_OWCPA_MSGBYTES]) {

    let c: u8;
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
            r.coeffs[5*i+j] = c as u16;
            c = c * 171 >> 9;
            j += 1;
        }
    }
    r.coeffs[NTRU_N- 1] = 0;
    poly_mod_3_phi_n(r);
}