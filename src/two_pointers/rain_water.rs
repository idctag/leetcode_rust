pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }
    let (mut l, mut r, mut res) = (0, height.len() - 1, 0);
    let (mut l_max, mut r_max) = (height[l], height[r]);
    while l < r {
        if l_max < r_max {
            l += 1;
            l_max = l_max.max(height[l]);
            res += l_max - height[l];
        } else {
            r -= 1;
            r_max = r_max.max(height[r]);
            res += r_max - height[r]
        }
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(input), 6)
    }
    #[test]
    fn two() {
        let input = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(input), 9)
    }
}
