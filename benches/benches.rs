#![feature(test)]
extern crate test;

use rusty_floyd::floyd;
use rusty_floyd::random_vector;
use test::Bencher;

const size: u64 = 1_000_000;
const upperbound: Option<u64> = None;

#[bench]
fn random_vec_unif(b: &mut Bencher) {
    b.iter(|| random_vector::random_vector_with_duplicates_unif(size, upperbound));
}

#[bench]
fn random_vec_gen(b: &mut Bencher) {
    b.iter(|| random_vector::random_vector_with_duplicates(size, upperbound));
}

#[bench]
fn floyd_cycle_entry(b: &mut Bencher) {
    let vector = random_vector::random_vector_with_duplicates(size, upperbound);
    b.iter(|| floyd::floyd_cycle_entry_detection(&vector));
}
