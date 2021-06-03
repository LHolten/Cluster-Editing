use cluster_lib::disk::load;

use cluster_lib::search::Solver;
use criterion::{criterion_group, criterion_main, SamplingMode};

use std::fs::File;

use criterion::BenchmarkId;
use criterion::Criterion;

fn exact_track(c: &mut Criterion) {
    let mut group = c.benchmark_group("exact");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    for instance in (1..=11).step_by(2) {
        let input = load(File::open(format!("../exact/exact{:03}.gr", instance)).unwrap()).unwrap();
        let solver = Solver::new(input);
        group.bench_with_input(BenchmarkId::from_parameter(instance), &solver, |b, g| {
            b.iter_batched_ref(
                || g.clone(),
                |s| s.search_components(),
                criterion::BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, exact_track);
criterion_main!(benches);
