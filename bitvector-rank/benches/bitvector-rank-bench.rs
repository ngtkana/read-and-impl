mod common;

use bitvector_rank::{Rank1, Rank64, Rank25664, Rank25664Interlaced, Rank51264Interlaced};
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
    bench_construct::<Rank25664Interlaced>(&mut group, "Rank25664Interlaced", &a);
    bench_construct::<Rank51264Interlaced>(&mut group, "Rank51264Interlaced", &a);

    group.finish();
}

fn bench_bitvector_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitvector Rank");
    let mut rng = StdRng::seed_from_u64(42);
    let TestCase { a, queries } = TestCase::generate(&mut rng);

    let rank1: Rank1 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank1", &rank1, &queries);

    let rank64: Rank64 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank64", &rank64, &queries);

    let rank25664: Rank25664 = a.iter().copied().collect();
    bench_rank(&mut group, "Rank25664", &rank25664, &queries);

    let rank25664interlaced: Rank25664Interlaced = a.iter().copied().collect();
    bench_rank(
        &mut group,
        "Rank25664Interlaced",
        &rank25664interlaced,
        &queries,
    );

    let rank51264interlaced: Rank51264Interlaced = a.iter().copied().collect();
    bench_rank(
        &mut group,
        "Rank51264Interlaced",
        &rank51264interlaced,
        &queries,
    );

    group.finish();
}

criterion_group!(benches, bench_bitvector_construct, bench_bitvector_rank);
criterion_main!(benches);
