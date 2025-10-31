//https://leetcode.com/problems/valid-anagram
pub fn is_anagram(s: String, t: String) -> bool {
    let mut count = vec![0; 255];

    s.chars().for_each(|c| count[c as usize] += 1);

    for c in t.chars() {
        count[c as usize] -= 1;
        if count[c as usize] < 0 {
            return false;
        }
    }

    !count.iter().any(|c| *c != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_valid() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert!(!is_anagram(s, t))
    }
    #[test]
    fn test_valid() {
        let s = "anagrma".to_string();
        let t = "nagaram".to_string();
        assert!(is_anagram(s, t))
    }
}
