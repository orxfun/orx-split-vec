use orx_parallel::*;
use orx_split_vec::*;

fn main() {
    let n = 1024;
    let values: SplitVec<String> = (0..n).map(|x| x.to_string()).collect();

    // let sum_lengths: usize = values.iter().map(|x| x.len()).sum();
    // let sum_lengths_pll: usize = values.as_par().map(|x| x.len()).sum();

    // println!("sum-lengths: {}", sum_lengths);
    // println!("sum-lengths-pll: {}", sum_lengths_pll);
}
