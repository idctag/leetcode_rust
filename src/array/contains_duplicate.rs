// https://leetcode.com/problems/contains-duplicate/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort();
    for i in 1..nums.len() as usize {
        if nums[i - 1] == nums[i] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate_true() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn test_contains_duplicate_false() {
        let nums = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums));
    }

    #[test]
    fn test_contains_duplicate_empty() {
        let nums = vec![];
        assert!(!contains_duplicate(nums));
    }
}
