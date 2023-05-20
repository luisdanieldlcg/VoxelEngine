use criterion::{black_box, criterion_group, criterion_main, Criterion};
use voxgen::renderer::WorldRenderer;
use voxgen::world::chunk::{Chunk, ChunkPos};

fn meshgen_benchmark(c: &mut Criterion) {
    c.bench_function("meshgen", |b| {
        b.iter(|| {
            let chunk = Chunk::generate(&ChunkPos::new(0, 0));
        })
    });
}

criterion_group!(benches, meshgen_benchmark);
criterion_main!(benches);
