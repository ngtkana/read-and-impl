mod common;

use bitvector_rank::{
    Rank1, Rank64, Rank64Interlaced, Rank25664, Rank25664Interlaced, Rank51264Interlaced,
};
use common::{TestCase, bench_construct, bench_rank};
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{SeedableRng, rngs::StdRng};

fn bench_bitvector_construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitvector Construct");

    let mut rng = StdRng::seed_from_u64(42);
    let TestCase { a, queries: _ } = TestCase::generate(&mut rng);

    bench_construct::<Rank1>(&mut group, "Rank1", &a);
    bench_construct::<Rank64>(&mut group, "Rank64", &a);
    bench_construct::<Rank25664>(&mut group, "Rank25664", &a);
    bench_construct::<Rank64Interlaced>(&mut group, "Rank64Interlaced", &a);
    bench_construct::<Rank25664Interlaced>(&mut group, "Rank25664Interlaced", &a);
    bench_construct::<Rank51264Interlaced>(&mut group, "Rank51264Interlaced", &a);

    group.finish();
}

fn bench_bitvector_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitvector Rank");
    let mut rng = StdRng::seed_from_u64(42);
    let TestCase { a, queries } = TestCase::generate(&mut rng);

    let bvec: Rank1 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank1", &bvec, &queries);

    let bvec: Rank64 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank64", &bvec, &queries);

    let bvec: Rank25664 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank25664", &bvec, &queries);

    let bvec: Rank64Interlaced = a.iter().copied().collect();
    bench_rank(&mut group, "Rank64Interlaced", &bvec, &queries);

    let bvec: Rank25664Interlaced = a.iter().copied().collect();
    bench_rank(&mut group, "Rank25664Interlaced", &bvec, &queries);

    let bvec: Rank51264Interlaced = a.iter().copied().collect();
    bench_rank(&mut group, "Rank51264Interlaced", &bvec, &queries);

    group.finish();
}

criterion_group!(benches, bench_bitvector_construct, bench_bitvector_rank);
criterion_main!(benches);
