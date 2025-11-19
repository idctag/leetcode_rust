pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r, mut max) = (0, height.len() - 1, 0);
    while l < r {
        let w = (r - l) as i32;
        let h = height[l].min(height[r]);
        let area = w * h;
        max = max.max(area);
        if height[l] < height[r] {
            l += 1
        } else {
            r -= 1
        }
    }
    max as i32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }
    #[test]
    fn two() {
        assert_eq!(max_area(vec![1, 1]), 1)
    }
}
