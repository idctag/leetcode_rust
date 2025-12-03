pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for t in tokens {
        match t.as_str() {
            "+" | "/" | "-" | "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(match t.as_str() {
                    "*" => b * a,
                    "-" => b - a,
                    "+" => b + a,
                    "/" => b / a,
                    _ => unreachable!(),
                });
            }
            _ => stack.push(t.parse().unwrap()),
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = vec!["2", "1", "+", "3", "*"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(eval_rpn(input), 9)
    }
    #[test]
    fn two() {
        let input = vec!["4", "13", "5", "/", "+"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(eval_rpn(input), 6)
    }
    #[test]
    fn three() {
        let input = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        assert_eq!(eval_rpn(input), 22)
    }
}
