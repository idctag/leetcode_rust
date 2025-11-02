pub fn top_k_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    nums.into_iter()
        .for_each(|n| *map.entry(n).or_insert(0) += 1);
    let mut v: Vec<(i32, i32)> = map.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v.iter().take(k as usize).map(|p| p.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(vec![1, 2], top_k_elements(input, 2))
    }

    #[test]
    fn test_two() {
        let input = vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2];
        assert_eq!(vec![1, 2], top_k_elements(input, 2))
    }

    #[test]
    fn test_three() {
        let input = vec![1];
        assert_eq!(vec![1], top_k_elements(input, 2))
    }
}
