#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        let custom_fun = std::rc::Rc::new(
            |fragments: &[$crate::Fragment<_>]| {
                if fragments.len() % 2 == 0 {
                    2
                } else {
                    8
                }
            },
        );
        $fun::<$crate::CustomGrowth<usize>>(SplitVec::with_custom_growth_function(custom_fun));
        $fun::<$crate::LinearGrowth>(SplitVec::with_linear_growth(2));
        $fun::<$crate::DoublingGrowth>(SplitVec::with_doubling_growth(2));
        $fun::<$crate::ExponentialGrowth>(SplitVec::with_exponential_growth(4, 1.5));
        $fun::<$crate::ExponentialGrowth>(SplitVec::with_exponential_growth(4, 2.5));
        $fun::<$crate::FixedCapacity>(SplitVec::with_fixed_capacity(1000));
    };
}
