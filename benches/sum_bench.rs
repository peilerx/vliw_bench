use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn native_sum(data: &[f32]) -> f32 {
    data.iter().sum()
}

pub fn native_ilp_sum(data: &[f32]) -> f32 {
    let mut acc0 = 0.0;
    let mut acc1 = 0.0;
    let mut acc2 = 0.0;
    let mut acc3 = 0.0;
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();
    for chunk in chunks {
        acc0 += chunk[0];
        acc1 += chunk[1];
        acc2 += chunk[2];
        acc3 += chunk[3];
    }
    let mut sum = acc0 + acc1 + acc2 + acc3;
    for &x in remainder {
        sum += x;
    }
    sum
}

#[inline(never)]
pub fn native_divide_f32(data: &[f32], divisor: f32) -> f32 {
    let mut sum = 0.0;
    for &x in data {
        sum += x / divisor;
    }
    sum
}

#[inline(never)]
pub fn prepped_divide_f32(data: &[f32], divisor: f32) -> f32 {
    let mut acc0 = 0.0;
    let mut acc1 = 0.0;
    let mut acc2 = 0.0;
    let mut acc3 = 0.0;

    for chunk in data.chunks_exact(4) {
        acc0 += chunk[0] / divisor;
        acc1 += chunk[1] / divisor;
        acc2 += chunk[2] / divisor;
        acc3 += chunk[3] / divisor;
    }

    acc0 + acc1 + acc2 + acc3
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 40_000_000;
    let data = vec![1.1f32; size];

    let mut group_sum = c.benchmark_group("Summing_f32");
    group_sum.bench_function("native", |b| b.iter(|| native_sum(black_box(&data))));
    group_sum.bench_function("native-ilp-style", |b| {
        b.iter(|| native_ilp_sum(black_box(&data)))
    });
    group_sum.finish();

    let size_div = 10_000_000;
    let data_div = vec![100.0f32; size_div];
    let divisor = black_box(7.0f32);

    let mut group_div = c.benchmark_group("Division_f32");

    group_div.bench_function("native-divide-f32", |b| {
        b.iter(|| native_divide_f32(black_box(&data_div), divisor))
    });

    group_div.bench_function("prepped-divide-f32-ilp", |b| {
        b.iter(|| prepped_divide_f32(black_box(&data_div), divisor))
    });

    group_div.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
