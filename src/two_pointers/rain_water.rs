pub fn trap(height: Vec<i32>) -> i32 {
    let (mut l, mut r, mut total) = (0, height.len() - 1, 0);
    total
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
