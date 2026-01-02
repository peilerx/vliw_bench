# VLIW-style Summation Experiment (Rust)

A deep dive into Instruction Level Parallelism (ILP) mechanisms and SIMD vectorization efficiency when facing the "Memory Wall."

## Experiment Overview
This benchmark compares two different approaches to summing an array of 1,000,000 `f32` elements (approx. 4 MB):
1. **Native**: Utilizing standard Rust iterators. Due to strict data dependencies, the CPU is forced to execute additions sequentially (Scalar Addition).
2. **VLIW-style**: Manually unrolling the loop into 4 independent accumulators. This "hints" the compiler to use SIMD instructions (`vaddps`) and saturate multiple execution ports simultaneously.

## Hardware Specifications
- **CPU**: Intel Core i5-7200U (Skylake-U) @ 2.50GHz
- **Architecture**: x86_64 (AVX2, FMA support)
- **L3 Cache**: 3 MB (The 4 MB test set intentionally exceeds L3 to trigger RAM access)
- **OS**: Linux
- **Compiler**: rustc 1.92.0

## Performance Results (1,000,000 elements)

| Method | Mean Time | Throughput (approx) |
| :--- | :--- | :--- |
| **Native** | 11.857 ms | ~337 MB/s |
| **VLIW-style (4x Acc)** | 3.447 ms | ~1.16 GB/s |

**Analysis**: We achieved a **3.44x speedup**. Despite spilling into RAM, the parallelized data access pattern allows the Hardware Prefetcher to utilize memory bandwidth more effectively compared to the scalar approach.

## How to Run

To reproduce these results with maximum hardware-specific optimizations, use the following flags:

```bash
RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench
