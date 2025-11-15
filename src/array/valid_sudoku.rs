pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut boxes: HashMap<usize, HashSet<char>> = HashMap::new();
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            let curr_val = board[r][c];
            if curr_val == '.' {
                continue;
            }
            let curr_box = (r / 3) * 3 + (c / 3);
            if rows.entry(r).or_default().contains(&curr_val) {
                return false;
            }
            if cols.entry(c).or_default().contains(&curr_val) {
                return false;
            }
            if boxes.entry(curr_box).or_default().contains(&curr_val) {
                return false;
            }
            rows.get_mut(&r).unwrap().insert(curr_val);
            cols.get_mut(&c).unwrap().insert(curr_val);
            boxes.get_mut(&curr_box).unwrap().insert(curr_val);
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ex_one() {
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(input), true)
    }
    #[test]
    fn ex_two() {
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(input), false)
    }
}
