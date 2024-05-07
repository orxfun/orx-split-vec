use crate::Fragment;
use std::slice::Iter;

type Outer<'a, T> = Iter<'a, Fragment<T>>;
type Inner<'a, T> = Iter<'a, T>;

pub fn all<'a, T, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, mut f: F) -> bool
where
    F: FnMut(&'a T) -> bool,
{
    for x in inner {
        if !f(x) {
            return false;
        }
    }
    for fragment in outer {
        for x in fragment.iter() {
            if !f(x) {
                return false;
            }
        }
    }
    true
}

pub fn any<'a, T, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, mut f: F) -> bool
where
    F: FnMut(&'a T) -> bool,
{
    for x in inner {
        if f(x) {
            return true;
        }
    }
    for fragment in outer {
        for x in fragment.iter() {
            if f(x) {
                return true;
            }
        }
    }
    false
}

pub fn fold<'a, T, B, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, init: B, mut f: F) -> B
where
    F: FnMut(B, &'a T) -> B,
{
    let mut res = inner.fold(init, &mut f);
    for fragment in outer {
        res = fragment.iter().fold(res, &mut f);
    }
    res
}
