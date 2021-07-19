use criterion::criterion_main;

mod raytracing;

criterion_main! {
    raytracing::benches,
}
