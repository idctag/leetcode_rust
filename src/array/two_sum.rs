//https://leetcode.com/problems/two-sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut values: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match values.get(&(target - v)) {
            Some(index) => return vec![*index, i as i32],
            None => values.insert(*v, i as i32),
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![0, 1], two_sum(nums, target))
    }
    #[test]
    fn test_two() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(vec![1, 2], two_sum(nums, target))
    }
    #[test]
    fn test_three() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(vec![0, 1], two_sum(nums, target))
    }
}
