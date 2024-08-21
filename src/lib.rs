#![no_std]

use core::cmp::{max, Ordering};

/// Represents a Hybrid Logical Clock (HLC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridLogicalClock {
    pub physical: u64,
    pub logical: u32,
}

impl HybridLogicalClock {
    /// Creates a new HybridLogicalClock with the given physical time.
    /// The logical time is initialized to 0.
    pub fn new(physical: u64) -> Self {
        Self {
            physical,
            logical: 0,
        }
    }

    /// Creates a new HybridLogicalClock with both physical and logical components.
    pub fn new_with_both_physical_and_logical_clock_time(physical: u64, logical: u32) -> Self {
        Self { physical, logical }
    }

    /// Updates the clock based on a received timestamp.
    pub fn update(&mut self, received: &Self, now: u64) {
        self.physical = max(max(self.physical, received.physical), now);
        if self.physical == received.physical && self.physical == now {
            self.logical = max(self.logical, received.logical) + 1;
        } else if self.physical == received.physical {
            self.logical = max(self.logical, received.logical) + 1;
        } else if self.physical == now {
            self.logical += 1;
        } else {
            self.logical = 0;
        }
    }

    /// Checks if this clock is concurrent with another clock.
    pub fn is_concurrent(&self, other: &Self) -> bool {
        self.physical == other.physical && self.logical != other.logical
    }

}

impl PartialOrd for HybridLogicalClock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HybridLogicalClock {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.physical.cmp(&other.physical) {
            Ordering::Equal => self.logical.cmp(&other.logical),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hlc = HybridLogicalClock::new(100);
        assert_eq!(hlc.physical, 100);
        assert_eq!(hlc.logical, 0);
    }

    #[test]
    fn test_new_with_both() {
        let hlc = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 50);
        assert_eq!(hlc.physical, 100);
        assert_eq!(hlc.logical, 50);
    }

    #[test]
    fn test_update() {
        let mut hlc1 = HybridLogicalClock::new(100);
        let hlc2 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(150, 10);
        hlc1.update(&hlc2, 200);
        assert_eq!(hlc1.physical, 200);
        assert_eq!(hlc1.logical, 1);
    }

    #[test]
    fn test_is_concurrent() {
        let hlc1 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 5);
        let hlc2 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 10);
        assert!(hlc1.is_concurrent(&hlc2));
    }

    #[test]
    fn test_ordering() {
        let hlc1 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 5);
        let hlc2 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 10);
        let hlc3 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(150, 0);
        assert!(hlc1 < hlc2);
        assert!(hlc2 < hlc3);
    }
}
