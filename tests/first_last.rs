use orx_pinned_vec::PinnedVec;
use orx_split_vec::SplitVec;

#[test]
fn last_on_pop() {
    let n = 164135;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for _ in 0..(n - 1) {
        let expected_pop = vec.len() - 1;
        let expected_last = expected_pop - 1;
        let pop = vec.pop();
        assert_eq!(Some(expected_pop), pop);
        assert_eq!(Some(&expected_last), vec.last());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&0), vec.last());

    let pop = vec.pop();
    assert_eq!(Some(0), pop);
    assert!(vec.last().is_none());
}

#[test]
fn last_on_remove() {
    let n = 1641;
    let expected_last = n - 1;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for _ in 0..(n - 1) {
        let i = if vec.len() <= 2 { 0 } else { vec.len() / 2 };
        let _ = vec.remove(i);
        assert_eq!(Some(&expected_last), vec.last());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&expected_last), vec.last());

    let last = vec.remove(0);
    assert_eq!(expected_last, last);
    assert!(vec.last().is_none());
}

#[test]
fn last_on_truncate() {
    let n = 164135;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for _ in 0..(n - 1) {
        let new_len = vec.len() - 1;
        let expected_last = vec.len() - 2;
        vec.truncate(new_len);
        assert_eq!(Some(&expected_last), vec.last());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&0), vec.last());

    let pop = vec.pop();
    assert_eq!(Some(0), pop);
    assert!(vec.last().is_none());
}

#[test]
fn first_on_pop() {
    let n = 164135;
    let expected_first = 0;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for _ in 0..(n - 1) {
        let expected_pop = vec.len() - 1;
        let pop = vec.pop();
        assert_eq!(Some(expected_pop), pop);
        assert_eq!(Some(&expected_first), vec.first());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&0), vec.first());

    let pop = vec.pop();
    assert_eq!(Some(0), pop);
    assert!(vec.first().is_none());
}

#[test]
fn first_on_remove() {
    let n = 1635;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for i in 0..(n - 1) {
        let expected_removed = i;
        let expected_first = i + 1;
        let removed = vec.remove(0);
        assert_eq!(expected_removed, removed);
        assert_eq!(Some(&expected_first), vec.first());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&(n - 1)), vec.first());

    let last = vec.remove(0);
    assert_eq!(n - 1, last);
    assert!(vec.first().is_none());
}

#[test]
fn first_on_truncate() {
    let n = 164135;

    let mut vec = SplitVec::new();
    for i in 0..n {
        vec.push(i);
    }

    for _ in 0..(n - 1) {
        let new_len = vec.len() - 1;
        vec.truncate(new_len);
        assert_eq!(Some(&0), vec.first());
    }

    assert_eq!(vec.len(), 1);
    assert_eq!(Some(&0), vec.first());

    let pop = vec.pop();
    assert_eq!(Some(0), pop);
    assert!(vec.first().is_none());
}
