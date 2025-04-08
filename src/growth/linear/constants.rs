const fn fixed_capacity(const_size_power: usize) -> usize {
    usize::pow(2, const_size_power as u32)
}

#[cfg(target_pointer_width = "32")]
const CAPACITIES_LEN: usize = 29;
#[cfg(target_pointer_width = "64")]
const CAPACITIES_LEN: usize = 32;

pub(super) const FIXED_CAPACITIES: [usize; CAPACITIES_LEN] = [
    fixed_capacity(0),
    fixed_capacity(1),
    fixed_capacity(2),
    fixed_capacity(3),
    fixed_capacity(4),
    fixed_capacity(5),
    fixed_capacity(6),
    fixed_capacity(7),
    fixed_capacity(8),
    fixed_capacity(9),
    fixed_capacity(10),
    fixed_capacity(11),
    fixed_capacity(12),
    fixed_capacity(13),
    fixed_capacity(14),
    fixed_capacity(15),
    fixed_capacity(16),
    fixed_capacity(17),
    fixed_capacity(18),
    fixed_capacity(19),
    fixed_capacity(20),
    fixed_capacity(21),
    fixed_capacity(22),
    fixed_capacity(23),
    fixed_capacity(24),
    fixed_capacity(25),
    fixed_capacity(26),
    fixed_capacity(27),
    fixed_capacity(28),
    #[cfg(target_pointer_width = "64")]
    fixed_capacity(29),
    #[cfg(target_pointer_width = "64")]
    fixed_capacity(30),
    #[cfg(target_pointer_width = "64")]
    fixed_capacity(31),
];
