#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
#[cfg(feature = "parallel")]
use orx_parallel::*;
use orx_split_vec::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const TEST_LARGE_OUTPUT: bool = false;

const LARGE_OUTPUT_LEN: usize = match TEST_LARGE_OUTPUT {
    true => 64,
    false => 0,
};
const SEED: u64 = 5426;
const FIB_UPPER_BOUND: u32 = 201;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Output {
    name: String,
    numbers: [i64; LARGE_OUTPUT_LEN],
}

fn map(idx: &usize) -> Output {
    let idx = *idx;
    let prefix = match idx % 7 {
        0 => "zero-",
        3 => "three-",
        _ => "sth-",
    };
    let fib = fibonacci(&(idx as u32));
    let name = format!("{}-fib-{}", prefix, fib);

    let mut numbers = [0i64; LARGE_OUTPUT_LEN];
    for (i, x) in numbers.iter_mut().enumerate() {
        *x = match (idx * 7 + i) % 3 {
            0 => idx as i64 + i as i64,
            _ => idx as i64 - i as i64,
        };
    }

    Output { name, numbers }
}

fn filter(output: &Output) -> bool {
    let last_char = output.name.chars().last().unwrap();
    let last_digit: u32 = last_char.to_string().parse().unwrap();
    last_digit < 4
}

fn fibonacci(n: &u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..*n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

fn inputs(len: usize) -> Vec<usize> {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    (0..len)
        .map(|_| rng.random_range(0..FIB_UPPER_BOUND) as usize)
        .collect()
}

fn seq(inputs: &[usize]) -> Vec<Output> {
    inputs.iter().map(map).filter(filter).collect()
}

#[cfg(feature = "parallel")]
fn par_over_slice(inputs: &[usize]) -> Vec<Output> {
    inputs.into_par().map(map).filter(filter).collect()
}

#[cfg(feature = "parallel")]
fn par_over_split_vec<G: Growth>(inputs: &SplitVec<usize, G>) -> Vec<Output> {
    inputs.into_par().map(map).filter(filter).collect()
}

fn run(c: &mut Criterion) {
    let treatments = [65_536, 65_536 * 4];

    #[allow(unused_mut)]
    let mut group = c.benchmark_group("par_collect_map_filter");

    #[cfg(feature = "parallel")]
    for n in &treatments {
        let input = inputs(*n);
        let expected = seq(&input);

        let input_doubling: SplitVec<_, Doubling> = input.iter().copied().collect();

        let input_linear = {
            let mut input_linear = SplitVec::with_linear_growth(10);
            input_linear.extend_from_slice(&input);
            input_linear
        };

        group.bench_with_input(BenchmarkId::new("seq", n), n, |b, _| {
            assert_eq!(&expected, &seq(&input));
            b.iter(|| seq(black_box(&input)))
        });

        group.bench_with_input(BenchmarkId::new("par_over_slice", n), n, |b, _| {
            assert_eq!(&expected, &par_over_slice(&input));
            b.iter(|| par_over_slice(black_box(&input)))
        });

        group.bench_with_input(
            BenchmarkId::new("par_over_split_vec_doubling", n),
            n,
            |b, _| {
                assert_eq!(&expected, &par_over_split_vec(&input_doubling));
                b.iter(|| par_over_split_vec(black_box(&input_doubling)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("par_over_split_vec_linear", n),
            n,
            |b, _| {
                assert_eq!(&expected, &par_over_split_vec(&input_linear));
                b.iter(|| par_over_split_vec(black_box(&input_linear)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, run);
criterion_main!(benches);
