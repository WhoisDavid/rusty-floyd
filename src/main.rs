mod floyd;
mod random_vector;

fn main() {
    let vector_size = 1_000_000;
    let upperbound = None;

    let vector = random_vector::random_vector_with_duplicates(vector_size, upperbound);
    // let vector = vec![1, 1, 3, 3, 1];

    // println!("{:?}", vector);
    println!("Vector created...");

    let dups = floyd::floyd_cycle_entry_detection(&vector);
    println!("Duplicate value: {} at index {} and {}", dups.0, dups.1, dups.2);
    assert!(vector[dups.1] == vector[dups.2]);
}