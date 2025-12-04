pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let n = temperatures.len();
    let mut res = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();
    for (i, &t) in temperatures.iter().enumerate() {
        while let Some(&prev) = stack.last() {
            if t <= temperatures[prev] {
                break;
            }
            stack.pop();
            res[prev] = (i - prev) as i32
        }
        stack.push(i);
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let res = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(daily_temperatures(input), res)
    }
    #[test]
    fn two() {
        let input = vec![30, 40, 50, 60];
        let res = vec![1, 1, 1, 0];
        assert_eq!(daily_temperatures(input), res)
    }
    #[test]
    fn three() {
        let input = vec![30, 60, 90];
        let res = vec![1, 1, 0];
        assert_eq!(daily_temperatures(input), res)
    }
}
