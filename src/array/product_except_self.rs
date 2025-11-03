pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![1; nums.len()];

    let mut pre = 1;
    for i in 0..nums.len() {
        res[i] = pre;
        pre *= nums[i]
    }

    let mut post = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= post;
        post *= nums[i]
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(vec![24, 12, 8, 6], product_except_self(input))
    }

    #[test]
    fn example_two() {
        let input = vec![-1, 1, 0, -3, 3];
        assert_eq!(vec![0, 0, 9, 0, 0], product_except_self(input))
    }
}
