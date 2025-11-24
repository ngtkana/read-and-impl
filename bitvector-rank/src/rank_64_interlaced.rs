pub struct Rank64Interlaced {
    len: usize,
    words: Vec<u64>,
}
impl Rank64Interlaced {
    pub fn rank(&self, index: usize) -> usize {
        assert!(index <= self.len);
        let a = index / 64;
        let b = index % 64;
        let sum = self.words[a * 2];
        let word = self.words[a * 2 + 1];
        let ans = sum + u64::from((word & ((1 << b) - 1)).count_ones());
        ans as usize
    }
}

impl crate::test_utils::RankDataStructure for Rank64Interlaced {
    fn rank(&self, index: usize) -> usize {
        self.rank(index)
    }
}

impl FromIterator<bool> for Rank64Interlaced {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut len = 0usize;
        let mut word = 0u64;
        let mut words = vec![0u64];
        let mut sum = 0u64;
        for elm in iter {
            word |= u64::from(elm) << (len % 64);
            len += 1;
            if len.is_multiple_of(64) {
                sum += u64::from(word.count_ones());
                words.push(std::mem::take(&mut word));
                words.push(sum);
            }
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
    fn test_rank25664_interlaced() {
        test_rank_implementation::<Rank64Interlaced>();
    }
}
