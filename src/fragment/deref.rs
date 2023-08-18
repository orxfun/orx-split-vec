use crate::Fragment;
use std::ops::{Deref, DerefMut};

impl<T> Deref for Fragment<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> DerefMut for Fragment<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
