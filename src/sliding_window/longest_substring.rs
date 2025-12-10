pub fn solution(s: String) -> i32 {
    use std::collections::HashMap;

    let mut char_index: HashMap<char, i32> = HashMap::new();
    let mut max_len = 0;
    let mut left = 0;

    for (ridx, rv) in s.chars().enumerate() {
        if let Some(&prev_idx) = char_index.get(&rv) {
            if prev_idx >= left {
                left = prev_idx + 1
            }
        }
        char_index.insert(rv, ridx as i32);
        max_len = max_len.max(ridx as i32 - left + 1)
    }
    max_len as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(solution("abcabcbb".to_string()), 3)
    }
    #[test]
    fn two() {
        assert_eq!(solution("bbbbb".to_string()), 1)
    }
    #[test]
    fn three() {
        assert_eq!(solution("pwwkew".to_string()), 3)
    }
    #[test]
    fn four() {
        assert_eq!(solution("au".to_string()), 2)
    }
    #[test]
    fn five() {
        assert_eq!(solution("aa".to_string()), 1)
    }
    #[test]
    fn six() {
        assert_eq!(solution("dvdf".to_string()), 3)
    }
}
