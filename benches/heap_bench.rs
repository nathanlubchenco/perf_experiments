use criterion::{black_box, criterion_group, criterion_main, Criterion};

use perf_experiments::data;
use perf_experiments::load;
use perf_experiments::find;

fn criterion_benchmark(c: &mut Criterion) {
    let target_vecs = load::load_vectors("target_vec.json");
    let search_space = load::load_vectors("vectors.json");

    c.bench_function("heap 10", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &10,  &data::NnImpls::Heap  ))));

    c.bench_function("heap 100", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &100,  &data::NnImpls::Heap  ))));

    c.bench_function("heap 1000", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &1000,  &data::NnImpls::Heap  ))));

    c.bench_function("heap 10000", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &10000,  &data::NnImpls::Heap  ))));

    c.bench_function("parallel sort 10", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &10,  &data::NnImpls::ParallelSort  ))));

    c.bench_function("parallel sort 100", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &100,  &data::NnImpls::ParallelSort  ))));

    c.bench_function("parallel sort 1000", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &1000,  &data::NnImpls::ParallelSort  ))));

    c.bench_function("parallel sort 10000", |b| b.iter(|| black_box(
        find::k_nearest(target_vecs.as_ref().unwrap().get(0).unwrap(), search_space.as_ref().unwrap(), &10000,  &data::NnImpls::ParallelSort  ))));

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
