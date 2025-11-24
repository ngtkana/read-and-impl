pub struct Rank25664Interlaced {
    len: usize,
    words: Vec<u64>,
}
impl Rank25664Interlaced {
    pub fn rank(&self, index: usize) -> usize {
        assert!(index <= self.len);
        let a = index / 256;
        let b = index % 256 / 64;
        let c = index % 64;
        let sum = self.words[a * 6];
        let packed = self.words[a * 6 + 1];
        let word = self.words[a * 6 + 2 + b];
        let ans = sum + (packed >> (b * 8) & 255) + u64::from((word & ((1 << c) - 1)).count_ones());
        ans as usize
    }
}

impl FromIterator<bool> for Rank25664Interlaced {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut len = 0usize;
        let mut word = 0u64;
        let mut words = vec![0, 0];
        let mut lsum = 0u32;
        for elm in iter {
            word |= u64::from(elm) << (len % 64);
            if (len + 1).is_multiple_of(64) {
                lsum += word.count_ones();
                words.push(std::mem::take(&mut word));
                let a = len / 256;
                let b = len % 256 / 64;
                if (len + 1).is_multiple_of(256) {
                    words.push(words[a * 6] + u64::from(lsum));
                    words.push(0);
                    lsum = 0;
                } else {
                    words[a * 6 + 1] |= u64::from(lsum) << ((b + 1) * 8);
                }
            }
            len += 1;
        }
        words.push(word);
        Self { len, words }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn test_rank25664() {
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
            let bvec: Rank25664Interlaced = a.iter().copied().collect();
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
