import * as wasm from "crypto_test";

document.addEventListener('click', function (event) {

    // If the clicked element doesn't have the right selector, bail
    let allowed_events = [
        '#random-bytes',
        "#biggest-signed",
        "#biggest-unsigned"
    ];
    let matches = false;
    allowed_events.forEach(item => {
        if (event.target.matches(item)) {
            matches = true;
        }
    })
    if (!matches) {
        return;
    }

    event.preventDefault();
    // console.log(event.target.id);
    switch(event.target.id) {
        case 'random-bytes':
            let num_bytes = BigInt(document.getElementById('num-bytes').value);

            if(num_bytes === 0n || num_bytes === undefined || num_bytes === null) {
                num_bytes = BigInt("128");
            }
            console.log(num_bytes);
            alert(wasm.get_random_bytes(num_bytes));
            break;
        case 'biggest-signed':
            alert(wasm.biggest_signed());
            break;
        case 'biggest-unsigned':
            alert(wasm.biggest_unsigned());
            break;
    }


}, false);
