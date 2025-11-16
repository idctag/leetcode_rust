pub fn is_palindrome(s: String) -> bool {
    let string: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    string.chars().eq(string.chars().rev())
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn ex_one() {
        let input = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(is_palindrome(input), true)
    }
    #[test]
    fn ex_two() {
        let input = "race a car".to_string();
        assert_eq!(is_palindrome(input), false)
    }
    #[test]
    fn ex_three() {
        let input = " ".to_string();
        assert_eq!(is_palindrome(input), true)
    }
    #[test]
    fn ex_four() {
        let input = "0P".to_string();
        assert_eq!(is_palindrome(input), false)
    }
}
