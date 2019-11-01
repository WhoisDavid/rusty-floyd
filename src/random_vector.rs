use rand::distributions::Uniform;
use rand::Rng;

#[allow(dead_code)]
pub fn random_vector_with_duplicates(size: u64, optional_upperbound: Option<u64>) -> Vec<u64> {
    let upperbound = match optional_upperbound {
        Some(i) => i,
        None => size - 1,
    };

    let mut rng = rand::thread_rng();

    // Return value
    (0..size) // iterator. this will yield 0, 1, 2, 3, etc.
        .map(|_| {
            // map a closure with an anonymous var
            rng.gen_range(0, upperbound) // generate a random int [1, upperbound)
        })
        .collect::<Vec<u64>>() // collect the iterator into a vector
}

#[allow(dead_code)]
pub fn random_vector_with_duplicates_unif(size: u64, optional_upperbound: Option<u64>) -> Vec<u64> {
    let upperbound = match optional_upperbound {
        Some(i) => i,
        None => size - 1,
    };

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, upperbound);

    // Return value
    (0..size) // iterator. this will yield 0, 1, 2, 3, etc.
        .map(|_| {
            // map a closure with an anonymous var
            rng.sample(&range) // generate a random int [1, upperbound)
        })
        .collect::<Vec<u64>>() // collect the iterator into a vector
}

#[allow(dead_code)]
pub fn random_vector_with_duplicates_imperative(
    size: u64,
    optional_upperbound: Option<u64>,
) -> Vec<u64> {
    let upperbound = match optional_upperbound {
        Some(i) => i,
        None => size - 1,
    };

    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<u64>::with_capacity(size as usize);
    for _ in 0..size {
        numbers.push(rng.gen_range(0, upperbound));
    }
    return numbers;
}
