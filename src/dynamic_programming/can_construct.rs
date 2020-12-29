#[allow(dead_code)]
pub fn can_construct(target: &str, word_bank: &Vec<&str>) -> bool {
    if target.is_empty() {
        return true;
    }

    for &word in word_bank {
        if target.starts_with(word) {
            let remainder = target.strip_prefix(word).unwrap_or("");
            if can_construct(remainder, word_bank) == true {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod best_sum_test {
    use super::*;

    #[test]
    fn construct_test() {
        let a = can_construct("abcdef", &vec!["ab", "abc", "cd", "def", "abcd"]);
        assert_eq!(a, true);
        let b = can_construct("abcdefg", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(b, false);
    }
}
