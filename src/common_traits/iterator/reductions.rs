use crate::Fragment;
use core::slice::Iter;

type Outer<'a, T> = Iter<'a, Fragment<T>>;
type Inner<'a, T> = Iter<'a, T>;

pub fn all<'a, T, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, mut f: F) -> bool
where
    F: FnMut(&'a T) -> bool,
{
    match inner.all(&mut f) {
        false => false,
        true => !outer.any(|inner| !inner.iter().all(&mut f)),
    }
}

pub fn any<'a, T, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, mut f: F) -> bool
where
    F: FnMut(&'a T) -> bool,
{
    match inner.any(&mut f) {
        true => true,
        false => outer.any(|inner| inner.iter().any(&mut f)),
    }
}

pub fn fold<'a, T, B, F>(outer: &mut Outer<'a, T>, inner: &mut Inner<'a, T>, init: B, mut f: F) -> B
where
    F: FnMut(B, &'a T) -> B,
{
    let res = inner.fold(init, &mut f);
    outer.fold(res, |res, inner| inner.iter().fold(res, &mut f))
}
