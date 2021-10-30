import * as wasm from "crypto_test";
import {runTests} from "./test_run";
import {runBench} from "./bench_run";

const DEFAULT_RANDOM_BYTES = 128;

let DO_COMPARISON = 0;

function hexHashDisplayBuilder(hash_array) {
    return Array.prototype.map.call(hash_array, x => ('00' + x.toString()).slice(-2)).join('');
}

function displayRunTimeStats(times) {
    let sum = 0;
    let max = null;
    let min = null;
    for (let i = 0; i < times.length; i++) {
        if (times[i] > max || max === null) {
            max = times[i];
        }
        if (times[i] < min || min === null) {
            min = times[i];
        }
        sum += times[i];
    }
    console.log("Total time of " + times.length + " runs is " + sum + "ms. Average per run: " + (sum / times.length) + "ms. Max: " + max + "ms Min: " + min + "ms.");
}

document.addEventListener('click', function (event) {
    // If the clicked element doesn't have the right selector, bail
    let allowed_events = [
        '#random-bytes',
        '#hash-start',
        '#activate-comparison',
        '#toggle-test-functions',
        '#encryption-start',
        '#run-tests',
        '#run-bench',
    ];
    let matches        = false;
    allowed_events.forEach(item => {
        if (event.target.matches(item)) {
            matches = true;
        }
    })
    if (!matches) {
        return;
    }

    switch (event.target.id) {
        case 'random-bytes':
            let num_bytes = BigInt(document.getElementById('num-bytes').value);

            if (num_bytes === 0n || num_bytes === undefined || num_bytes === null) {
                num_bytes = BigInt(DEFAULT_RANDOM_BYTES);
            }
            alert(wasm.get_random_bytes(num_bytes));
            break;
        case 'hash-start':
            let input                 = document.getElementById('hash-input').value;
            let reference             = document.getElementById('hash-reference');
            let hash_algorithm_radios = document.getElementsByName('hash_algorithm');
            let hash                  = "";
            for (let i = 0, length = hash_algorithm_radios.length; i < length; i++) {
                if (hash_algorithm_radios[i].checked) {
                    hash = wasm.tiny_keccak(input, hash_algorithm_radios[i].value);
                }
            }
            let hexHash         = hexHashDisplayBuilder(hash);
            reference.innerHTML = hexHash;
            if (DO_COMPARISON) {
                let comparison_textarea = document.getElementById('hash-comparison');
                if (hexHash === comparison_textarea.value) {
                    alert("Result and Comparison are equal.");
                }
            }
            break;
        case 'activate-comparison':
            let comparison_textarea           = document.getElementById('hash-comparison');
            let comparison_checkbox           = document.getElementById('activate-comparison');
            comparison_textarea.style.display = comparison_checkbox.checked ? "block" : "none";
            DO_COMPARISON                     = comparison_checkbox.checked ? 1 : 0;
            break;
        case 'toggle-test-functions':
            let test_function_row           = document.getElementById('test-functions');
            let display_class               = (test_function_row.style.display === "none" || test_function_row.style.display === "") ? "block" : "none";
            test_function_row.style.display = display_class;
            break;
        case 'encryption-start':
            wasm.crypto_kem_keypair();
            break;
        case 'run-tests':
            runTests();
            break;
        case 'run-bench':
            let runTimes = runBench(1000);
            displayRunTimeStats(runTimes);
            break;
    }
}, false);
