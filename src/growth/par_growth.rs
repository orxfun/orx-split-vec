use crate::Growth;
use orx_concurrent_iter::implementations::jagged_arrays::JaggedIndexer;

/// A [`Growth`] that supports parallelization.
///
/// All types implementing both [`Growth`] and [`JaggedIndexer`] implement [`ParGrowth`].
///
/// [`Doubling`], [`Linear`] and [`Recursive`] growth strategies all support parallel growth.
///
/// [`Doubling`]: crate::Doubling
/// [`Linear`]: crate::Linear
/// [`Recursive`]: crate::Recursive
pub trait ParGrowth: Growth + JaggedIndexer {}

impl<G: Growth + JaggedIndexer> ParGrowth for G {}
