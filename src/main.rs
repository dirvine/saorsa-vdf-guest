#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_zkvm::io;

pub fn main() {
    // Read inputs:
    // 1. challenge (seed): [u8; 32]
    // 2. iterations: u64
    let challenge: [u8; 32] = io::read();
    let iterations: u64 = io::read();

    let mut state = challenge;

    // Iterated Hashing VDF
    // H(H(...H(seed)))
    // This is sequential and cannot be parallelized.
    for _ in 0..iterations {
        state = *blake3::hash(&state).as_bytes();
    }

    // Commit the result (the final VDF output)
    io::commit(&state);
}
