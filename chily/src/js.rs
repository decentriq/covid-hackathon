use super::Cipher;
use super::Keypair;
use super::Nonce;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

use x25519_dalek::PublicKey;
use x25519_dalek::StaticSecret;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name= Nonce)]
pub struct JsNonce(Nonce);

impl From<Nonce> for JsNonce {
    fn from(nonce: Nonce) -> Self {
        JsNonce(nonce)
    }
}

#[wasm_bindgen(js_class = Nonce)]
impl JsNonce {
    #[wasm_bindgen(js_name = fromRandom)]
    pub fn from_random() -> JsNonce {
        Nonce::from_random().into()
    }

    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Box<[u8]> {
        let bytes = self.0.bytes;
        bytes.to_vec().into_boxed_slice()
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: Box<[u8]>) -> JsNonce {
        let nonce: Nonce = bytes.as_ref().into();
        nonce.into()
    }
}

#[wasm_bindgen(js_name= StaticSecret)]
pub struct JsStaticSecret(StaticSecret);

impl From<StaticSecret> for JsStaticSecret {
    fn from(secret: StaticSecret) -> Self {
        JsStaticSecret(secret)
    }
}

#[wasm_bindgen(js_class = StaticSecret)]
impl JsStaticSecret {
    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Box<[u8]> {
        let bytes = self.0.to_bytes();
        bytes.to_vec().into_boxed_slice()
    }
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: Box<[u8]>) -> JsStaticSecret {
        let mut buffer: [u8; 32] = [0; 32];
        buffer.copy_from_slice(&bytes[0..32]);
        let secret: StaticSecret = buffer.into();
        secret.into()
    }
}

#[wasm_bindgen(js_name= PublicKey)]
pub struct JsPublicKey(PublicKey);

impl From<PublicKey> for JsPublicKey {
    fn from(public_key: PublicKey) -> Self {
        JsPublicKey(public_key)
    }
}

#[wasm_bindgen(js_class = PublicKey)]
impl JsPublicKey {
    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Box<[u8]> {
        let bytes = self.0.as_bytes();
        bytes.to_vec().into_boxed_slice()
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: Box<[u8]>) -> JsPublicKey {
        let mut buffer: [u8; 32] = [0; 32];
        buffer.copy_from_slice(&bytes[0..32]);
        let public_key: PublicKey = buffer.into();
        public_key.into()
    }
}

#[wasm_bindgen(js_name = Keypair)]
pub struct JsKeypair(Keypair);

#[wasm_bindgen(js_class = Keypair)]
impl JsKeypair {
    #[wasm_bindgen(js_name = fromRandom)]
    pub fn from_random() -> JsKeypair {
        let key_pair = Keypair::generate();
        JsKeypair(key_pair)
    }

    #[wasm_bindgen(js_name = fromSecret)]
    pub fn from_secret(js_secret: &JsStaticSecret) -> JsKeypair {
        let public = PublicKey::from(&js_secret.0);
        // no `Copy` implemented for `StaticSecret`
        let bytes = js_secret.0.to_bytes();
        let secret: StaticSecret = bytes.into();
        JsKeypair(Keypair { secret, public })
    }

    #[wasm_bindgen(getter)]
    pub fn secret(&self) -> JsStaticSecret {
        // no `Copy` implemented for `StaticSecret`
        let bytes = self.0.secret.to_bytes();
        let secret: StaticSecret = bytes.into();
        secret.into()
    }

    #[wasm_bindgen(getter = publicKey)]
    pub fn public_key(&self) -> JsPublicKey {
        self.0.public.into()
    }
}

#[wasm_bindgen(js_name = Cipher)]
pub struct JsCipher(Cipher);

#[wasm_bindgen(js_class = Cipher)]
impl JsCipher {
    #[wasm_bindgen(constructor)]
    pub fn new(my_secret_key: JsStaticSecret, their_pub_key: JsPublicKey) -> JsCipher {
        JsCipher(Cipher::new(&my_secret_key.0, &their_pub_key.0))
    }

    #[wasm_bindgen(js_name = encrypt)]
    pub fn encrypt(&mut self, data: Box<[u8]>, nonce: &JsNonce) -> Box<[u8]> {
        self.0.encrypt(&data, &nonce.0).into_boxed_slice()
    }

    #[wasm_bindgen(js_name = decrypt)]
    pub fn decrypt(&mut self, data: Box<[u8]>, nonce: &JsNonce) -> Box<[u8]> {
        self.0.decrypt(&data, &nonce.0).into_boxed_slice()
    }
}
