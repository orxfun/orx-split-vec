use crate::Fragment;
use core::fmt::Debug;

impl<T> Debug for Fragment<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.data.fmt(f)
    }
}
