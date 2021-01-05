use std::collections::HashMap;

#[allow(dead_code)]
pub fn count_construct(target: &str, word_bank: &Vec<&str>) -> i32 {
    let mut memo: HashMap<&str, i32> = HashMap::new();
    helper(target, word_bank, &mut memo)
}

#[allow(dead_code)]
pub fn count_construct_2(target: &str, word_bank: &Vec<&str>) -> i64 {
    let n = target.len() + 1;
    let mut table: Vec<i64> = vec![0; n + 1];
    table[0] = 1;
    for i in 0..=n {
        if table[i] > 0 {
            for &word in word_bank {
                if target[i..].starts_with(word) {
                    table[i + word.len()] += table[i].clone();
                }
            }
        }
    }
    table[target.len()]
}

pub fn helper<'a>(target: &'a str, word_bank: &Vec<&str>, memo: &mut HashMap<&'a str, i32>) -> i32 {
    if memo.contains_key(target) {
        return memo[target];
    }
    if target.is_empty() {
        return 1;
    }

    let mut count = 0;

    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap_or("");
            let rest_cnt = helper(suffix, word_bank, memo);
            count += rest_cnt;
        }
    }
    memo.insert(target, count);
    return count;
}

#[cfg(test)]
mod best_sum_test {
    use super::*;

    #[test]
    fn construct_test() {
        let a = count_construct("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(a, 3);
        let b = count_construct(
            "abcdefg",
            &vec!["ab", "abc", "cd", "def", "abcd", "ef", "g"],
        );
        assert_eq!(b, 3);
        let c = count_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec![
                "e",
                "eee",
                "eeee",
                "eeeee",
                "eeeeee",
                "eeeeeeeee",
                "eeeeeeeeeeeee",
                "eeeeeeeeeeeeee",
                "eeeeeeeeeeeeeee",
                "eeeeeeeeeeeeeeee",
            ],
        );
        assert_eq!(c, 0);
    }
    #[test]
    fn construct_test_2() {
        let a = count_construct_2("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(a, 3);
        let b = count_construct_2(
            "abcdefg",
            &vec!["ab", "abc", "cd", "def", "abcd", "ef", "g"],
        );
        assert_eq!(b, 3);
        let c = count_construct_2(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            &vec![
                "e",
                "eee",
                "eeee",
                "eeeee",
                "eeeeee",
                "eeeeeeeee",
                "eeeeeeeeeeeee",
                "eeeeeeeeeeeeee",
                "eeeeeeeeeeeeeee",
                "eeeeeeeeeeeeeeee",
            ],
        );
        assert_eq!(c, 0);
    }
}
