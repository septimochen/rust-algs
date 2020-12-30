use std::collections::HashMap;

#[allow(dead_code)]
pub fn count_construct(target: &str, word_bank: &Vec<&str>) -> i32 {
    let mut memo: HashMap<&str, i32> = HashMap::new();
    helper(target, word_bank, &mut memo)
}

pub fn helper<'a>(
    target: &'a str,
    word_bank: &Vec<&str>,
    memo: &mut HashMap<&'a str, i32>,
) -> i32 {
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
            if helper(suffix, word_bank, memo) == 1 {
                count += 1;
            }
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
        let a = count_construct("abcdef", &vec!["ab", "abc", "cd", "def", "abcd"]);
        assert_eq!(a, 1);
        let b = count_construct("abcdefg", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(b, 0);
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
}
