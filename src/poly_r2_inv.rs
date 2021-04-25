use crate::params::NTRU_N;
use crate::Poly;

fn both_negative_mask(x: i16, y: i16) -> i16 {
    (x & y) >> 15
}

pub fn poly_r2_inv(r: &mut Poly, a: &mut Poly) {
    let mut v = Poly::construct();
    let mut w = Poly::construct();
    let mut f = Poly::build(1);
    let mut g = Poly::construct();

    let mut delta: i16 = 1;
    let sign: i16;
    let swap: i16;
    let mut t: i16;

    for i in 0..NTRU_N - 1 {
        g.coeffs[NTRU_N - 2 - i] = (a.coeffs[i] ^ a.coeffs[NTRU_N - 1]) & 1;
    }
    g.coeffs[NTRU_N - 1] = 0;

    for _ in 0..2 * (NTRU_N - 1) - 1 {
        for i in (NTRU_N - 1..0).step_by(-1) {
            v.coeffs[i] = v.coeffs[i - 1];
        }
        v.coeffs[0] = 0;
        sign = (g.coeffs[0] & f.coeffs[0]) as i16;
        swap = both_negative_mask(0 - delta, (0 - g.coeffs[0]) as i16);
        delta ^= swap & (delta ^ (0 - delta));
        delta += 1;

        for i in 0..NTRU_N {
            t = swap & (f.coeffs[i] ^ g.coeffs[i]);
            f.coeffs[i] ^= t;
            g.coeffs[i] ^= t;
            t = swap & (v.coeffs[i] ^ w.coeffs[i]);
            v.coeffs[i] ^= t;
            w.coeffs[i] ^= t;
        }
        for i in 0..NTRU_N {
            g.coeffs[i] = g.coeffs[i] ^ (sign & f.coeffs[i]);
        }
        for i in 0..NTRU_N {
            w.coeffs[i] = w.coeffs[i] ^ (sign & v.coeffs[i]);
        }
        for i in NTRU_N - 1 {
            g.coeffs[i] = g.coeffs[i + 1];
        }
        g.coeffs[NTRU_N - 1] = 0;
    }
    for i in 0..NTRU_N - 1 {
        r.coeffs[i] = v.coeffs[NTRU_N - 2 - i];
    }
    r.coeffs[NTRU_N - 1] = 0;
}