pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = String::from("()");
        assert_eq!(is_valid(input), true)
    }
    #[test]
    fn two() {
        let input = String::from("[]{}()");
        assert_eq!(is_valid(input), true)
    }
    #[test]
    fn three() {
        let input = String::from("([])");
        assert_eq!(is_valid(input), true)
    }
    #[test]
    fn four() {
        let input = String::from("(]");
        assert_eq!(is_valid(input), false)
    }
    #[test]
    fn five() {
        let input = String::from("([)]");
        assert_eq!(is_valid(input), false)
    }
}
