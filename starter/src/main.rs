use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use sparse_merkle_tree::H256;
use blake2b_ref::{Blake2b, Blake2bBuilder};


fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32).personal(b"SMT").build()
}
fn hash_message(msg: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32).personal(b"SMT").build();
    blake2b.update(msg);
    blake2b.finalize(&mut output);
    output
}

fn main() {
    for (i, word) in "The quick brown fox jumps over the lazy dog"
    .split_whitespace()
    .enumerate()
    {
        let key: H256 = hash_message(&(i as u32).to_le_bytes()).into();
        let value: H256 = hash_message(word.as_bytes()).into();
        let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
        prover.add_input(to_vec(&key.as_slice()).unwrap().as_slice()).unwrap();
        prover.add_input(to_vec(&value.as_slice()).unwrap().as_slice()).unwrap();
        let receipt = prover.run().unwrap();
        let c: bool = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
        println!("I know the factors of {}, and I can prove it!", c);
        receipt.verify(MULTIPLY_ID).unwrap();
    }         
}
