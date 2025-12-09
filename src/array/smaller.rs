pub fn smaller(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut res: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut sorted: Vec<i32> = nums.clone();
    sorted.sort();
    for (i, v) in sorted.into_iter().enumerate() {
        map.entry(v).or_insert(i as i32);
    }
    for n in nums {
        res.push(map[&n]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let nums = vec![8, 1, 2, 2, 3];
        assert_eq!(smaller(nums), vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn two() {
        let nums = vec![6, 5, 4, 8];
        assert_eq!(smaller(nums), vec![2, 1, 0, 3]);
    }
    #[test]
    fn three() {
        let nums = vec![7, 7, 7, 7];
        assert_eq!(smaller(nums), vec![0, 0, 0, 0]);
    }
}
