#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        $fun::<$crate::Linear>(SplitVec::with_linear_growth(2));
        $fun::<$crate::Doubling>(SplitVec::with_doubling_growth());
    };
}

#[cfg(test)]
#[derive(Debug, PartialEq, Clone)]
pub struct Num(usize);
impl Num {
    pub fn new(number: usize) -> Self {
        Self(number)
    }
}
