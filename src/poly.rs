use crate::params::{NTRU_N, NTRU_Q};
use crate::poly_r2_inv::poly_r2_inv;
use crate::poly_rq_mul::poly_rq_mul;

pub struct Poly {
    pub coeffs: [u16; NTRU_N],
}

impl Poly {
    pub fn new() -> Poly {
        Poly {
            coeffs: [0; NTRU_N],
        }
    }
    pub fn build(value: u16) -> Poly {
        Poly {
            coeffs: [value; NTRU_N],
        }
    }
}

pub const MODQ: fn(u16) -> u16 = |x| {
    x & (NTRU_Q - 1) as u16
};

pub fn poly_z3_to_zq(r: &mut Poly) {
    for i in 0..NTRU_N {
        r.coeffs[i] = r.coeffs[i] | (0 - (r.coeffs[i] >> 1)) & (NTRU_Q - 1) as u16;
    }
}

pub fn poly_r2_inv_to_rq_inv(r: &mut Poly, ai: Poly, a: &Poly) {
    // TODO: change to compiler error macro
    if NTRU_Q <= 256 || NTRU_Q >= 65536 {
        panic!("poly_R2_inv_to_Rq_inv in poly.c assumes 256 < q < 65536");
    }
    let mut b = Poly::new();
    let mut c = Poly::new();
    let mut s = Poly::new();

    // for 0..4
    //    ai = ai * (2 - a*ai)  mod q
    for i in 0..NTRU_N {
        b.coeffs[i] = 0 - a.coeffs[i];
    }
    for i in 0..NTRU_N {
        r.coeffs[i] = ai.coeffs[i];
    }
    poly_rq_mul(&mut c, r, &b);
    c.coeffs[0] += 2; // c = 2 - a*ai
    poly_rq_mul(&mut s, &c, r); // s = ai*c

    poly_rq_mul(&mut c, &s, &b);
    c.coeffs[0] += 2; // c = 2 - a*s
    poly_rq_mul(r, &c, &s);

    poly_rq_mul(&mut c, r, &b);
    c.coeffs[0] += 2; // c = 2 - a*r
    poly_rq_mul(&mut s, &c, r); // s = r*c

    poly_rq_mul(&mut c, &s, &b);
    c.coeffs[0] += 2; // c = 2 - a*s
    poly_rq_mul(r, &c, &s); // r = s*c
}

pub fn poly_rq_inv(r: &mut Poly, a: &Poly) {
    let mut ai2: Poly = Poly::new();
    poly_r2_inv(&mut ai2, a);
    poly_r2_inv_to_rq_inv(r, ai2, a);
}