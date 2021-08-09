//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate crypto_test;
extern crate hex;
extern crate wasm_bindgen_test;

use std::fs;

use hex::FromHexError;
use wasm_bindgen_test::*;

pub mod provider;
pub mod params;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_owcpa_keypair() {
    let mut keypair_test_data = provider::OwcpaKeypairTestData::new();
    let mut keypair_comparison_data = provider::OwcpaKeypairTestData::new();
    keypair_test_data.provide_test_data(1);
    // keypair_comparison_data.provide_comparison_data();
    // for i in 0..TEST_DATA_CHUNK_SIZE {
    //     crypto_test::owcpa::owcpa_keypair(&mut keypair_test_data.test_data[i].pk, &mut keypair_test_data.test_data[i].sk, keypair_test_data.test_data[i].seed);
    //     assert_eq!(keypair_test_data.test_data[i].seed, keypair_comparison_data.test_data[i].seed);
    // }
}

