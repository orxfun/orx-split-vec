use std::{fmt::Debug, rc::Rc};

pub(crate) type GetCapacityOfFragment = Rc<dyn Fn(usize) -> usize>;

#[derive(Clone)]
/// Growth policy of fragments in a split vector.
///
/// A policy can be defined by one of the three constructors.
///
/// * `FragmentGrowth::exponential(initial_capacity, capacity_multiplier)`
///     * capacity of the f-th fragment will be computed as `initial_capacity * capacity_multiplier^f`.
///
/// * `FragmentGrowth::constant(constant_fragment_length)`
///     * capacity of all fragments will be equal to `constant_fragment_length`, leading to a linear growth.
///
/// * `FragmentGrowth::by_function(get_capacity_of_fragment)`
///     * capacity of the f-th fragment will be computed as `get_capacity_of_fragment(f)`;
///     * any custom growth policy can be represented by this functional form.
pub struct FragmentGrowth {
    fun: GetCapacityOfFragment,
}

impl Default for FragmentGrowth {
    /// Creates the default growth with an initial capacity of 4 and capacity multiplier of 1.5.
    ///
    /// Fragment capacities with the default function will be:
    /// * fragment-0: capacity=4 -> total-capacity=4
    /// * fragment-1: capacity=6 -> total-capacity=10
    /// * fragment-2: capacity=9 -> total-capacity=19
    /// * fragment-3: capacity=13 -> total-capacity=32
    /// * ...
    fn default() -> Self {
        fn default_exponential_growth(f: usize) -> usize {
            const INITIAL_CAPACITY: usize = 4;
            const CAPACITY_MULTIPLIER: f32 = 1.5;
            (INITIAL_CAPACITY as f32 * f32::powf(CAPACITY_MULTIPLIER, f as f32)) as usize
        }
        Self {
            fun: Rc::new(default_exponential_growth),
        }
    }
}

