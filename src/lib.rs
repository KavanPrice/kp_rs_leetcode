/*
    ======================
             Lib
    ======================
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..=nums.len() - 1 {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

/*
    ======================
            Tests
    ======================
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(vec![3, 2, 4], 6), Some((1, 2)));
        assert_eq!(two_sum(vec![3, 3], 6), Some((0, 1)));
    }
}
