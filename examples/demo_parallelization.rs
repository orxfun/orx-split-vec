// cargo run --release --example demo_parallelization

use orx_parallel::*;
use orx_split_vec::*;

fn main() {
    let n = 12345;
    let input: SplitVec<_> = (0..n).map(|x| x.to_string()).collect();
    let expected_num_characters = 50615;

    // computation using iterators

    let total_num_characters: usize = input.iter().map(|x| x.len()).sum();
    assert_eq!(total_num_characters, expected_num_characters);

    // computation using parallel iterator: replace `iter()` with `par()`

    let total_num_characters: usize = input.par().map(|x| x.len()).sum();
    assert_eq!(total_num_characters, expected_num_characters);

    // configure parallel computation
    let total_num_characters: usize = input
        .par()
        .num_threads(2)
        .chunk_size(64)
        .map(|x| x.len())
        .sum();
    assert_eq!(total_num_characters, expected_num_characters);

    // consuming parallel iterator: replace `into_iter` with `into_par`
    let total_num_characters: usize = input.into_par().map(|x| x.len()).sum();
    assert_eq!(total_num_characters, expected_num_characters);
}
