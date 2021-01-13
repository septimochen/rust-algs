use std::vec;

#[allow(dead_code)]
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut track: Vec<i32> = vec![];
    backtrack(&nums, &mut track, &mut res);
    res
}

fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if track.len() == nums.len() {
        res.push(track.clone());
        return;
    }

    for i in 0..nums.len() {
        if track.contains(&nums[i]) {
            continue;
        }
        track.push(nums[i]);
        backtrack(nums, track, res);
        track.pop();
    }
}

#[test]
fn permute_test() {
    let a = permute(vec![1, 2, 3]);
    println!("{:?}", a);
    let b = permute(vec![3, 4, 5, 6]);
    println!("{:?}", b);
}
