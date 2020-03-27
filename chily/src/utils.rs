use rand::RngCore;

#[cfg(not(target_env = "sgx"))]
use rand::rngs::OsRng;

#[cfg(target_env = "sgx")]
use rdrand::RdRand;

// for the sgx target we use RdRand
#[cfg(target_env = "sgx")]
pub fn getrandom() -> RdRand {
    RdRand::new().expect("Something went wrong with getting randomness")
}

// for all other targets we rely on OsRng (in wasm via js bridge)
#[cfg(not(target_env = "sgx"))]
pub fn getrandom() -> OsRng {
    OsRng::new().expect("Something went wrong with getting randomness")
}

pub fn getrandom_192bits() -> [u8; 24] {
    let mut rng = getrandom();
    let mut buffer = [0; 24];
    rng.fill_bytes(&mut buffer);
    buffer
}
