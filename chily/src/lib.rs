mod utils;

// we only compile js if we target wasm
#[cfg(target_arch = "wasm32")]
pub mod js;
// we only compile python if we target python build
#[cfg(feature = "python")]
pub mod python;

pub use x25519_dalek::PublicKey;
pub use x25519_dalek::StaticSecret;

use rand::CryptoRng;
use rand::Rng;

use xsalsa20poly1305::aead::{generic_array::GenericArray, Aead, NewAead};
use xsalsa20poly1305::{Tag, XSalsa20Poly1305};
use salsa20::hsalsa20;
use std::convert::From;

pub struct Nonce {
    pub bytes: GenericArray<u8, <xsalsa20poly1305::XSalsa20Poly1305 as Aead>::NonceSize>,
}

impl From<&[u8]> for Nonce {
    fn from(rand: &[u8]) -> Self {
        let slice = &rand
            .get(0..24)
            .expect("Make sure the nonce is 192bits. We don't apply padding.");
        Self {
            bytes: GenericArray::clone_from_slice(&slice),
        }
    }
}

impl Nonce {
    pub fn from_random() -> Self {
        let nonce = utils::getrandom_192bits();
        Self {
            bytes: GenericArray::clone_from_slice(&nonce),
        }
    }
}

/// An x25519 keypair.
pub struct Keypair {
    /// The secret half of this keypair.
    pub secret: StaticSecret,
    /// The public half of this keypair.
    pub public: PublicKey,
}

impl Clone for Keypair {
    fn clone(&self) -> Keypair {
        Keypair::from_secret_key(self.secret.clone())
    }
}

impl Keypair {
    pub fn from_random<R>(rand: &mut R) -> Keypair
    where
        R: CryptoRng + Rng,
    {
        let secret = StaticSecret::new(rand);
        Self::from_secret_key(secret)
    }

    pub fn generate() -> Keypair {
        let mut rand = utils::getrandom();
        Self::from_random(&mut rand)
    }

    pub fn from_secret_key(secret: StaticSecret) -> Keypair {
        let public = PublicKey::from(&secret);
        Keypair { secret, public }
    }
}

pub struct Cipher(XSalsa20Poly1305);

impl Cipher {
    pub fn new(my_secret_key: &StaticSecret, their_pub_key: &PublicKey) -> Cipher {
        let shared_secret = my_secret_key.diffie_hellman(&their_pub_key);
        let key = hsalsa20(
            &GenericArray::clone_from_slice(shared_secret.as_bytes()),
            &GenericArray::default(),
        );
        let aead = XSalsa20Poly1305::new(key);
        Cipher(aead)
    }

    #[must_use]
    pub fn encrypt(&self, msg: &[u8], nonce: &Nonce) -> Vec<u8> {
        self.0
            .encrypt(&nonce.bytes, msg)
            .expect("Encryption failed")
    }

    #[must_use]
    pub fn decrypt(&self, msg: &[u8], nonce: &Nonce) -> Vec<u8> {
        self.0
            .decrypt(&nonce.bytes, msg)
            .expect("Decryption failed")
    }

    pub fn encrypt_in_place_detached(&self, msg: &mut [u8], aad: &[u8], nonce: &Nonce) -> Tag {
        self.0
            .encrypt_in_place_detached(&nonce.bytes, aad, msg)
            .expect("Encryption failed")
    }

    pub fn decrypt_in_place_detached(&self, msg: &mut [u8], aad: &[u8], nonce: &Nonce, tag: &Tag) {
        self.0
            .decrypt_in_place_detached(&nonce.bytes, aad, msg, tag)
            .expect("Decryption failed")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    // use ascii::AsciiStr;
    // use hex;

    #[test]
    fn test_commutative_diffie_hellman() {
        let alice = Keypair::generate();
        let bob = Keypair::generate();

        let shared_secret1 = alice.secret.diffie_hellman(&bob.public);
        let shared_secret2 = bob.secret.diffie_hellman(&alice.public);

        assert_eq!(shared_secret1.as_bytes(), shared_secret2.as_bytes())
    }

    #[test]
    fn test_cipher_identity() {
        let alice = Keypair::generate();
        let bob = Keypair::generate();

        let plaintext = b"corona sucks";
        let mut buffer = plaintext.to_vec();

        let nonce = Nonce::from_random();
        let mut cipher = Cipher::new(&alice.secret, &bob.public);

        let tag = cipher.encrypt_in_place_detached(&mut buffer, b"", &nonce);
        // let ascii_str_cipher = hex::encode(&buffer);
        // println!("Cipher: {:#?}", &ascii_str_cipher);

        cipher.decrypt_in_place_detached(&mut buffer, b"", &nonce, &tag);
        // let ascii_str_decrypted = AsciiStr::from_ascii(&decrypted).unwrap();
        // println!("Encrypted {:#?}", &ascii_str_decrypted);
        assert_eq!(plaintext.to_vec(), buffer);
    }
    #[test]
    fn test_js_integration() {
        let alice_secret: [u8; 32] = [
            56, 4, 190, 47, 42, 57, 72, 25, 194, 75, 105, 248, 146, 43, 60, 53, 55, 165, 30, 61,
            122, 57, 95, 197, 210, 123, 95, 97, 215, 28, 193, 92,
        ];

        let bob_secret: [u8; 32] = [
            64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
            243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
        ];

        let nonce: Nonce = [0u8; 24].as_ref().into();

        let alice = Keypair::from_secret_key(alice_secret.into());
        let bob = Keypair::from_secret_key(bob_secret.into());

        let mut cipher = Cipher::new(&alice.secret, &bob.public);

        let plaintext: [u8; 2] = [21, 31];
        let mut buffer = plaintext.to_vec();
        let tag = cipher.encrypt_in_place_detached(&mut buffer, b"", &nonce);
        println!("Enc: {:#?}", &buffer);
        assert_eq!([63 as u8, 106 as u8].to_vec(), buffer);
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_slice_out_of_bounds() {
        let bytes = [0 as u8; 23].as_ref();
        let nonce: Nonce = bytes.into();
    }
}
