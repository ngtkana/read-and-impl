pub struct Rank25664 {
    len: usize,
    words: Vec<u64>,
    block: Vec<u8>,
    sblock: Vec<u64>,
}
impl Rank25664 {
    pub fn rank(&self, index: usize) -> usize {
        assert!(index <= self.len);
        let ans = self.sblock[index / 256]
            + u64::from(self.block[index / 64])
            + u64::from((self.words[index / 64] & ((1 << (index % 64)) - 1)).count_ones());
        ans as usize
    }
}

impl FromIterator<bool> for Rank25664 {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut len = 0;
        let mut word = 0u64;
        let mut words = vec![];
        let mut block = vec![0];
        let mut sblock = vec![0];
        let mut sum = 0u8;
        let mut ssum = 0u64;
        for b in iter {
            word |= u64::from(b) << (len % 64);
            len += 1;
            if len % 64 == 0 {
                let pc = word.count_ones();
                words.push(std::mem::take(&mut word));
                sum += pc as u8;
                ssum += u64::from(pc);
                if len % 256 == 0 {
                    sblock.push(ssum);
                    sum = 0;
                }
                block.push(sum);
            }
        }
        words.push(word);
        Self {
            len,
            words,
            block,
            sblock,
        }
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
            let mut n = rng.random_range(0..=1000);
            if rng.random_ratio(1, 2) {
                n = n / 64 * 64;
            }
            eprintln!("Testcase #{tid}: n = {n}");
            let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
                .take(n)
                .collect();
            let bvec: Rank25664 = a.iter().copied().collect();
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
