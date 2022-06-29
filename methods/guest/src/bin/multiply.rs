#![no_main]
#![no_std]

use risc0_zkvm_guest::env;
use sparse_merkle_tree::{ default_store::DefaultStore,
    error::Error, MerkleProof,CompiledMerkleProof,
    SparseMerkleTree, traits::{Value, Hasher}, H256};
    use checker_core::Blake2bHasher;

#[macro_use]
extern crate alloc;

type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

fn slice_to_array_32<T>(slice: &[T]) -> &[T; 32] {
        let ptr = slice.as_ptr() as *const [T; 32];
        unsafe {&*ptr}
}

fn construct_smt(key: H256, value: H256) -> bool{
    let mut tree = SMT::default();
        tree.update(key.clone(), value.clone()).expect("update");
        let proof = tree.merkle_proof(vec![key.clone()]).expect("gen proof");
        proof.verify::<Blake2bHasher>(tree.root(), vec![(key, value.to_h256())]).unwrap()
}

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let key: &[u8] = env::read();
    let value: &[u8] = env::read();
    let key: H256 = H256::from(*slice_to_array_32(key));
    let value: H256 = H256::from(*slice_to_array_32(value));
    let c: bool =  construct_smt(key, value);
    env::commit(&c);
}
