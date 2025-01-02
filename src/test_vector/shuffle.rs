/*
   https://leetcode.com/problems/shuffle-the-array/
*/

pub fn test() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    println!("shuffle :  {:?}", shuffle(&nums));
}

fn shuffle(nums: &[i32]) -> Vec<i32> {
    let n = nums.len() / 2;
    (0..n).flat_map(|i| [nums[i], nums[n + i]]).collect()
}

#[test]
fn tc() {
    let nums = vec![2, 5, 1, 3, 4, 7];
    let result = shuffle(&nums);
    let check = vec![2, 3, 5, 4, 1, 7];
    assert_eq!(result, check);
}
