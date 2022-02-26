use hex;

use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{error, fs};

use ntrust::api::{
    CRYPTO_BYTES, CRYPTO_CIPHERTEXTBYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES,
};
use ntrust::kem::{crypto_kem_dec, crypto_kem_enc, crypto_kem_keypair};
use ntrust::params::NTRU_N;
use ntrust::rng::{randombytes, randombytes_init, Aes256CtrDrbgStruct};

const CRYPTO_ALGNAME: &str = if NTRU_N == 509 {
    "ntruhps2048509"
} else if NTRU_N == 677 {
    "ntruhps2048677"
} else if NTRU_N == 821 {
    "ntruhps4096821"
} else
/*if NTRU_N == 701*/
{
    "ntruhrss"
};

type R = Result<(), Box<dyn error::Error>>;

#[derive(Debug)]
struct Testcase {
    count: usize,
    seed: [u8; 48],
    pk: [u8; CRYPTO_PUBLICKEYBYTES],
    sk: [u8; CRYPTO_SECRETKEYBYTES],
    ct: [u8; CRYPTO_CIPHERTEXTBYTES],
    ss: [u8; CRYPTO_BYTES],
}

fn is_zero(x: &[u8]) -> bool {
    if x.is_empty() {
        true
    } else {
        x[0] == 0 && is_zero(&x[1..])
    }
}

impl Testcase {
    fn new() -> Testcase {
        Testcase {
            count: 0,
            seed: [0u8; 48],
            pk: [0u8; CRYPTO_PUBLICKEYBYTES],
            sk: [0u8; CRYPTO_SECRETKEYBYTES],
            ct: [0u8; CRYPTO_CIPHERTEXTBYTES],
            ss: [0u8; CRYPTO_BYTES],
        }
    }

    fn write_to_file(&self, fd: &mut fs::File) -> R {
        let repr_bytes = |bytes: &[u8]| -> String {
            if is_zero(&bytes) {
                "".to_string()
            } else {
                hex::encode(bytes)
            }
        };

        writeln!(fd, "count = {}", self.count)?;
        writeln!(fd, "seed = {}", hex::encode(self.seed))?;
        writeln!(fd, "pk = {}", repr_bytes(&self.pk).as_str())?;
        writeln!(fd, "sk = {}", repr_bytes(&self.sk).as_str())?;
        writeln!(fd, "ct = {}", repr_bytes(&self.ct).as_str())?;
        writeln!(fd, "ss = {}\n", repr_bytes(&self.ss).as_str())?;

        Ok(())
    }

    fn read_line(&mut self, line: &str) -> Result<bool, Box<dyn error::Error>> {
        if line.starts_with('#') {
            return Ok(true);
        }

        let mut fields = line.split("=");
        let name = match fields.nth(0) {
            Some(n) => n.trim(),
            None => return Ok(false),
        };
        let value = match fields.nth(0) {
            Some(v) => v.trim(),
            None => return Ok(false),
        };

        match name {
            "count" => self.count = value.parse::<usize>()?,
            "seed" => hex::decode_to_slice(value, &mut self.seed as &mut [u8])?,
            "pk" => hex::decode_to_slice(value, &mut self.pk as &mut [u8])?,
            "sk" => hex::decode_to_slice(value, &mut self.sk as &mut [u8])?,
            "ct" => hex::decode_to_slice(value, &mut self.ct as &mut [u8])?,
            "ss" => hex::decode_to_slice(value, &mut self.ss as &mut [u8])?,
            _ => panic!("invalid name '{}'", name),
        };

        Ok(true)
    }

    fn read_from_file(&mut self, reader: &mut BufReader<fs::File>) -> R {
        for line in reader.lines() {
            if !self.read_line(&line?)? {
                return Ok(());
            }
        }

        Ok(())
    }
}

fn create_request_file(filepath: &str, rng: &mut Aes256CtrDrbgStruct) -> R {
    let mut fd = fs::File::create(filepath)?;

    // initialize RNG
    let mut entropy_input = [0u8; 48];
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }
    randombytes_init(entropy_input, rng);

    // create 100 testcase seeds
    for _ in 0..100 {
        let mut tc = Testcase::new();
        randombytes(&mut tc.seed, rng);

        tc.write_to_file(&mut fd)?;
    }

    Ok(())
}

fn create_response_file(filepath: &str, rng: &mut Aes256CtrDrbgStruct) -> R {
    let mut fd = fs::File::create(filepath)?;
    writeln!(&mut fd, "# {}\n", CRYPTO_ALGNAME)?;

    // initialize RNG
    let mut entropy_input = [0u8; 48];
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }
    randombytes_init(entropy_input, rng);

    // create 100 testcase seeds
    for _ in 0..100 {
        let mut tc = Testcase::new();
        randombytes(&mut tc.seed, rng);

        crypto_kem_keypair(&mut tc.pk, &mut tc.sk, rng);
        crypto_kem_enc(&mut tc.ct, &mut tc.ss, &tc.pk, rng);
        //let fail = crypto_kem_dec(&mut tc.ss, &tc.ct, &tc.sk);

        tc.write_to_file(&mut fd)?;
    }

    Ok(())
}

fn verify(filepath: &str) -> R {
    let fd = fs::File::open(filepath)?;
    let mut reader = BufReader::new(fd);
    let mut rng = Aes256CtrDrbgStruct::new();

    // create 100 testcase seeds
    for _ in 0..100 {
        let mut expected = Testcase::new();
        expected.read_from_file(&mut reader)?;
        randombytes_init(expected.seed, &mut rng);

        let mut actual = Testcase::new();
        crypto_kem_keypair(&mut actual.pk, &mut actual.sk, &mut rng);
        crypto_kem_enc(&mut actual.ct, &mut actual.ss, &actual.pk, &mut rng);
        let fail = crypto_kem_dec(&mut actual.ss, &actual.ct, &actual.sk);

        assert!(fail == 0);
        assert_eq!(expected.seed, actual.seed);
        assert_eq!(expected.pk, actual.pk);
        assert_eq!(expected.sk, actual.sk);
        assert_eq!(expected.ct, actual.ct);
        assert_eq!(expected.ss, actual.ss);
    }

    Ok(())
}

fn main() -> R {
    let req_file = format!("PQCkemKAT_{}.req", CRYPTO_SECRETKEYBYTES);
    create_request_file(&req_file, &mut Aes256CtrDrbgStruct::new())?;

    let rsp_file = format!("PQCkemKAT_{}.rsp", CRYPTO_SECRETKEYBYTES);
    create_response_file(&rsp_file, &mut Aes256CtrDrbgStruct::new())?;

    verify(&rsp_file)?;

    Ok(())
}
