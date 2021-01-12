use std::vec;

#[allow(dead_code)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let res = vec![];
    let track: Vec<i32> = vec![];
    backtrack(nums, track, &res);
    res
}

fn backtrack(nums: Vec<i32>, track: Vec<i32>, res: &Vec<Vec<i32>>) {}
