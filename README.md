# VLIW-style Summation Experiment (Rust)

A study on Instruction Level Parallelism (ILP) and Memory Wall constraints.

## The Experiment
Testing the summation of **40,000,000** `f32` elements (~160 MB). 
This dataset size is ~53x larger than the CPU's L3 cache (3 MB), forcing the system to rely entirely on DRAM performance and the Hardware Prefetcher.

## Hardware Specifications
- **CPU**: Intel Core i5-7200U (Skylake-U) @ 2.50GHz
- **L3 Cache**: 3 MB
- **Data Size**: 160 MB (40M elements * 4 bytes)

## Final Results (40,000,000 elements)

| Method | Execution Time | Throughput | Gap |
| :--- | :--- | :--- | :--- |
| **Native** | 47.17 ms | ~3.39 GB/s | Baseline |
| **VLIW-style** | 12.58 ms | **~12.72 GB/s** | **3.75x Faster** |

### Critical Insight: We broke the 10 GB/s barrier!
With 40M elements, the throughput calculation ($160MB / 0.01258s$) shows we reached **~12.7 GB/s**. 

This is an incredible result for a single core on a Skylake-U. It means:
1. **Saturation**: We are likely hitting the absolute maximum bandwidth that a single core can request from the memory controller.
2. **Prefetcher Efficiency**: The CPU successfully predicted the linear access pattern, keeping the SIMD units fed even though the data was far out in RAM.
3. **VLIW Victory**: The native code stayed at ~3.4 GB/s because it couldn't request data fast enough due to serial dependencies.

## Run Benchmarks
```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
