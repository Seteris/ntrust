import * as wasm from "crypto_test";
import {provideData} from "./test_data";

let passed_test_count = 0;
let failed_test_count = 0;
let failed_tests      = [];

export function runTests() {
    let pass          = false;
    passed_test_count = 0;
    failed_test_count = 0;
    failed_tests      = [];
    for (let i = 1; i <= 10; i++) {
        let chunk = provideData(i);
        for (let j = 0; j < chunk.length; j++) {
            pass = runTest(chunk[j][2], chunk[j][3], chunk[j][1]);
            if (pass === false) {
                failed_tests.push(chunk[j][0]);
            }
        }
    }
    alert((passed_test_count + failed_test_count) + " tests run. " + passed_test_count + " passed. " + failed_test_count + " failed.");
    console.log("List of failed tests: " + failed_tests);
}

function runTest(comparison_pk, comparison_sk, seed) {
    let pk = new Uint8Array(699);
    let sk = new Uint8Array(935);
    try {
        let test_result = wasm.crypto_kem_keypair_test(pk, sk, seed, new Uint8Array(comparison_pk), new Uint8Array(comparison_sk), new Uint8Array(seed));
        console.log(test_result);
        passed_test_count++;
        return true;
    } catch (e) {
        failed_test_count++;
        return false;
    }
}