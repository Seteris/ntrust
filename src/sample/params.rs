// defines:
pub const NTRU_N: usize = 509;
pub const NTRU_SAMPLE_IID_BYTES: usize = NTRU_N - 1;
pub const NTRU_SAMPLE_FT_BYTES: usize = (30 * (NTRU_N - 1) + 7) / 8;
pub const NTRU_SAMPLE_FG_BYTES: usize = NTRU_SAMPLE_IID_BYTES + NTRU_SAMPLE_FT_BYTES;

pub const NTRU_LOGQ: usize = 11;
pub const NTRU_Q: usize = 1 << NTRU_LOGQ;
pub const NTRU_WEIGHT: usize = NTRU_Q / 8 - 2;

pub const NTRU_HPS: bool = false;
pub const NTRU_HRSS: bool = true;