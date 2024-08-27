use orx_split_vec::*;

#[test]
fn fragment_capacity_doubling() {
    let growth = Doubling;

    let mut capacity = 4;

    for f in 0..31 {
        assert_eq!(growth.fragment_capacity_of(f), capacity);
        capacity *= 2;
    }
}

#[test]
fn fragment_capacity_linear() {
    let growth = Linear::new(10);

    let capacity = u32::pow(2, 10) as usize;

    for f in 0..31 {
        assert_eq!(growth.fragment_capacity_of(f), capacity);
    }
}
