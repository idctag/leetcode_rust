
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![1, -1];
    }
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        let sum = numbers[l] + numbers[r];
        if sum == target {
            return vec![(l + 1) as i32, (r + 1) as i32]
        }
        if sum > target {
            r = r.saturating_sub(1)
        }
        if sum < target {
            l = l+1
        }
    }

    return vec![1, -1]
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![1, 2])
    }
    #[test]
    fn two() {
        assert_eq!(two_sum(vec![2,3,4], 6), vec![1,3])
    }
    #[test]
    fn three() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2])
    }
}
