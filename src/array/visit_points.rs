pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut moves = 0;
    let len = &points.len();
    for (i, p) in points.clone().into_iter().enumerate() {
        if i == len - 1 {
            break;
        }
        let next = &points[i + 1];
        let (x_diff, y_diff) = (p[0].abs_diff(next[0]), p[1].abs_diff(next[1]));
        let both_moves = x_diff.min(y_diff);
        let one_moves = x_diff.max(y_diff);
        moves += both_moves + (one_moves - both_moves);
    }
    moves as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        assert_eq!(min_time_to_visit_all_points(points), 7)
    }
    #[test]
    fn test_two() {
        let points = vec![vec![3, 2], vec![-2, 2]];
        assert_eq!(min_time_to_visit_all_points(points), 5)
    }
}
