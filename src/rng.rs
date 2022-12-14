use ctr::cipher::{NewCipher, StreamCipher};

const RNG_SUCCESS: i32 = 0;

pub struct Aes256CtrDrbgStruct {
    pub key: [u8; 32],
    pub v: [u8; 16],
    pub reseed_counter: i32,
}

impl Aes256CtrDrbgStruct {
    pub fn new() -> Aes256CtrDrbgStruct {
        Aes256CtrDrbgStruct {
            key: [0; 32],
            v: [0; 16],
            reseed_counter: 0,
        }
    }
}

//    key - 256-bit AES key
//    ctr - a 128-bit plaintext value
//    buffer - a 128-bit ciphertext value
fn aes256_ecb(key: &mut [u8; 32], ctr: &mut [u8; 16], buffer: &mut [u8; 16]) {
    type Aes256Ctr = ctr::Ctr128BE<aes::Aes256>;
    let mut new_key = [0u8; 32];
    new_key.copy_from_slice(key);
    let new_nonce = &[0; 16];
    let mut cipher = Aes256Ctr::new(&new_key.into(), new_nonce.into());
    buffer.copy_from_slice(&ctr[..]);
    cipher.apply_keystream(&mut buffer[..]);
}

pub fn randombytes(x: &mut [u8], drbg_ctx: &mut Aes256CtrDrbgStruct) -> i32 {
    let mut xlen = x.len() as u64;
    let mut block = [0u8; 16];
    let mut i = 0;
    while xlen > 0 {
        let mut j: isize = 15;
        while j >= 0 {
            if drbg_ctx.v[j as usize] == 0xff {
                drbg_ctx.v[j as usize] = 0x00;
            } else {
                drbg_ctx.v[j as usize] += 1;
                break;
            }
            j -= 1;
        }
        aes256_ecb(&mut drbg_ctx.key, &mut drbg_ctx.v, &mut block);
        if xlen > 15 {
            x[i..i + 16].copy_from_slice(&block[..16]);
            i += 16;
            xlen -= 16;
        } else {
            x[xlen as usize..(xlen + xlen) as usize].copy_from_slice(&block[..xlen as usize]);
            xlen = 0;
        }
    }
    let provided_data: &mut Option<[u8; 48]> = &mut None;
    aes256_ctr_drbg_update(provided_data, &mut drbg_ctx.key, &mut drbg_ctx.v);
    drbg_ctx.reseed_counter += 1;
    RNG_SUCCESS
}

pub fn randombytes_init(entropy_input: [u8; 48], drbg_ctx: &mut Aes256CtrDrbgStruct) {
    drbg_ctx.key = [0u8; 32];
    drbg_ctx.v = [0u8; 16];

    aes256_ctr_drbg_update(&mut Some(entropy_input), &mut drbg_ctx.key, &mut drbg_ctx.v);
    drbg_ctx.reseed_counter = 1;
}

fn aes256_ctr_drbg_update(
    provided_data: &mut Option<[u8; 48]>,
    key: &mut [u8; 32],
    v: &mut [u8; 16],
) {
    let mut temp = [0u8; 48];
    let mut buffer = [0u8; 16];

    for i in 0..3 {
        for j in (0..16).rev() {
            if v[j] == 0xff {
                v[j] = 0x00;
            } else {
                v[j] += 1;
                break;
            }
        }
        buffer.copy_from_slice(&temp[16 * i..16 * i + 16]);
    }

    aes256_ecb(key, v, &mut buffer);
    if provided_data.is_some() {
        for (i, val) in temp.iter_mut().enumerate() {
            *val ^= provided_data.unwrap()[i];
        }
    }
    key[..32].copy_from_slice(&temp[..32]);
    v[..16].copy_from_slice(&temp[32..]);
}
