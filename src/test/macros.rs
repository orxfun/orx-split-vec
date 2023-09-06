#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        #[derive(Clone)]
        pub struct DoubleEveryFourFragments;
        impl Growth for DoubleEveryFourFragments {
            fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
                let do_double = fragments.len() % 4 == 0;
                let last_capacity = fragments.last().map(|f| f.capacity()).unwrap_or(4);
                if do_double {
                    last_capacity * 2
                } else {
                    last_capacity
                }
            }
        }
        $fun::<DoubleEveryFourFragments>(SplitVec::with_growth(DoubleEveryFourFragments));
        $fun::<$crate::Linear>(SplitVec::with_linear_growth(2));
        $fun::<$crate::Doubling>(SplitVec::with_doubling_growth(2));
        $fun::<$crate::Exponential>(SplitVec::with_exponential_growth(4, 1.5));
        $fun::<$crate::Exponential>(SplitVec::with_exponential_growth(4, 2.5));
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
