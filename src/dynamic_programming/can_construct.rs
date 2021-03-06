use std::collections::HashMap;

#[allow(dead_code)]
pub fn can_construct(target: &str, word_bank: &Vec<&str>) -> bool {
    let mut memo: HashMap<&str, bool> = HashMap::new();
    helper(target, word_bank, &mut memo)
}

#[allow(dead_code)]
pub fn can_construct_2(target: &str, word_bank: &Vec<&str>) -> bool {
    let n = target.len() + 1;
    let mut table = vec![false; n + 1];
    table[0] = true;
    for i in 0..=n {
        if table[i] == true {
            for &word in word_bank {
                if target[i..].starts_with(word) {
                    table[i + word.len()] = true;
                }
            }
        }
    }
    table[target.len()]
}

pub fn helper<'a>(
    target: &'a str,
    word_bank: &Vec<&str>,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if memo.contains_key(target) {
        return memo[target];
    }
    if target.is_empty() {
        return true;
    }

    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap_or("");
            if helper(suffix, word_bank, memo) == true {
                memo.insert(target, true);
                return true;
            }
        }
    }
    memo.insert(target, false);
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
        let c = can_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec![
                "e",
                "eee",
                "eeee",
                "eeeee",
                "eeeeee",
                "eeeeeee",
                "eeeeeeeee",
                "eeeeeeeeeeeee",
                "eeeeeeeeeeeeee",
                "eeeeeeeeeeeeeee",
                "eeeeeeeeeeeeeeee",
            ],
        );
        assert_eq!(c, false);
    }
    #[test]
    fn construct_test_2() {
        let a = can_construct_2("abcdef", &vec!["ab", "abc", "cd", "def", "abcd"]);
        assert_eq!(a, true);
        let b = can_construct_2("abcdefg", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(b, false);
        let c = can_construct_2(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec![
                "e",
                "eee",
                "eeee",
                "eeeee",
                "eeeeee",
                "eeeeeee",
                "eeeeeeeee",
                "eeeeeeeeeeeee",
                "eeeeeeeeeeeeee",
                "eeeeeeeeeeeeeee",
                "eeeeeeeeeeeeeeee",
            ],
        );
        assert_eq!(c, false);
    }
}
