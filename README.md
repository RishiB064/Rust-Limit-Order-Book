# Rust Limit Order Book Engine
A high-performance, single-threaded matching engine built in Rust. Designed to simulate the microstructure of a high-frequency trading (HFT) exchange with nanosecond-level latency.

##  Performance Benchmarks
Benchmarked on [Apple MacBook Pro |Intel i9]:

| **Throughput** | ~7,100,000 Orders/Sec
| **Latency** | ~141 nanoseconds(avg) |
| **Complexity** | $O(\log N)$ using B-Tree Maps |

##  Logic

### 1. Fixed-Point Arithmetic (`u64` vs `f64`)
I explicitly chose **not** to use Floating Point math (`f64`) for price calculations.
- **Problem:** IEEE 754 floats introduce rounding errors (e.g., `0.1 + 0.2 != 0.3`), creating "Ghost Money" in financial audits.
- **Solution:** Prices are stored as `u64` (integers) representing atomic units (e.g., cents or paise). This ensures **zero-loss precision** and regulatory compliance.

### 2. Data Structures (`BTreeMap`)
- Replaced standard Vectors ($O(N \log N)$ sorting) with `BTreeMap` ($O(\log N)$ insertion).
- This keeps the order book permanently sorted, allowing the engine to "peek" at the Best Bid/Ask in $O(1)$ time.

### 3. Memory Layout
- Utilized `VecDeque` for order queues to enforce strictly FIFO (First-In-First-Out) Time Priority matching without expensive memory shifting.

##  Usage
To run the benchmark with release optimizations:
```bash
cargo run --release
