use aes::{Aes256, NewBlockCipher, BlockEncrypt};
use aes::cipher::Block;
use ctr::cipher::{NewCipher, StreamCipher, StreamCipherSeek};

const RNG_SUCCESS: i32 = 0;
const RNG_BAD_MAXLEN: i32 = -1;
const RNG_BAD_OUTBUF: i32 = -2;
const RNG_BAD_REQ_LEN: i32 = -3;

pub struct AesXofStruct {
    pub buffer: [u8; 16],
    pub buffer_pos: i32,
    pub length_remaining: u32,
    pub key: [u8; 32],
    pub ctr: [u8; 16],
}

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
            reseed_counter: 0
        }
    }
}

//    key - 256-bit AES key
//    ctr - a 128-bit plaintext value
//    buffer - a 128-bit ciphertext value
fn aes256_ecb(
    key: &mut [u8; 32],
    ctr: &mut [u8; 16],
    mut buffer: &mut [u8; 16]
) {
    // TODO: CTR MODE, Generic Array
    // TODO: IV is NULL
    // type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;
    // let mut cipher = Aes128Ctr::new(key.into(), [].into());
    // cipher.apply_keystream(&mut buffer);
}


pub fn randombytes(x: &mut [u8], xlen: &mut u64, drbg_ctx: &mut Aes256CtrDrbgStruct) -> i32 {
    let mut block: [u8; 16] = [0; 16];
    let mut i = 0;

    while *xlen > 0 {
        let mut j = 15;
        while j >= 0 {
            if drbg_ctx.v[j] == 0xff {
                drbg_ctx.v[j] = 0x00;
            }
            else {
                drbg_ctx.v[j] += 1;
                break;
            }
            j-= 1;
        }
        aes256_ecb(&mut drbg_ctx.key, &mut drbg_ctx.v, &mut block);
    }
    if *xlen > 15 {
        x[i..].copy_from_slice(&block[..16]);
        i += 16;
        *xlen -= 16;
    } else {
        x[*xlen as usize..].copy_from_slice(&block[..*xlen as usize]);
        *xlen = 0;
    }
    let provided_data: &mut Option<[u8; 48]> = &mut None;
    aes256_ctr_drbg_update(provided_data, &mut drbg_ctx.key, &mut drbg_ctx.v);
    drbg_ctx.reseed_counter += 1;
    RNG_SUCCESS
}

fn aes256_ctr_drbg_update(
    provided_data: &mut Option<[u8; 48]>,
    key: &mut [u8; 32],
    v: &mut [u8; 16]
) {
    let mut temp: [u8; 48] = [0; 48];
    let mut buffer: [u8; 16] = [0; 16];

    for i in 0..3 {
        let mut j = 15;
        while j >= 0 {
            if v[j] == 0xff {
                v[j] = 0x00;
            }
            else {
                v[j] += 1;
                break;
            }
            j-= 1;
        }
        buffer.copy_from_slice(&temp[16*i..16*i + 16]);
    }

    aes256_ecb(key, v, &mut buffer);
    if provided_data.is_some() {
        for i in 0..48 {
            temp[i] ^= (provided_data.unwrap())[i];
        }
    }
    key[..32].copy_from_slice(&temp[..32]);
    v[32..].copy_from_slice(&temp[32..]);
}
