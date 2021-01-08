use std::collections::HashMap;

#[allow(dead_code)]
pub fn all_construct<'a>(target: &'a str, word_bank: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut memo: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    helper(target, word_bank, &mut memo)
}

#[allow(dead_code)]
pub fn all_construct_2<'a>(target: &'a str, word_bank: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let n = target.len() + 1;
    let mut table: Vec<Vec<Vec<&str>>> = vec![vec![]; n + 1];
    table[0].push(vec![]);
    for i in 0..=n {
        if table[i].len() != 0 {
            for &word in word_bank {
                if target[i..].starts_with(word) {
                    let mut new_vec = vec![];
                    for mut item in table[i].clone() {
                        item.push(word);
                        new_vec.push(item);
                    }
                    table[i + word.len()].extend(new_vec.into_iter());
                }
            }
        }
    }
    table[n - 1].clone()
}

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
    result
}

#[cfg(test)]
mod all_construct_tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_construct_test() {
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
    #[test]
    fn all_construct_test_2() {
        let a = all_construct_2("abcdef", &vec!["ab", "abc", "cd", "def", "abcd", "ef"]);
        let b = vec![
            vec!["ab", "cd", "ef"],
            vec!["abc", "def"],
            vec!["abcd", "ef"],
        ];
        let hash_a: HashSet<Vec<&str>> = a.into_iter().collect();
        let hash_b: HashSet<Vec<&str>> = b.into_iter().collect();
        assert_eq!(hash_a, hash_b);
    }
}
