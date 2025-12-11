pub fn solution(heights: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack: Vec<i32> = Vec::new();
    for (i, h) in heights.into_iter().enumerate() {}
    ans
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn case_one() {
        let input = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(solution(input), 10)
    }
    #[test]
    fn case_two() {
        let input = vec![2, 4];
        assert_eq!(solution(input), 4)
    }
}
