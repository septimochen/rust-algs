use std::collections::HashMap;

#[allow(dead_code)]
pub fn all_construct<'a>(target: &'a str, word_bank: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut memo: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    return helper(target, word_bank, &mut memo);
}

#[allow(dead_code)]
pub fn helper<'a>(
    target: &'a str,
    word_bank: &Vec<&'a str>,
    memo: &mut HashMap<&'a str, Vec<Vec<&'a str>>>,
) -> Vec<Vec<&'a str>> {
    if memo.contains_key(target) {
        return memo.get(target).unwrap().to_vec();
    }

    if target.is_empty() {
        return vec![vec![]];
    }

    let mut result: Vec<Vec<&str>> = vec![];

    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap_or("");
            let suffix_ways = helper(suffix, word_bank, memo);
            let mut target_ways = vec![];
            for mut x in suffix_ways {
                x.insert(0, word);
                target_ways.push(x);
            }
            result.push(target_ways.into_iter().flatten().collect());
        }
    }
    memo.insert(target, result.clone());
    return result;
}

#[cfg(test)]
mod best_sum_test {
    use super::*;

    #[test]
    fn acllstruct_test() {
        let a = all_construct("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(
            a,
            vec![
                vec!["ab", "cd", "ef"],
                vec!["abc", "def"],
                vec!["abcd", "ef"]
            ]
        );
    }
}