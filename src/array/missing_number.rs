// https://leetcode.com/problems/contains-duplicate/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut sorted: Vec<i32> = nums;
    sorted.sort();

    if sorted[0] != 0 {
        return 0;
    }
    if sorted[sorted.len() - 1] != sorted.len() as i32 {
        return sorted.len() as i32;
    }
    for i in 0..sorted.len() {
        if sorted[i] != i as i32 {
            return i as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three() {
        let nums = vec![3, 0, 1];
        assert_eq!(missing_number(nums), 2);
    }

    #[test]
    fn two() {
        let nums = vec![0, 1];
        assert_eq!(missing_number(nums), 2);
    }

    #[test]
    fn one() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(missing_number(nums), 8);
    }
}
