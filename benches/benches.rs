#![feature(test)]
extern crate test;

use rusty_floyd::floyd;
use rusty_floyd::random_vector;
use test::Bencher;

const SIZE: u64 = 1_000_000;
const UPPERBOUND: Option<u64> = None;

#[bench]
fn random_vec_unif(b: &mut Bencher) {
    b.iter(|| random_vector::random_vector_with_duplicates_unif(SIZE, UPPERBOUND));
}

#[bench]
fn random_vec_gen(b: &mut Bencher) {
    b.iter(|| random_vector::random_vector_with_duplicates(SIZE, UPPERBOUND));
}

#[bench]
fn floyd_cycle_entry(b: &mut Bencher) {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1];
    b.iter(|| floyd::floyd_cycle_entry_detection(&vector));
}