impl FragmentGrowth {
    /// Creates an exponential growth policy with the given positive coefficients.
    ///
    /// The capacity of the f-th fragment will be computed as:
    ///
    /// `initial_capacity * capacity_multiplier^f`
    ///
    /// # Panics
    ///
    /// Panics when:
    /// * `initial_capacity` is nonpositive, or
    /// * `capacity_multiplier` is nonpositive;
    ///
    /// as either of these cases would lead to zero capacity allocations for growth.
    ///
    /// # Examples
    ///
    /// Below example demonstrates an exponential growth with a multiplier of 1.5.
    /// ```
    /// use orx_split_vec::{FragmentGrowth, SplitVec};
    ///
    /// let growth = FragmentGrowth::exponential(4, 1.5);
    /// let mut vec = SplitVec::with_growth(growth);
    ///
    /// assert_eq!(4, vec.capacity());
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4]);
    /// assert_eq!(5, vec.len());
    /// assert_eq!(4 + 6, vec.capacity());
    ///
    /// vec.extend_from_slice(&[0, 1, 2, 3, 4]);
    /// assert_eq!(10, vec.len());
    /// assert_eq!(4 + 6, vec.capacity());
    ///
    /// vec.push(42);
    /// assert_eq!(11, vec.len());
    /// assert_eq!(4 + 6 + 9, vec.capacity());
    /// assert_eq!(4, vec.fragments()[0].capacity());
    /// assert_eq!(6, vec.fragments()[1].capacity());
    /// assert_eq!(9, vec.fragments()[2].capacity());
    /// ```
    ///
    /// One can also allocate the same amount of memory every time new capacity is required.
    ///
    /// ```
    /// use orx_split_vec::{FragmentGrowth, SplitVec};
    ///
    /// let growth = FragmentGrowth::exponential(10, 1.0);
    /// let mut vec = SplitVec::with_growth(growth);
    ///
    /// assert_eq!(10, vec.capacity());
    ///
    /// for x in 0..10 {
    ///     vec.push(x);
    /// }
    ///
    /// assert_eq!(10, vec.len());
    /// assert_eq!(10, vec.capacity());
    ///
    /// vec.push(42);
    /// assert_eq!(11, vec.len());
    /// assert_eq!(20, vec.capacity());
    /// for fragment in vec.fragments() {
    ///     assert_eq!(10, fragment.capacity());
    /// }
    /// ```
    pub fn exponential(initial_capacity: usize, capacity_multiplier: f32) -> Self {
        assert!(initial_capacity > 0, "initial capacity must be positive");
        assert!(
            capacity_multiplier > 1e-5,
            "capacity multiplier must be positive"
        );
        let fun: GetCapacityOfFragment = Rc::new(move |f| {
            (initial_capacity as f32 * f32::powf(capacity_multiplier, f as f32)) as usize
        });
        Self { fun }
    }
    /// Creates a constant growth policy where every fragment will have the same length;
    /// i.e., `constant_fragment_length`.
    ///
    /// Note that `FragmentGrowth::constant(10)` is a shorthand for `FragmentGrowth::exponential(10, 1.0)`.
    ///
    /// # Panics
    ///
    /// Panics when:
    /// * `constant_fragment_length` is nonpositive
    ///
    /// as ot would lead to zero capacity allocations for growth.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::{FragmentGrowth, SplitVec};
    ///
    /// let growth = FragmentGrowth::constant(10);
    /// let mut vec = SplitVec::with_growth(growth);
    ///
    /// assert_eq!(10, vec.capacity());
    ///
    /// for x in 0..10 {
    ///     vec.push(x);
    /// }
    ///
    /// assert_eq!(10, vec.len());
    /// assert_eq!(10, vec.capacity());
    ///
    /// vec.push(42);
    /// assert_eq!(11, vec.len());
    /// assert_eq!(20, vec.capacity());
    /// for fragment in vec.fragments() {
    ///     assert_eq!(10, fragment.capacity());
    /// }
    /// ```
    pub fn constant(constant_fragment_length: usize) -> Self {
        assert!(
            constant_fragment_length > 0,
            "constant growth factor, fragment length, must be positive"
        );
        let fun: GetCapacityOfFragment = Rc::new(move |_| constant_fragment_length);
        Self { fun }
    }
    /// Creates a growth policy function where the capacities are computed by the given function.
    ///
    /// The capacity of the f-th fragment will be computed as:
    ///
    /// `get_capacity_by_fragment(f)`
    ///
    /// # Examples
    ///
    /// One interesting policy could be to increase the fragment lengths until it reaches a particular level.
    /// Then, each expansion could be a constant expansion.
    ///
    /// ```
    /// use orx_split_vec::{FragmentGrowth, SplitVec};
    /// use std::rc::Rc;
    ///
    /// fn get_fragment_capacity(fragment: usize) -> usize {
    ///     let exp = (4.0 * f32::powf(1.5, fragment as f32)) as usize;
    ///     exp.min(10)
    /// }
    /// let growth = FragmentGrowth::by_function(Rc::new(get_fragment_capacity));
    /// let mut vec = SplitVec::with_growth(growth);
    ///
    /// for i in 0..1000 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(4, vec.fragments()[0].capacity());
    /// assert_eq!(6, vec.fragments()[1].capacity());
    /// assert_eq!(9, vec.fragments()[2].capacity());
    /// assert_eq!(10, vec.fragments()[3].capacity());
    /// for fragment in vec.fragments().iter().skip(4) {
    ///     assert_eq!(10, fragment.capacity());
    /// }
    ///
    /// ```
    pub fn by_function(get_capacity_of_fragment: Rc<dyn Fn(usize) -> usize>) -> Self {
        Self {
            fun: get_capacity_of_fragment,
        }
    }

    /// Returns the capacity of the `fragment_index`-th fragment.
    ///
    /// This method returns the maximum of 4 and the computed capacity with the defined strategy;
    /// i.e., capacities 0, 1, 2 and 3 are not allowed.
    pub fn get_capacity(&self, fragment_index: usize) -> usize {
        let capacity = (self.fun)(fragment_index);
        capacity.max(4)
    }
}

impl Debug for FragmentGrowth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FragmentGrowth").finish()
    }
}
