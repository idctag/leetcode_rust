pub fn solution(_s: String) -> i32 {
    // use std::collections::HashSet;
    // if s.len() <= 1 {
    //     return s.len() as i32;
    // }
    // let c_vec = s.chars();
    // let mut l = 0;
    // let mut curr_collection: HashSet<char> = HashSet::new();
    // for r in 1..s.len() {}
    0
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
}
