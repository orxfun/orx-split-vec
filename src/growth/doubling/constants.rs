pub(super) const FIRST_FRAGMENT_CAPACITY_POW: usize = 2;
pub(super) const FIRST_FRAGMENT_CAPACITY: usize = usize::pow(2, FIRST_FRAGMENT_CAPACITY_POW as u32);
pub(super) const SIZE_USIZE: usize = core::mem::size_of::<usize>() * 8;
pub(super) const OFFSET_FRAGMENT_IDX: usize = SIZE_USIZE - FIRST_FRAGMENT_CAPACITY_POW - 1;

const fn fragment_capacity(fragment_idx: usize) -> usize {
    usize::pow(2, (fragment_idx + FIRST_FRAGMENT_CAPACITY_POW) as u32)
}

const fn cumulative_capacity(fragment_idx: usize) -> usize {
    usize::pow(2, (fragment_idx + FIRST_FRAGMENT_CAPACITY_POW + 1) as u32) - FIRST_FRAGMENT_CAPACITY
}

#[cfg(target_pointer_width = "32")]
pub const CUMULATIVE_CAPACITIES_LEN: usize = 30;
#[cfg(target_pointer_width = "64")]
pub const CUMULATIVE_CAPACITIES_LEN: usize = 33;

pub(super) const CAPACITIES: [usize; CUMULATIVE_CAPACITIES_LEN - 1] = [
    fragment_capacity(0),
    fragment_capacity(1),
    fragment_capacity(2),
    fragment_capacity(3),
    fragment_capacity(4),
    fragment_capacity(5),
    fragment_capacity(6),
    fragment_capacity(7),
    fragment_capacity(8),
    fragment_capacity(9),
    fragment_capacity(10),
    fragment_capacity(11),
    fragment_capacity(12),
    fragment_capacity(13),
    fragment_capacity(14),
    fragment_capacity(15),
    fragment_capacity(16),
    fragment_capacity(17),
    fragment_capacity(18),
    fragment_capacity(19),
    fragment_capacity(20),
    fragment_capacity(21),
    fragment_capacity(22),
    fragment_capacity(23),
    fragment_capacity(24),
    fragment_capacity(25),
    fragment_capacity(26),
    fragment_capacity(27),
    fragment_capacity(28),
    #[cfg(target_pointer_width = "64")]
    fragment_capacity(29),
    #[cfg(target_pointer_width = "64")]
    fragment_capacity(30),
    #[cfg(target_pointer_width = "64")]
    fragment_capacity(31),
];

pub(super) const CUMULATIVE_CAPACITIES: [usize; CUMULATIVE_CAPACITIES_LEN] = [
    0,
    cumulative_capacity(0),
    cumulative_capacity(1),
    cumulative_capacity(2),
    cumulative_capacity(3),
    cumulative_capacity(4),
    cumulative_capacity(5),
    cumulative_capacity(6),
    cumulative_capacity(7),
    cumulative_capacity(8),
    cumulative_capacity(9),
    cumulative_capacity(10),
    cumulative_capacity(11),
    cumulative_capacity(12),
    cumulative_capacity(13),
    cumulative_capacity(14),
    cumulative_capacity(15),
    cumulative_capacity(16),
    cumulative_capacity(17),
    cumulative_capacity(18),
    cumulative_capacity(19),
    cumulative_capacity(20),
    cumulative_capacity(21),
    cumulative_capacity(22),
    cumulative_capacity(23),
    cumulative_capacity(24),
    cumulative_capacity(25),
    cumulative_capacity(26),
    cumulative_capacity(27),
    cumulative_capacity(28),
    #[cfg(target_pointer_width = "64")]
    cumulative_capacity(29),
    #[cfg(target_pointer_width = "64")]
    cumulative_capacity(30),
    #[cfg(target_pointer_width = "64")]
    cumulative_capacity(31),
];
