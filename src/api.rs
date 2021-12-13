#[cfg(feature = "ntruhps2048509")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 699;
#[cfg(feature = "ntruhps2048509")]
pub const CRYPTO_SECRETKEYBYTES: usize = 935;
#[cfg(feature = "ntruhps2048509")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 699;

#[cfg(feature = "ntruhps2048677")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 930;
#[cfg(feature = "ntruhps2048677")]
pub const CRYPTO_SECRETKEYBYTES: usize = 1234;
#[cfg(feature = "ntruhps2048677")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 930;

#[cfg(feature = "ntruhps4096821")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 1230;
#[cfg(feature = "ntruhps4096821")]
pub const CRYPTO_SECRETKEYBYTES: usize = 1590;
#[cfg(feature = "ntruhps4096821")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 1230;

#[cfg(feature = "ntruhrss701")]
pub const CRYPTO_PUBLICKEYBYTES: usize = 1138;
#[cfg(feature = "ntruhrss701")]
pub const CRYPTO_SECRETKEYBYTES: usize = 1450;
#[cfg(feature = "ntruhrss701")]
pub const CRYPTO_CIPHERTEXTBYTES: usize = 1138;

pub const CRYPTO_BYTES: usize = 32;
