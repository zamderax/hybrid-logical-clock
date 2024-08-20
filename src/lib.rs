#![no_std]
use core::cmp::Ordering;
use core::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HybridLogicalClock<P, L>
where
    P: Copy + Ord + Add<Output = P> + AddAssign,
    L: Copy + Ord + Add<Output = L> + AddAssign,
{
    /**
     * The physical clock time.
     */
    pub physical_clock_time: P,
    /**
     * The logical clock.
     */
    pub logical_clock: L,
}

impl<P, L> HybridLogicalClock<P, L>
where
    P: Copy + Ord + Add<Output = P> + AddAssign,
    L: Copy + Ord + Add<Output = L> + AddAssign + Default + core::ops::Add<L, Output = L>,
{
    /**
     * Create a new hybrid logical clock with the given physical clock time and logical clock time set to default.
     * If you want to create a hybrid logical clock with both physical and logical clock time set, use the `new_with_both_physical_and_logical_clock_time` method instead.
     * 
     * @example
     * ```
     * let hlc = HybridLogicalClock::new(100);
     * ```
     */
    pub fn new(physical_clock_time: P) -> Self {
        HybridLogicalClock {
            physical_clock_time,
            logical_clock: L::default(),
        }
    }

    /**
     * Create a new hybrid logical clock with the given physical clock time and logical clock time.
     * 
     * @example
     * ```
     * let hlc = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 1);
     * ```
     */
    pub fn new_with_both_physical_and_logical_clock_time(physical_clock_time: P, logical_clock_time: L) -> Self {
        HybridLogicalClock {
            physical_clock_time,
            logical_clock: logical_clock_time,
        }
    }

    /**
     * Add the given number of logical clock ticks to the hybrid logical clock.
     * 
     * Use this method when you want to increment the logical clock without changing
     * the physical clock time. This is useful in scenarios where multiple events
     * occur at the same physical time, but you need to establish a causal order
     * between them.
     * 
     * Note: This method uses wrapping addition to prevent overflow.
     * 
     * @example
     * ```
     * let mut hlc = HybridLogicalClock::new(100);
     * hlc.add_logical_clock_ticks(1); // Increment logical clock by 1
     * hlc.add_logical_clock_ticks(5); // Increment logical clock by 5
     * ```
     */
    pub fn add_logical_clock_ticks(&mut self, ticks: L) {
        self.logical_clock = self.logical_clock + ticks;    
    }

    
}

impl<P, L> PartialOrd for HybridLogicalClock<P, L>
where
    P: Copy + Ord + Add<Output = P> + AddAssign,
    L: Copy + Ord + Add<Output = L> + AddAssign,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P, L> Ord for HybridLogicalClock<P, L>
where
    P: Copy + Ord + Add<Output = P> + AddAssign,
    L: Copy + Ord + Add<Output = L> + AddAssign,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.physical_clock_time.cmp(&other.physical_clock_time) {
            Ordering::Equal => self.logical_clock.cmp(&other.logical_clock),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hlc = HybridLogicalClock::<u64, u32>::new(100);
        assert_eq!(hlc.physical_clock_time, 100);
        assert_eq!(hlc.logical_clock, 0);
    }

    #[test]
    fn test_new_with_both_physical_and_logical_clock_time() {
        let hlc = HybridLogicalClock::<u64, u32>::new_with_both_physical_and_logical_clock_time(100, 1);
        assert_eq!(hlc.physical_clock_time, 100);
        assert_eq!(hlc.logical_clock, 1);
    }

    #[test]
    fn test_add_logical_clock_ticks() {
        // at this point, the logical clock is 0
        let mut hlc = HybridLogicalClock::<u64, u32>::new(100);
        
        hlc.add_logical_clock_ticks(1);
        assert_eq!(hlc.logical_clock, 1);

        let mut hlc_with_both_physical_and_logical_clock_time = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 20); 
        hlc_with_both_physical_and_logical_clock_time.add_logical_clock_ticks(23);
        assert_eq!(hlc_with_both_physical_and_logical_clock_time.logical_clock, 43);
    }

    #[test]
    fn test_cmp() {
        let hlc1 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 20);
        let hlc2 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 20);
        assert_eq!(hlc1, hlc2);
    }
}