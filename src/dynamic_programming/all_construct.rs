#[allow(dead_code)]
pub fn helper<'a>(target: &str, word_bank: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    if target.is_empty() {
        return vec![vec![]];
    }

    // else {return vec![];}
    let mut result: Vec<Vec<&str>> = vec![];

    for &word in word_bank {
        if target.starts_with(word) {
            let suffix = target.strip_prefix(word).unwrap_or("");
            let suffix_ways = helper(suffix, word_bank);
            let mut target_ways = vec![];
            for mut x in suffix_ways {
                x.push(word);
                target_ways.push(x);
            }
            result.push(target_ways.into_iter().flatten().collect());
        }
    }
    return result;
}

#[cfg(test)]
mod best_sum_test {
    use super::*;

    #[test]
    fn acllstruct_test() {
        let a = helper("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        assert_eq!(
            a,
            vec![
                vec!["ef", "cd", "ab"],
                vec!["def", "abc"],
                vec!["ef", "abcd"]
            ]
        );
    }
}
