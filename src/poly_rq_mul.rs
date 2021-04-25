use crate::params::NTRU_N;
use crate::Poly;

pub fn poly_rq_mul(r: &mut Poly, a: &Poly, b: &Poly) {
    for k in 0..NTRU_N {
        r.coeffs[k] = 0;
        for i in 1..NTRU_N - k {
            r.coeffs[k] += a.coeffs[k + i] * b.coeffs[NTRU_N - i];
        }
        for i in 0..(k + 1) {
            r.coeffs[k] += a.coeffs[k - i] * b.coeffs[i];
        }
    }
}