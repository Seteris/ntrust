use crate::params::NTRU_N;
use crate::poly::Poly;

pub fn mod3(a: u16) -> u16 {
    let mut r: u16;
    let t: i16;
    let c: i16;

    r = (a >> 8) + (a & 0xff); // r mod 255 == a mod 255
    r = (r >> 4) + (r & 0xf); // r' mod 15 == r mod 15
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3
    r = (r >> 2) + (r & 0x3); // r' mod 3 == r mod 3

    t = (r - 3) as i16;
    c = t >> 15;

    ((c as u16) & r) as u16 ^ (!c & t) as u16
}

pub fn poly_mod_q_phi_n(r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = r.coeffs[i] - r.coeffs[NTRU_N - 1];
    }
}

pub fn poly_mod_3_phi_n(mut r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = mod3(r.coeffs[i] + 2 * r.coeffs[NTRU_N - 1]);
    }
}