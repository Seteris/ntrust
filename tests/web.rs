//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

pub mod provider;

use wasm_bindgen_test::*;

use ntrust::owcpa;
use ntrust::params;
use ntrust::params::NTRU_SAMPLE_FG_BYTES;
use provider::{TEST_DATA_CHUNK_COUNT, TEST_DATA_CHUNK_SIZE};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_owcpa_keypair() {
    let mut keypair_test_data = provider::OwcpaKeypairTestData::new();
    let mut keypair_comparison_data = provider::OwcpaKeypairTestData::new();

    for chunk in 1..TEST_DATA_CHUNK_COUNT + 1 {
        keypair_test_data.provide_test_data(chunk as i32);
        keypair_comparison_data.provide_comparison_data(chunk as i32);
        for i in 0..TEST_DATA_CHUNK_SIZE {
            console_log!(
                "Running Test with params:\n\
            pk: {:x?}\nLength: {}\n\
            sk: {:x?}\nLength: {}\n\
            seed:{:x?}\nLength: {}\n",
                keypair_test_data.test_data[i].pk,
                keypair_test_data.test_data[i].pk.len(),
                keypair_test_data.test_data[i].sk,
                keypair_test_data.test_data[i].sk.len(),
                keypair_test_data.test_data[i].seed,
                keypair_test_data.test_data[i].seed.len()
            );
            console_log!(
                "Expecting to get:\n\
            pk: {:x?}\nLength: {}\n\
            sk: {:x?}\nLength: {}\n\
            seed:{:x?}\nLength: {}\n",
                keypair_comparison_data.test_data[i].pk,
                keypair_comparison_data.test_data[i].pk.len(),
                keypair_comparison_data.test_data[i].sk,
                keypair_comparison_data.test_data[i].sk.len(),
                keypair_comparison_data.test_data[i].seed,
                keypair_comparison_data.test_data[i].seed.len()
            );
            ntrust::owcpa::owcpa_keypair(
                &mut keypair_test_data.test_data[i].pk,
                &mut keypair_test_data.test_data[i].sk,
                keypair_test_data.test_data[i].seed,
            );
            assert_eq!(
                keypair_test_data.test_data[i].seed, keypair_comparison_data.test_data[i].seed,
                "\nSeed differs in test {}",
                keypair_test_data.test_data[i].count
            );
            assert_eq!(
                keypair_test_data.test_data[i].pk, keypair_comparison_data.test_data[i].pk,
                "\nPK differs in test {}",
                keypair_test_data.test_data[i].count
            );
            assert_eq!(
                keypair_test_data.test_data[i].sk, keypair_comparison_data.test_data[i].sk,
                "\nSK differs in test {}",
                keypair_test_data.test_data[i].count
            );
        }
    }
}
