use cluster_lib::disk::load;

use cluster_lib::search::Solver;
use criterion::{criterion_group, criterion_main, AxisScale, PlotConfiguration, SamplingMode};

use std::fs::File;

use criterion::BenchmarkId;
use criterion::Criterion;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn exact_track(c: &mut Criterion) {
    let mut group = c.benchmark_group("exact");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(10);
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    // group.plot_config(plot_config);
    let instances = vec![1, 3, 5, 7, 9, 11, 13, 15, 21, 23, 25, 31, 35, 41, 47];
    for instance in instances {
        let input = load(File::open(format!("../exact/exact{:03}.gr", instance)).unwrap()).unwrap();
        let solver = Solver::new(input);
        group.bench_with_input(BenchmarkId::from_parameter(instance), &solver, |b, s| {
            b.iter_batched_ref(
                || {
                    let mut new_s = s.clone();
                    new_s.graph.active.shuffle(&mut thread_rng());
                    new_s
                },
                |s| s.search_components(),
                criterion::BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, exact_track);
criterion_main!(benches);
