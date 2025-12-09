pub fn solution(n: i32, p: i32) -> i32 {
    let page_to_find = (p / 2) + 1;
    let total_pages = (n / 2) + 1;
    if total_pages == page_to_find && total_pages / 2 == page_to_find {
        return 0;
    }
    if page_to_find > total_pages / 2 {
        return total_pages - page_to_find;
    } else {
        return page_to_find / 2;
    }
}
// 1 23 45 6
// 1 2  3  4
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(solution(5, 3), 1)
    }
    #[test]
    fn two() {
        assert_eq!(solution(6, 2), 1)
    }
    #[test]
    fn three() {
        assert_eq!(solution(5, 4), 0)
    }
}
