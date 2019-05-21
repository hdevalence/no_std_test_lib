#![no_std]

extern crate curve25519_dalek;
extern crate rand_core;

use rand_core::{RngCore, CryptoRng};

/// An example function that uses curve25519-dalek internally to "do stuff"
pub fn foo<R: RngCore + CryptoRng>(rng: &mut R) -> [u8; 32] {
    use curve25519_dalek::scalar::Scalar;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;

    let key = Scalar::random(rng);
    let point = (&key * &RISTRETTO_BASEPOINT_TABLE).compress();

    point.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
