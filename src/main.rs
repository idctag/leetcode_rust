use leetcode::array::contains_duplicate;

fn main() {
    let nums = vec![1, 2, 3, 1];
    let result = contains_duplicate::contains_duplicate(nums);
    println!("Contains duplicate: {}", result);
}
