pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut res = Vec::new();
    for n in &nums {
        set.insert(n);
    }
    for i in 1..=nums.len() {
        if !set.contains(&(i as i32)) {
            res.push(i as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![5, 6]);
    }

    #[test]
    fn two() {
        let nums = vec![1, 1];
        assert_eq!(find_disappeared_numbers(nums), vec![2]);
    }
}
