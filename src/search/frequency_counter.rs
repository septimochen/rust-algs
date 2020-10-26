use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use super::binary_search_st::BinarySearchST;
use super::bst::BST;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn frequency_counter<P>(minlen: i32, filepath: P) -> (String, i32)
where
    P: AsRef<Path>,
{
    let mut st: HashMap<String, i32> = HashMap::new();
    let mut word_str = String::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(line) = line {
                let line_words = line.split_whitespace();
                for word in line_words {
                    word_str = word.to_string();
                    if word_str.len() < minlen as usize {
                        continue;
                    // } else if st.get(&word_str) == None {
                    //     st.insert(word_str, 1);
                    // } else {
                    //     *st.get_mut(&word_str).unwrap() += 1;
                    // }
                    } else {
                        match st.get_mut(&word_str) {
                            None => {st.insert(word_str, 1);},
                            Some(x) => {*x += 1;},
                        }
                    }
                }
            }
        }
    }
    let mut max_str = String::from("");
    st.insert(max_str.clone(), 0);
    for key in st.keys() {
        if st.get(key) > st.get(&max_str) {
            max_str = key.clone();
        }
    }
    let max_num = st.get(&max_str.clone()).unwrap();
    (max_str, *max_num)
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn frequency_counter_2<P>(minlen: i32, filepath: P) -> (String, i32)
where
    P: AsRef<Path>,
{
    let mut st: BinarySearchST = BinarySearchST::new();
    let mut word_str = String::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(line) = line {
                let line_words = line.split_whitespace();
                for word in line_words {
                    word_str = word.to_string();
                    if word_str.len() < minlen as usize {
                        continue;
                    // } else if st.get(&word_str) == None {
                    //     st.insert(word_str, 1);
                    // } else {
                    //     *st.get_mut(&word_str).unwrap() += 1;
                    // }
                    } else {
                        // println!("{}", word);
                        match st.get(word.to_string()) {
                            None => {st.put(word_str, 1);},
                            Some(x) => {st.put(word_str.to_string(), x+1);},
                        }
                        // println!("{:?}", st);
                    }
                }
            }
        }
    }
    println!("{:?}", st);
    let mut max_str = String::from("");
    st.put(max_str.clone(), 0);
    for key in &st.keys {
        if st.get(key.to_string()).unwrap() > st.get(max_str.to_string()).unwrap() {
            max_str = key.clone();
        }
    }
    let max_num = st.get(max_str.clone()).unwrap();
    (max_str, max_num)
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn frequency_counter_bst<P>(minlen: i32, filepath: P) -> (String, i32)
where
    P: AsRef<Path>,
{
    let mut st: BST = BST::new();
    let mut word_str = String::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(line) = line {
                let line_words = line.split_whitespace();
                for word in line_words {
                    word_str = word.to_string();
                    if word_str.len() < minlen as usize {
                        continue;
                    // } else if st.get(&word_str) == None {
                    //     st.insert(word_str, 1);
                    // } else {
                    //     *st.get_mut(&word_str).unwrap() += 1;
                    // }
                    } else {
                        // println!("{}", word);
                        match st.get(word.to_string()) {
                            None => {st.put(word_str, 1);},
                            Some(x) => {st.put(word_str.to_string(), x+1);},
                        }
                        // println!("{:?}", st);
                    }
                }
            }
        }
    }
    // println!("{:?} with size {}", st.get("the".to_owned()), st.size());
    let mut max_str = String::from("");
    st.put(max_str.clone(), 0);
    for key in st.keys() {
        if st.get(key.to_string()).unwrap() > st.get(max_str.to_string()).unwrap() {
            max_str = key.clone();
        }
    }
    let max_num = st.get(max_str.clone()).unwrap();
    (max_str, max_num)
}

#[test]
pub fn fq_test_v2() {
    let path = Path::new("data/tale.txt");
    let result = frequency_counter_2(8, &path);
    println!("{:?}", result);
    assert_eq!(result, ("business".to_owned(), 122))
}

#[test]
pub fn fq_test_bst() {
    let path = Path::new("data/tale.txt");
    let result = frequency_counter_bst(8, &path);
    println!("{:?}", result);
    assert_eq!(result, ("business".to_owned(), 122))
}
