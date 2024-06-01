use orx_split_vec::prelude::*;

#[test]
fn grow_and_initialize() {
    let mut vec = SplitVec::new();

    for i in 0..2044 {
        vec.push(i);
    }
    let capacity_before = vec.capacity();

    let result = vec.grow_and_initialize(0, || 42);
    assert_eq!(result, Ok(vec.capacity()));
    assert_eq!(vec.capacity(), capacity_before);
    assert_eq!(vec.len(), vec.capacity());
    for i in 0..2044 {
        assert_eq!(vec.get(i), Some(&i));
    }
    for i in 2044..vec.len() {
        assert_eq!(vec.get(i), Some(&42));
    }

    let result = vec.grow_and_initialize(10000, || 42);
    assert_eq!(result, Ok(vec.capacity()));
    assert_eq!(vec.len(), vec.capacity());
    assert!(vec.capacity() >= 10000);
    for i in 0..2044 {
        assert_eq!(vec.get(i), Some(&i));
    }
    for i in 2044..vec.len() {
        assert_eq!(vec.get(i), Some(&42));
    }
}
