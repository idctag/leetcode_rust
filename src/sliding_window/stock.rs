pub fn solution(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut l_idx = 0;
    for i in 0..prices.len() {
        if i == 0 {
            continue;
        }
        if prices[l_idx] > prices[i] {
            l_idx = i;
            continue;
        }
        max_profit = max_profit.max(prices[i] - prices[l_idx]);
    }
    max_profit
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(solution(vec![7, 1, 5, 3, 6, 4]), 5)
    }
    #[test]
    fn two() {
        assert_eq!(solution(vec![7, 6, 4, 3, 1]), 0)
    }
}
