mod floyd;
mod random_vector;

fn main() {
    let vector_size = 1_000_000;
    let upperbound = None;

    let vector = random_vector::random_vector_with_duplicates(vector_size, upperbound);

    println!("Vector created...");

    let dups = floyd::floyd_cycle_entry_detection(&vector);
    println!(
        "Duplicate value: {} at index {} and {} in {} iterations",
        dups.0, dups.1, dups.2, dups.3
    );
    assert!(vector[dups.1] == vector[dups.2]);
}
