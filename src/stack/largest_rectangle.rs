pub fn solution(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let len: i32 = heights.len() as i32;
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for (i, h) in heights.into_iter().enumerate() {
        if stack.is_empty() {
            stack.push((i as i32, h));
        }
        let top = stack.last().unwrap();
        if h < top.1 {
        } else {
        }
    }
    for p in &stack {
        let area = ((len - p.0) + 1) * p.1;
        max_area = max_area.max(area)
    }
    max_area
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
