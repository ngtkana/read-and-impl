use rand::{Rng, SeedableRng, rngs::StdRng};

pub trait RankDataStructure: FromIterator<bool> {
    fn rank(&self, index: usize) -> usize;
}

pub fn test_rank_implementation<T: RankDataStructure>() {
    let mut rng = StdRng::seed_from_u64(42);
    for tid in 1..=100 {
        let mut n = rng.random_range(0..=3000);
        if rng.random_ratio(1, 2) {
            n = n / 64 * 64;
        }
        eprintln!("Testcase #{tid}: n = {n}");
        let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
            .take(n)
            .collect();
        let bvec: T = a.iter().copied().collect();
        for qid in 1..=200 {
            let index = rng.random_range(0..=n);
            eprintln!("Query #{tid}.{qid}: rank({index})");
            let expected = a.iter().take(index).filter(|&&b| b).count();
            let result = bvec.rank(index);
            assert_eq!(result, expected);
        }
    }
}
