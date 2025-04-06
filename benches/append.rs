use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use orx_split_vec::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const NUM_APPEND_OPS: usize = 32;

struct Vectors(Vec<Vec<usize>>);
impl Clone for Vectors {
    fn clone(&self) -> Self {
        let vecs = (0..self.0.len()).map(|i| self.0[i].clone()).collect();
        Self(vecs)
    }
}

fn get_vectors(n: usize) -> Vectors {
    let mut rng = ChaCha8Rng::seed_from_u64(685412);
    Vectors(
        (0..NUM_APPEND_OPS)
            .map(|_| (0..n).map(|_| rng.random_range(0..n)).collect())
            .collect(),
    )
}

fn calc_std_vec(mut vec: Vec<usize>, mut vectors: Vectors) -> Vec<usize> {
    for x in &mut vectors.0 {
        vec.append(x);
    }
    vec
}

fn calc_split_vec_extend<G: Growth>(
    mut vec: SplitVec<usize, G>,
    mut vectors: Vectors,
) -> SplitVec<usize, G> {
    for x in &mut vectors.0 {
        vec.extend_from_slice(x);
        x.clear();
    }
    vec
}

fn calc_split_vec_append(
    mut vec: SplitVec<usize, Recursive>,
    vectors: Vectors,
) -> SplitVec<usize, Recursive> {
    let vectors = vectors.0;
    for x in vectors {
        vec.append(x)
    }
    vec
}

fn bench(c: &mut Criterion) {
    let treatments = vec![1_024, 16_384, 262_144, 4_194_304];

    let mut group = c.benchmark_group("append");

    for n in treatments {
        let treatment = format!("n={}]", n);
        let vectors = get_vectors(n);

        group.bench_with_input(BenchmarkId::new("std_vec_new", &treatment), &n, |b, _| {
            b.iter_batched(
                || get_vectors(n),
                |vectors| calc_std_vec(black_box(Vec::new()), black_box(vectors)),
                BatchSize::LargeInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("std_vec_with_exact_capacity", &treatment),
            &n,
            |b, _| {
                let capacity: usize = vectors.0.iter().map(|x| x.len()).sum();
                b.iter_batched(
                    || get_vectors(n),
                    |vectors| {
                        calc_std_vec(black_box(Vec::with_capacity(capacity)), black_box(vectors))
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("split_vec_linear - 2^10", &treatment),
            &n,
            |b, _| {
                b.iter_batched(
                    || get_vectors(n),
                    |vectors| {
                        calc_split_vec_extend(
                            black_box(SplitVec::with_linear_growth(10)),
                            black_box(vectors),
                        )
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("split_vec_doubling", &treatment),
            &n,
            |b, _| {
                b.iter_batched(
                    || get_vectors(n),
                    |vectors| {
                        calc_split_vec_extend(
                            black_box(SplitVec::with_doubling_growth()),
                            black_box(vectors),
                        )
                    },
                    BatchSize::LargeInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("split_vec_recursive", &treatment),
            &n,
            |b, _| {
                b.iter_batched(
                    || get_vectors(n),
                    |vectors| {
                        calc_split_vec_append(
                            black_box(SplitVec::with_recursive_growth()),
                            black_box(vectors),
                        )
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
