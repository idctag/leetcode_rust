use leetcode::stack::car_fleet;

fn main() {
    let speed = vec![2, 4, 1, 1, 3];
    let position = vec![10, 8, 0, 5, 3];
    let target = 12;
    car_fleet::solution(target, position, speed);
}
