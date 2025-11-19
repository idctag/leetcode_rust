pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut v: Vec<i32> = nums.clone();
    v.sort();
    for i in 0..v.len() {
        let curr = v[i];
        if curr > 0 {
            break;
        }
        if i > 0 && curr == v[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, v.len() - 1);
        while l < r {
            let sum = v[i] + v[l] + v[r];
            if sum > 0 {
                r -= 1
            } else if sum < 0 {
                l += 1
            } else {
                res.push(vec![v[i], v[l], v[r]]);
                l += 1;
                r -= 1;
                while l < r && v[l] == v[l - 1] {
                    l += 1
                }
            }
        }
    }
    res
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }
    #[test]
    fn two() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }
    #[test]
    fn three() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::from([]))
    }
}
