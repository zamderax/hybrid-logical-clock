# Hybrid Logical Clock in Rust

I built this crate to provide a hybrid logical clock in Rust so that I could learn about distributed systems. This is a work in progress and I will be adding more features to it in the future but it should be a good starting point for anyone looking to use a hybrid logical clock in their Rust project. This project is also ready for [no_std](https://docs.rust-embedded.org/book/intro/no-std.html) embedded environments.


## Why use a hybrid logical clock?

1. Physical clocks are not monotonic and unreliable in complex distributed systems. Each actor could be in a different geographic location, different network communcation health, and physical clock drift.
2. We can use logical clocks to order events in a distributed system, they have no relation to physical time. If you need to query data between multiple actors using physical time for debugging purposes, you can't if the system only uses logical clocks.

## How to use this crate

You can create a new hybrid logical clock with the physical clock time set to the given value.

```rs
use hybrid_logical_clock::HybridLogicalClock;

let hlc = HybridLogicalClock::new(100);
```

Sometimes you want to create a hybrid logical clock with both the physical and logical clock time set.

```rs
use hybrid_logical_clock::HybridLogicalClock;

let hlc = HybridLogicalClock::new_with_both_physical_and_logical_clock_time(100, 100);
```

You can compare two hybrid logical clocks to see if they are causally related.

```rs
use hybrid_logical_clock::HybridLogicalClock;

let hlc1 = HybridLogicalClock::new(100);
let hlc2 = HybridLogicalClock::new(200);

assert!(hlc1.is_concurrent(&hlc2));
```

## Why use a hybrid logical clock?

Most distributed systems use lamport or logical clocks to order events. However, these clocks have several drawbacks:

1. They do not provide causality tracking with ordering, meaning that two events can be causally related but not ordered. Ths is specifically important when building systems that have humans involved as humans seem to reach for physical time to determine ordering
2. They do not provide a total order, meaning that two events can be concurrent but not ordered.
3. They do not provide a way to detect conflicts, meaning that two events can be concurrent but not ordered.

Hybrid Logical Clocks (HLCs) offer several advantages over traditional logical clocks or physical clocks alone:

1. Causality tracking: HLCs combine the benefits of physical and logical clocks, allowing for accurate causality tracking between events in distributed systems.

2. Better performance: Unlike vector clocks, HLCs have constant-size timestamps, leading to improved performance in large-scale distributed systems.

3. Clock drift tolerance: HLCs can handle clock drift between nodes in a distributed system more gracefully than pure physical clocks.

4. Monotonicity: HLCs ensure that timestamps always move forward, even if there are small backwards jumps in the physical clock.

5. Fine-grained ordering: When multiple events occur at the same physical time, HLCs can still establish a total order using the logical component.

6. Compatibility: HLCs can be used as a drop-in replacement for physical timestamps in many systems, providing additional causality information without major architectural changes.

7. Conflict resolution: In distributed databases or collaborative editing systems, HLCs can help in detecting and resolving conflicts between concurrent updates.

By using a Hybrid Logical Clock, you can achieve a more robust and accurate timekeeping mechanism in distributed systems, leading to better consistency and easier reasoning about event ordering.
