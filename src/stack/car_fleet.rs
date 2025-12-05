pub fn solution(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut order: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    let mut stack = Vec::new();
    order.sort_by_key(|&(pos, _speed)| -pos);
    for p in order {
        let time = (target - p.0) as f64 / p.1 as f64;
        stack.push(time);
        if stack.len() >= 2 && stack.last().unwrap() <= &stack[stack.len() - 2] {
            stack.pop();
        }
    }
    stack.len() as i32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        let speed = vec![2, 4, 1, 1, 3];
        let position = vec![10, 8, 0, 5, 3];
        let target = 12;
        assert_eq!(solution(target, position, speed), 3)
    }
    #[test]
    fn two() {
        let speed = vec![3];
        let position = vec![3];
        let target = 10;
        assert_eq!(solution(target, position, speed), 1)
    }
    #[test]
    fn three() {
        let speed = vec![0, 2, 4];
        let position = vec![4, 2, 1];
        let target = 100;
        assert_eq!(solution(target, position, speed), 1)
    }
    #[test]
    fn four() {
        let speed = vec![6, 8];
        let position = vec![3, 2];
        let target = 10;
        assert_eq!(solution(target, position, speed), 2)
    }
    #[test]
    fn five() {
        let speed = vec![4, 4, 4, 4, 4, 4];
        let position = vec![8, 3, 7, 4, 6, 5];
        let target = 10;
        assert_eq!(solution(target, position, speed), 6)
    }
}
