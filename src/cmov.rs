use crate::api::CRYPTO_BYTES;
use crate::params::NTRU_OWCPA_MSGBYTES;

/* b = 1 means mov, b = 0 means don't mov*/
pub fn cmov(
    r: &mut [u8; CRYPTO_BYTES],
    x: &[u8; NTRU_OWCPA_MSGBYTES],
    len: isize,
    b: u8
) {
    let b_temp = (!b + 1);

    for i in 0..len {
        r[i] ^= b_temp & (x[i] ^ r[i]);
    }
}