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

impl crate::test_utils::RankDataStructure for Rank51264Interlaced {
    fn rank(&self, index: usize) -> usize {
        self.rank(index)
    }
}

impl FromIterator<bool> for Rank51264Interlaced {
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
                let a = len / 512;
                let b = len % 512 / 64;
                if (len + 1).is_multiple_of(512) {
                    words.push(words[a * 10] + u64::from(lsum));
                    words.push(0);
                    lsum = 0;
                } else {
                    words[a * 10 + 1] |= u64::from(lsum) << (b * 9);
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
    use crate::test_utils::test_rank_implementation;

    #[test]
    fn test_rank51264_interlaced() {
        test_rank_implementation::<Rank51264Interlaced>();
    }
}
