use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rust_simd::{N, add};

fn bench_add(c: &mut Criterion) {
    let a: Vec<f32> = (0..N).map(|i| i as f32 * 0.5).collect();
    let b: Vec<f32> = (0..N).map(|i| (i as f32).sin()).collect();
    let mut out = vec![0.0; N];

    // 2 inputs + 1 output, f32 each
    let bytes = (N * 3 * std::mem::size_of::<f32>()) as u64;

    let mut group = c.benchmark_group("add");
    group.throughput(Throughput::Bytes(bytes));
    group.bench_with_input(BenchmarkId::from_parameter(N), &(), |bencher, _| {
        bencher.iter(|| {
            add(black_box(&a), black_box(&b), black_box(&mut out));
        });
    });
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);
