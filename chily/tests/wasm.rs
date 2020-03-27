#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use chily::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn js_integration_test_wasm() {
    let alice_secret: [u8; 32] = [
        56, 4, 190, 47, 42, 57, 72, 25, 194, 75, 105, 248, 146, 43, 60, 53, 55, 165, 30, 61, 122,
        57, 95, 197, 210, 123, 95, 97, 215, 28, 193, 92,
    ];

    let bob_secret: [u8; 32] = [
        64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129, 243,
        144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
    ];

    let alice = Keypair::from_secret_key(alice_secret.into());
    let bob = Keypair::from_secret_key(bob_secret.into());

    let mut cipher = Cipher::new_unseeded(&alice.secret, &bob.public);

    let plaintext: [u8; 2] = [21, 31];
    let mut buffer = plaintext.to_vec();
    cipher.encrypt_in_place(&mut buffer);
    println!("Enc: {:#?}", &buffer);
    assert_eq!([68 as u8, 150 as u8].to_vec(), buffer);
}
