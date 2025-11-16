pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max: i32 = 0;
    for n in nums {
        if map.contains_key(&n) {
            continue;
        }
        let prev_n = *map.get(&(n - 1)).unwrap_or(&0);
        let next_n = *map.get(&(n + 1)).unwrap_or(&0);
        let sum = prev_n + next_n + 1;

        map.insert(n, sum);

        let first = n - prev_n;
        map.insert(first, sum);

        let last = n + next_n;
        map.insert(last, sum);
        max = max.max(sum);
    }
    max
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn ex_one() {
        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(input), 9)
    }
    #[test]
    fn ex_two() {
        let input = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(input), 4)
    }
    #[test]
    fn ex_three() {
        let input = vec![1, 0, 1, 2];
        assert_eq!(longest_consecutive(input), 3)
    }
}
