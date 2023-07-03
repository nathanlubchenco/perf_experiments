use criterion::{black_box, criterion_group, criterion_main, Criterion};

use perf_experiments::cli;
use perf_experiments::load;
use perf_experiments::find;

fn criterion_benchmark(c: &mut Criterion) {
    let target_vecs = load::load_vectors("target_vec.json");
    let search_space = load::load_vectors("vectors.json");

    c.bench_function("heap 1", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &2,  &cli::NnImpls::Heap  ))));

    c.bench_function("parallel sort 1", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &2,  &cli::NnImpls::ParallelSort  ))));

}

fn custom_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(std::time::Duration::new(20, 0)) // increase target time to allow for more samples of slow methods
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = criterion_benchmark
}
criterion_main!(benches);
