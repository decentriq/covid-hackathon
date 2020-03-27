## 🌶 CryptoLib: chily 

### Authenticated Encryption

The plan for `chily` is to have pure Rust implementation for the following crypto protocols (following RFC 7539):

- Key exchange: X25519
- Encryption: XSalsa20 stream cipher
- Authentication: Poly1305 MAC 

In contrast to RFC 7530 we use XChaCha20 instead of ChaCha in order to have a 192 byte nonce (instead of 96bits).

### Randomness

We heavily rely on "secure" randomness in this library. Mainly for key generation in the enclave (no external static key can be provided) and nonce derivation. Depending on the target we use the following sources:

- x64: `getrandom` system call if available, otherwise `/dev/urandom`
- SGX:  Based on `rdrand` instructions of the CPU ([https://docs.rs/rdrand/0.6.0/rdrand/](https://docs.rs/rdrand/0.6.0/rdrand/))
- WASM: `Crypto.getRandomValues` exposed by the JS engine via `wasm-bindgen` bridge

## 🚴 Usage

### Rust

Just add `chily` as a dependency and see how it's being used in the following example:

```rust
// generate random keypair
let alice = Keypair::generate();

// or parse a secret from existing bytes
let bob_secret: [u8; 32] = [
    64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
    243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
];
// and then create the keypair
let bob = Keypair::from_secret_key(bob_secret.into());

// define some buffer
let plaintext = b"covid sucks";
let mut buffer = plaintext.to_vec();

// create a random Nonce
let nonce = Nonce::from_random();

// instantiate the cipher
let mut cipher = Cipher::new(&alice.secret, &bob.public);

// encrypt in place
let tag = cipher.encrypt_in_place_detached(&mut buffer, b"", &nonce);

// decrypt in place
cipher.decrypt_in_place_detached(&mut buffer, b"", &nonce, &tag);

assert_eq!(plaintext.to_vec(), buffer); // Ok!
```

### JavaScript / TypeScript

Add the package from folder `js/pkg` as dependency to the `package.json` in your project:
```json
  "dependencies": {
    "chily": "file:chily-0.2.0.tgz"
  }
```

Then the library can be used as shown below:

```ts
import * as chily from "chily";

// generate random keypair
let alice = chily.Keypair.fromRandom();

// or parse a secret from existing bytes
let bob_secret = chily.StaticSecret.fromBytes(new Uint8Array([
    64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
    243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
]));
// and then create the keypair
let bob = chily.Keypair.fromSecret(bob_secret);

// create a random Nonce
let nonce = chily.Nonce.fromRandom();

// instantiate the cipher
let cipher = chily.Cipher.new(alice.secret, bob.publicKey);

// define some buffer
var plaintext = new Uint8Array([21,31]);

// encrypt
let encrypted = cipher.encrypt(plaintext, nonce);

// decrypt
let decrypted = cipher.decrypt(encrypted, nonce);

expect(plaintext).to.eql(decrypted); // Ok!
```

### Python

Install the wheel from folder `py/pkg` by running `pip3 install chily.whl`
Then the library can be used as shown below:

```python
import chily

# generate random keypair
alice = chily.Keypair.from_random()

# or parse a secret from existing bytes
bob_secret = chily.StaticSecret.from_bytes([
    64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
    243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
])
# and then create the keypair
bob = chily.Keypair.from_secret(bob_secret)

# create a random Nonce
nonce = chily.Nonce.from_random()

# instantiate the cipher
cipher = chily.Cipher(alice.secret, bob.publicKey, nonce)

# define some buffer
plaintext = [21,31]

# encrypt 
enc = cipher.encrypt(plaintext, nonce)

# decrypt
dec = cipher.decrypt(enc, nonce)

assert plaintext == dec
```

## 🛠️ Test
We have four different test stages.

### Rust
Regular tests written in Rust. Just call `cargo test`.

### WASM
Some test can be specified to run in the node wasm interpreter. They are defined using the `[wasm_bindgen_test]` attribute.  In order to run them go execute the following command in the `js` folder:
```bash
npm run wasm-test
```

### JavaScript / TypeScript 
There also are some tests for the JavaScript bindings using `mocha` and `chai`.  
They are defined in the folder `js/tests` and can be run  using the following command:
```bash
npm run test
```

### Python
There also are some tests for the Python bindings using `tox`.  
They are defined in the folder `py/tests` and can be run  using the following command:
```bash
tox
```

## 🎁 Build & Package 

### JavaScript / TypeScript
To build the wasm code and the js/ts binding run in the `js` folder:
* `npm run build-node` for nodejs
* `npm run build-bundler` for browser / webpack
* `npm run build` for one compatible with both

Then package the dependency by running `npm pack` in the corresponding `pkg` dir.

### Python
To build the python bindings you'll need `maturin`. Run in the `py` folder:
* `pip3 install maturin` to install maturin
* `./build.sh` to build the wheel for the current platform in the pkg folder

## 🔋 ToDos

* Add X.509 cert support
* Improve error handling