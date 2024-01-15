use crate::prelude::*;

#[test]
fn append_when_empty() {
    let mut rec = SplitVec::with_recursive_growth();

    rec.append(vec!['b', 'c']);
    assert_eq!(rec, &['b', 'c']);

    _ = rec.pop();
    assert_eq!(rec, &['b']);

    _ = rec.pop();
    assert_eq!(rec, &[]);

    rec.push('a');
    rec.push('b');
    rec.push('c');
    rec.push('d');
    rec.push('e');
    rec.push('f');
    assert_eq!(rec, &['a', 'b', 'c', 'd', 'e', 'f']);

    rec.append(vec!['h']);
    assert_eq!(rec, &['a', 'b', 'c', 'd', 'e', 'f', 'h']);
}

#[test]
fn append_remove() {
    fn assert_seq(rec: &SplitVec<usize, Recursive>, expected_len: usize) {
        assert_eq!(expected_len, rec.len());
        for i in 0..expected_len {
            assert_eq!(i, rec[i]);
        }
    }

    let mut rec = SplitVec::with_recursive_growth();

    rec.push(0);
    rec.append((1..30).collect::<Vec<_>>());

    assert_seq(&rec, 30);

    for i in 0..10 {
        rec.remove(i);
    }
    for (i, x) in rec.iter_mut().enumerate() {
        *x = i;
    }

    assert_seq(&rec, 20);

    let mut other = SplitVec::with_linear_growth(6);
    for i in 20..45 {
        other.push(i);
    }

    rec.append(other);
    assert_seq(&rec, 45);

    rec.truncate(14);
    assert_seq(&rec, 14);

    let mut other = SplitVec::with_doubling_growth();
    for i in 14..1243 {
        other.push(i);
    }

    rec.append(other);
    assert_seq(&rec, 1243);
}
