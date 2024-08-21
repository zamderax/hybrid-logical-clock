#![no_std]

use core::cmp::{max, Ordering};

/// Represents a Hybrid Logical Clock (HLC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridLogicalClock {
    /// The physical component of the clock, often referred to as the "wallclock" time.
    pub physical: u64,
    /// The logical component of the clock, used to distinguish events occurring at the same physical time.
    pub logical: u32,
}

impl HybridLogicalClock {
    /// Creates a new HybridLogicalClock with the given physical time.
    /// The logical time is initialized to 0.
    ///
    /// # Arguments
    ///
    /// * `physical` - The physical time.
    ///
    /// # Returns
    ///
    /// A new HybridLogicalClock with the given physical time and logical time set to 0.
    ///
    /// # Example
    ///
    /// ```
    /// use hybrid_logical_clock::HybridLogicalClock;
    ///
    /// let hlc = HybridLogicalClock::new(1000);
    /// assert_eq!(hlc.physical, 1000);
    /// assert_eq!(hlc.logical, 0);
    /// ```
    pub fn new(physical: u64) -> Self {
        Self {
            physical,
            logical: 0,
        }
    }

    /// Creates a new HybridLogicalClock with both physical and logical components.
    /// 
    /// # Arguments
    /// 
    /// * `physical` - The physical time.
    /// * `logical` - The logical time.
    /// 
    /// # Returns
    /// 
    /// A new HybridLogicalClock with the given physical and logical times.
    /// 
    /// # Example
    /// 
    /// ```
    /// use hybrid_logical_clock::HybridLogicalClock;
    /// 
    /// let hlc = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(1000, 5);
    /// assert_eq!(hlc.physical, 1000);
    /// assert_eq!(hlc.logical, 5);
    /// ```
    pub fn new_with_both_physical_and_logical_clock_time(physical: u64, logical: u32) -> Self {
        Self { physical, logical }
    }

    /// Updates the clock based on a received timestamp.
    /// 
    /// # Arguments
    /// 
    /// * `received` - The received HybridLogicalClock.
    /// * `now` - The current physical time.
    /// 
    /// # Example
    /// 
    /// ```
    /// use hybrid_logical_clock::HybridLogicalClock;
    /// 
    /// let mut hlc = HybridLogicalClock::new(100);
    /// let received = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(150, 10);
    /// hlc.update(&received, 200);
    /// 
    /// assert_eq!(hlc.physical, 200);
    /// assert_eq!(hlc.logical, 1);
    /// ```
    pub fn update(&mut self, received: &Self, now: u64) {
        // Update the physical time to the maximum of the current physical time, the received physical time, and the current time.
        self.physical = max(max(self.physical, received.physical), now);
        // If the physical time is the same as the received physical time and the current time, update the logical time.
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

    /// Checks if this clock is concurrent with another hybrid logical clock.
    /// 
    /// This method is crucial for determining the causal relationship between events in a distributed system.
    /// It's particularly useful when you need to detect and handle concurrent operations or updates.
    /// 
    /// Use this method when:
    /// 1. Implementing conflict resolution strategies in distributed databases or systems.
    /// 2. Detecting potential conflicts in collaborative editing systems.
    /// 3. Ensuring consistency in replicated state machines.
    /// 4. Implementing version control systems that need to handle parallel changes.
    /// 
    /// The importance of this method lies in its ability to identify events that happened without knowledge
    /// of each other. In distributed systems, this information is vital for maintaining consistency and
    /// correctly merging or reconciling divergent states.
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other HybridLogicalClock to compare with.
    /// 
    /// # Returns
    /// 
    /// `true` if the clocks are concurrent, `false` otherwise.
    /// 
    /// # Example
    /// 
    /// ```
    /// use hybrid_logical_clock::HybridLogicalClock;
    /// 
    /// let hlc1 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 5);
    /// let hlc2 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 10);
    /// assert!(hlc1.is_concurrent(&hlc2));
    /// 
    /// // Non-concurrent example
    /// let hlc3 = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(101, 1);
    /// assert!(!hlc1.is_concurrent(&hlc3));
    /// ```
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
