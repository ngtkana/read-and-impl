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

impl crate::test_utils::RankDataStructure for Rank25664 {
    fn rank(&self, index: usize) -> usize {
        self.rank(index)
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
    use crate::test_utils::test_rank_implementation;

    #[test]
    fn test_rank25664() {
        test_rank_implementation::<Rank25664>();
    }
}
