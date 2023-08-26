use crate::Fragment;
use std::fmt::Debug;

impl<T> Debug for Fragment<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
