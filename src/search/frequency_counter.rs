use std::collections::HashMap;
use std::fs::File;
// use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
// use std::{cell::RefCell};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn frequency_counter<P>(minlen: i32, filepath: P)
where
    P: AsRef<Path>,
{
    let mut st: HashMap<String, i32> = HashMap::new();
    let mut word_str = String::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(l) = line {
                // println!("{}", l);
                let line_words = l.split_whitespace();
                // println!("{:?}", line_words);
                for word in line_words {
                    // println!("{}", word);
                    word_str = word.to_string();
                    // println!("{:?}", word_str);
                    if word_str.len() < minlen as usize {
                        continue;
                    } else if st.get(&word_str) == None {
                        // println!("word {}", word);
                        st.insert(word_str, 1);
                    } else {
                        *st.get_mut(&word_str).unwrap() += 1;
                    }
                }
            }
        }
    }
    let mut max_str = String::from("");
    st.insert(max_str.clone(), 0);
    // println!("{:?}", st);
    for key in st.keys() {
        if st.get(key) > st.get(&max_str) {
            max_str = key.clone();
        }
    }
    println!("{:?}: {:?}", max_str, st.get(&max_str).unwrap());
}

#[test]
pub fn fq_test() {
    let path = Path::new("data/tale.txt");
    frequency_counter(0, &path);
}
