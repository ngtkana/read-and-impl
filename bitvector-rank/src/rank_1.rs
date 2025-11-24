pub struct Rank1 {
    sum: Vec<u64>,
}
impl Rank1 {
    pub fn rank(&self, index: usize) -> usize {
        self.sum[index] as usize
    }
}

impl crate::test_utils::RankDataStructure for Rank1 {
    fn rank(&self, index: usize) -> usize {
        self.rank(index)
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
    use crate::test_utils::test_rank_implementation;

    #[test]
    fn test_rank1() {
        test_rank_implementation::<Rank1>();
    }
}
