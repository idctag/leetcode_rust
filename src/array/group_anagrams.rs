pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    const LEN: usize = 26;
    let mut map: HashMap<[u8; LEN], Vec<String>> = HashMap::with_capacity(strs.len());
    let offset = 'a' as usize;
    for str in strs {
        let mut chars: [u8; LEN] = [0; LEN];

        for char in str.chars() {
            chars[char as usize - offset] += 1
        }

        map.entry(chars)
            .and_modify(|v| v.push(str.clone()))
            .or_insert(vec![str]);
    }
    map.into_values().collect()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let res: Vec<Vec<String>> = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_eq!(res, group_anagrams(input))
    }

    #[test]
    fn example_two() {
        let input = vec!["".to_string()];
        let res = vec![vec!["".to_string()]];
        assert_eq!(res, group_anagrams(input))
    }

    #[test]
    fn example_three() {
        let input = vec!["a".to_string()];
        let res = vec![vec!["a".to_string()]];
        assert_eq!(res, group_anagrams(input))
    }
}
