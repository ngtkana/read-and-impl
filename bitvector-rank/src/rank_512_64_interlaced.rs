pub struct Rank51264Interlaced {
    len: usize,
    words: Vec<u64>,
}
impl Rank51264Interlaced {
    pub fn rank(&self, index: usize) -> usize {
        assert!(index <= self.len);
        let a = index / 512;
        let b = index % 512 / 64;
        let c = index % 64;
        let sum = self.words[a * 10];
        let packed = self.words[a * 10 + 1];
        let word = self.words[a * 10 + 2 + b];
        let ans =
            sum + if b == 0 {
                0
            } else {
                packed >> ((b - 1) * 9) & 511
            } + u64::from((word & ((1 << c) - 1)).count_ones());
        ans as usize
    }
}

impl FromIterator<bool> for Rank51264Interlaced {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut len = 0usize;
        let mut word = 0u64;
        let mut words = vec![0, 0];
        let mut a = 0;
        let mut b = 0;
        let mut sum = 0u32;
        for elm in iter {
            word |= u64::from(elm) << (len % 64);
            len += 1;
            if len.is_multiple_of(64) {
                sum += word.count_ones();
                words.push(std::mem::take(&mut word));
                if len.is_multiple_of(512) {
                    words.push(words[a * 10] + u64::from(sum));
                    words.push(0);
                    a += 1;
                    b = 0;
                    sum = 0;
                } else {
                    words[a * 10 + 1] |= u64::from(sum) << (b * 9);
                    b += 1;
                }
            }
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
            let bvec: Rank51264Interlaced = a.iter().copied().collect();
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
