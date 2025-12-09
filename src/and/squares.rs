pub fn solution(a: i32, b: i32) -> i32 {
    let mut count = 0;
    for n in a..=b {
        let squared: f64 = (n as f64).sqrt();
        if (squared as i32) * (squared as i32) == n {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(solution(3, 9), 2)
    }
    #[test]
    fn two() {
        assert_eq!(solution(17, 24), 0)
    }
}
