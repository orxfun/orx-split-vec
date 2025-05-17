// cargo run --release --example bench_parallelization

mod utils;

use clap::Parser;
use orx_parallel::*;
use orx_split_vec::*;
use rayon::iter::*;
use utils::timed_collect_all;

#[derive(Parser, Debug)]
struct Args {
    /// Number of items in the input iterator.
    #[arg(long, default_value_t = 1_000_000)]
    len: usize,
    /// Number of repetitions to measure time; total time will be reported.
    #[arg(long, default_value_t = 100)]
    num_repetitions: usize,
}

fn fibonacci(n: usize) -> usize {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

fn main() {
    let args = Args::parse();

    let expected_output = {
        let split_vec: SplitVec<_> = (0..args.len as usize).collect();

        split_vec
            .into_iter()
            .filter(|x| x % 3 != 0)
            .map(|x| x + fibonacci(x % 40))
            .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
            .collect::<Vec<_>>()
    };

    let computations: Vec<(&str, Box<dyn Fn() -> Vec<String>>)> = vec![
        (
            "Sequential over Vec",
            Box::new(move || {
                let split_vec: SplitVec<_> = (0..args.len as usize).collect();

                split_vec
                    .into_iter()
                    .filter(|x| x % 3 != 0)
                    .map(|x| x + fibonacci(x % 40))
                    .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over Vec using rayon",
            Box::new(move || {
                let vec: Vec<_> = (0..args.len as usize).collect();
                vec.into_par_iter()
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 40))
                    .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over Vec using orx_parallel",
            Box::new(move || {
                let vec: Vec<_> = (0..args.len as usize).collect();

                vec.into_par()
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 40))
                    .filter_map(|x| (x % 2 == 0).then(|| x.to_string()))
                    .collect::<Vec<_>>()
            }),
        ),
        (
            "Parallelized over SplitVec using orx_parallel",
            Box::new(move || {
                let split_vec: SplitVec<_> = (0..args.len as usize).collect();

                split_vec
                    .into_par() // replace iter (into_iter) with par (into_par) to parallelize !
                    .filter(|x| *x % 3 != 0)
                    .map(|x| x + fibonacci(x % 40))
                    .filter(|x| x % 2 == 0)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            }),
        ),
    ];

    timed_collect_all(
        "benchmark_parallelization",
        args.num_repetitions,
        &expected_output,
        &computations,
    );
}
