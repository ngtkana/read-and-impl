pub struct Rank1 {
    sum: Vec<u64>,
}
impl Rank1 {
    pub fn rank(&self, index: usize) -> usize {
        self.sum[index] as usize
    }
}

impl FromIterator<bool> for Rank1 {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut x = 0;
        let mut sum = vec![];
        for b in iter {
            sum.push(x);
            x += u64::from(b);
        }
        sum.push(x);
        Self { sum }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_rank25664() {
        let mut rng = StdRng::seed_from_u64(42);
        for tid in 1..=20 {
            let mut n = rng.random_range(0..=1000);
            if rng.random_ratio(1, 2) {
                n = n / 64 * 64;
            }
            eprintln!("Testcase #{tid}: n = {n}");
            let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
                .take(n)
                .collect();
            let bvec: Rank1 = a.iter().copied().collect();
            for qid in 1..=200 {
                let index = rng.random_range(0..=n);
                eprintln!("Query #{tid}.{qid}: rank({index})");
                let expected = a.iter().take(index).filter(|&&b| b).count();
                let result = bvec.rank(index);
                assert_eq!(result, expected);
            }
        }
    }
}
